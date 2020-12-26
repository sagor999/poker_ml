import os
import pathlib
import PIL

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '1'

import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers
from tensorflow.keras.models import Sequential

import numpy as np

checkpoint_path = "/data/cp.ckpt"
checkpoint_dir = os.path.dirname(checkpoint_path)

num_classes = 53
class_names = ['2c', '2d', '2h', '2s', '3c', '3d', '3h', '3s', '4c', '4d', '4h', '4s', '5c', '5d', '5h', '5s', '6c', '6d', '6h', '6s', '7c', '7d', '7h', '7s', '8c', '8d', '8h', '8s', '9c', '9d', '9h', '9s', 'Ac', 'Ad', 'Ah', 'As', 'Empty', 'Jc', 'Jd', 'Jh', 'Js', 'Kc', 'Kd', 'Kh', 'Ks', 'Qc', 'Qd', 'Qh', 'Qs', 'Tc', 'Td', 'Th', 'Ts']
AUTOTUNE = tf.data.experimental.AUTOTUNE
batch_size = 32
img_height = 70
img_width = 48

Training = False

model = Sequential([
  layers.experimental.preprocessing.Rescaling(1./255, input_shape=(img_height, img_width, 3)),
  layers.Conv2D(64, 3, padding='same', activation='relu'),
  layers.MaxPooling2D(),
  layers.Conv2D(64, 3, padding='same', activation='relu'),
  layers.MaxPooling2D(),
  layers.Conv2D(128, 3, padding='same', activation='relu'),
  layers.MaxPooling2D(),
  layers.Conv2D(128, 3, padding='same', activation='relu'),
  layers.MaxPooling2D(),
  layers.Flatten(),
  layers.Dropout(0.5),
  layers.Dense(512, activation='relu'),
  layers.Dense(num_classes, activation='softmax')
])

model.compile(optimizer=tf.keras.optimizers.Adam(),
              loss=tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True),
              metrics=['accuracy'])
#model.summary()

if Training:
  train_dir = pathlib.Path('/data/train')
  validation_dir = pathlib.Path('/data/validate')
  #image_count = len(list(train_dir.glob('*/*.png')))
  #print(image_count)

  #roses = list(train_dir.glob('0/*'))
  #PIL.Image.open(str(roses[0]))

  data_augmentation = keras.Sequential(
    [
      layers.experimental.preprocessing.RandomTranslation(0.1, 0.1),
      layers.experimental.preprocessing.RandomContrast(0.5),
      layers.experimental.preprocessing.RandomRotation(0.1),
      layers.experimental.preprocessing.RandomZoom(0.1),
    ]
  )

  def prepare(ds, shuffle=False, augment=False):
    if shuffle:
      ds = ds.shuffle(1000)

    # Use data augmentation only on the training set
    if augment:
      ds = ds.map(lambda x, y: (data_augmentation(x, training=True), y), 
                  num_parallel_calls=AUTOTUNE)

    # Use buffered prefecting on all datasets
    return ds.prefetch(buffer_size=AUTOTUNE)

  train_ds = tf.keras.preprocessing.image_dataset_from_directory(
    train_dir,
    seed=123,
    image_size=(img_height, img_width),
    batch_size=batch_size)

  print(train_ds.class_names)
  assert class_names == train_ds.class_names

  #plt.figure(figsize=(10, 10))
  #for images, labels in train_ds.take(1):
  #  for i in range(9):
  #    ax = plt.subplot(3, 3, i + 1)
  #    plt.imshow(images[i].numpy().astype("uint8"))
  #    plt.title(class_names[labels[i]])
  #    plt.axis("off")

  val_ds = tf.keras.preprocessing.image_dataset_from_directory(
    validation_dir,
    seed=123,
    image_size=(img_height, img_width),
    batch_size=batch_size)

  train_ds = prepare(train_ds, shuffle=True, augment=True)
  val_ds = prepare(val_ds)

  cp_callback = tf.keras.callbacks.ModelCheckpoint(filepath=checkpoint_path,
                                                  save_weights_only=True,
                                                  save_best_only=True)

  # train
  epochs=1000
  history = model.fit(
    train_ds,
    validation_data=val_ds,
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
  model.load_weights(checkpoint_path)

  img = keras.preprocessing.image.load_img(
      '/data/test/1.png', target_size=(img_height, img_width)
  )
  img_array = keras.preprocessing.image.img_to_array(img)
  img_array = tf.expand_dims(img_array, 0) # Create a batch
  
  predictions = model.predict(img_array)
  score1 = tf.nn.softmax(predictions[0])

  img = keras.preprocessing.image.load_img(
      '/data/test/2.png', target_size=(img_height, img_width)
  )
  img_array = keras.preprocessing.image.img_to_array(img)
  img_array = tf.expand_dims(img_array, 0) # Create a batch
  
  predictions = model.predict(img_array)
  score2 = tf.nn.softmax(predictions[0])

  img = keras.preprocessing.image.load_img(
      '/data/test/3.png', target_size=(img_height, img_width)
  )
  img_array = keras.preprocessing.image.img_to_array(img)
  img_array = tf.expand_dims(img_array, 0) # Create a batch
  
  predictions = model.predict(img_array)
  score3 = tf.nn.softmax(predictions[0])

  img = keras.preprocessing.image.load_img(
      '/data/test/4.png', target_size=(img_height, img_width)
  )
  img_array = keras.preprocessing.image.img_to_array(img)
  img_array = tf.expand_dims(img_array, 0) # Create a batch
  
  predictions = model.predict(img_array)
  score4 = tf.nn.softmax(predictions[0])

  img = keras.preprocessing.image.load_img(
      '/data/test/5.png', target_size=(img_height, img_width)
  )
  img_array = keras.preprocessing.image.img_to_array(img)
  img_array = tf.expand_dims(img_array, 0) # Create a batch
  
  predictions = model.predict(img_array)
  score5 = tf.nn.softmax(predictions[0])

  img = keras.preprocessing.image.load_img(
      '/data/test/6.png', target_size=(img_height, img_width)
  )
  img_array = keras.preprocessing.image.img_to_array(img)
  img_array = tf.expand_dims(img_array, 0) # Create a batch
  
  predictions = model.predict(img_array)
  score6 = tf.nn.softmax(predictions[0])

  img = keras.preprocessing.image.load_img(
      '/data/test/7.png', target_size=(img_height, img_width)
  )
  img_array = keras.preprocessing.image.img_to_array(img)
  img_array = tf.expand_dims(img_array, 0) # Create a batch
  
  predictions = model.predict(img_array)
  score7 = tf.nn.softmax(predictions[0])

  print("{} {} {} {} {} {} {}".format(class_names[np.argmax(score1)],class_names[np.argmax(score2)],class_names[np.argmax(score3)],class_names[np.argmax(score4)],class_names[np.argmax(score5)],class_names[np.argmax(score6)],class_names[np.argmax(score7)]))
