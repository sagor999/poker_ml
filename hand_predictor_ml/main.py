import os, select
import sys
import pathlib
import PIL
import time

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '1'

import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers
from tensorflow.keras.models import Sequential
from tensorflow import feature_column

import numpy as np

checkpoint_path = "/data/model.tf"
checkpoint_dir = os.path.dirname(checkpoint_path)

batch_size = 4096*4
shuffle_size = batch_size*10

AUTOTUNE = tf.data.experimental.AUTOTUNE

Training = True

def get_dataset(path_dataset, shuffle):
    dataset = tf.data.experimental.make_csv_dataset(
      path_dataset,
      batch_size=batch_size, 
      label_name='did_win',
      num_epochs=1,
      shuffle=shuffle,
      shuffle_buffer_size=shuffle_size,
      ignore_errors=False,)
    #for batch, label in dataset.take(1):
    #  for key, value in batch.items():
    #    print(f"{key:20s}: {value}")
    #  print()
    #  print(f"{'label':20s}: {label}")
    
    return dataset

if Training:
  csv_train_ds = get_dataset('/data/ml_data.csv', True)

  feature_columns = []
  feature_layer_inputs = {}
  game_states = ['PREFLOP', 'FLOP', 'TURN', 'RIVER']
  game_state_categorical_column = feature_column.categorical_column_with_vocabulary_list(
      'state', game_states)
  game_state_column = feature_column.indicator_column(game_state_categorical_column)
  feature_columns.append(game_state_column)
  feature_layer_inputs['state'] = tf.keras.Input(shape=(1,), name='state', dtype=tf.string)

  card_names = ['2c', '3c', '4c', '5c', '6c', '7c', '8c', '9c', 'Tc', 'Jc', 'Qc', 'Kc', 'Ac', 
                '2h', '3h', '4h', '5h', '6h', '7h', '8h', '9h', 'Th', 'Jh', 'Qh', 'Kh', 'Ah', 
                '2s', '3s', '4s', '5s', '6s', '7s', '8s', '9s', 'Ts', 'Js', 'Qs', 'Ks', 'As', 
                '2d', '3d', '4d', '5d', '6d', '7d', '8d', '9d', 'Td', 'Jd', 'Qd', 'Kd', 'Ad',
                'none']
  card_column_names = ['hand1','hand2','flop1','flop2','flop3','turn','river']

  for col_name in card_column_names:
    categorical_column = feature_column.categorical_column_with_vocabulary_list(
        col_name, card_names)
    indicator_column = feature_column.indicator_column(categorical_column)
    feature_columns.append(indicator_column)
    feature_layer_inputs[col_name] = tf.keras.Input(shape=(1,), name=col_name, dtype=tf.string)
  
  float_columns = ['win_chance','won_on_flop','won_on_turn','won_on_river','hand_equity']
  for header in float_columns:
    feature_columns.append(feature_column.numeric_column(header))
    feature_layer_inputs[header] = tf.keras.Input(shape=(1,), name=header)

  #print(feature_columns)

  feature_layer = tf.keras.layers.DenseFeatures(feature_columns)
 
  csv_val_ds = get_dataset('/data/ml_val.csv', False)

  cp_callback = tf.keras.callbacks.ModelCheckpoint(filepath=checkpoint_path,
                                                  save_weights_only=False,
                                                  save_best_only=True)

  feature_layer = tf.keras.layers.DenseFeatures(feature_columns)
  feature_layer_outputs = feature_layer(feature_layer_inputs)

  x = layers.Dense(4096, activation='relu')(feature_layer_outputs)
  x = layers.Dense(4096, activation='relu')(x)
  x = layers.Dropout(0.5)(x)
  x = layers.Dense(2048, activation='relu')(x)
  x = layers.Dense(2048, activation='relu')(x)
  x = layers.Dropout(0.5)(x)
  x = layers.Dense(1024, activation='relu')(x)
  x = layers.Dense(1024, activation='relu')(x)
  x = layers.Dense(1024, activation='relu')(x)
  x = layers.Dropout(0.5)(x)
  x = layers.Dense(512, activation='relu')(x)
  x = layers.Dense(512, activation='relu')(x)
  x = layers.Dense(512, activation='relu')(x)
  x = layers.Dropout(0.5)(x)
  x = layers.Dense(64, activation='relu')(x)
  x = layers.Dense(16, activation='relu')(x)

  win_pred = layers.Dense(1)(x)

  model = keras.Model(inputs=[v for v in feature_layer_inputs.values()], outputs=win_pred)

  model.compile(optimizer=tf.keras.optimizers.Adam(),
                loss=tf.keras.losses.BinaryCrossentropy(from_logits=True),
                metrics=['accuracy'])
  #model.summary()


  # train
  epochs=20
  history = model.fit(
    csv_train_ds,
    validation_data=csv_val_ds,
    epochs=epochs,
    callbacks=[cp_callback]
  )

  # show results
  acc = history.history['accuracy']
  val_acc = history.history['val_accuracy']

  loss = history.history['loss']
  val_loss = history.history['val_loss']

  epochs_range = range(epochs)

  import matplotlib.pyplot as plt
  plt.figure(figsize=(8, 8))
  plt.subplot(1, 2, 1)
  plt.plot(epochs_range, acc, label='Training Accuracy')
  plt.plot(epochs_range, val_acc, label='Validation Accuracy')
  plt.legend(loc='lower right')
  plt.title('Training and Validation Accuracy')

  plt.subplot(1, 2, 2)
  plt.plot(epochs_range, loss, label='Training Loss')
  plt.plot(epochs_range, val_loss, label='Validation Loss')
  plt.legend(loc='upper right')
  plt.title('Training and Validation Loss')
  plt.show()

else:
  model = tf.keras.models.load_model(checkpoint_path)
  predict_ds = get_dataset('/data/predict.csv', False)

  predictions = model.predict(predict_ds)
  print(predictions)
  sys.stdout.flush()
