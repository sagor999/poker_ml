from PIL import Image, ImageGrab, ImageChops, ImageOps
import time
import os
import subprocess

#hard coded for now
# 7 bboxes of relevant cards
hand_w = 48
hand_h = 70
card_w = 70
card_h = 100
pot_w = 160
pot_h = 74
action_w = 150
action_h = 55

hand1 = (1964,597)
hand2 = (2016,597)
flop1 = (1812,434)
flop2 = (1896,434)
flop3 = (1980,434)
turn1 = (2063,434)
rive1 = (2144,434)
pot   = (1935,356)
action= (1980,738)

scan_areas = [(hand1[0], hand1[1], hand1[0]+hand_w, hand1[1]+hand_h), 
              (hand2[0], hand2[1], hand2[0]+hand_w, hand2[1]+hand_h), 
              (flop1[0], flop1[1], flop1[0]+card_w, flop1[1]+card_h), 
              (flop2[0], flop2[1], flop2[0]+card_w, flop2[1]+card_h), 
              (flop3[0], flop3[1], flop3[0]+card_w, flop3[1]+card_h), 
              (turn1[0], turn1[1], turn1[0]+card_w, turn1[1]+card_h), 
              (rive1[0], rive1[1], rive1[0]+card_w, rive1[1]+card_h),
              (pot[0], pot[1], pot[0]+pot_w, pot[1]+pot_h),
              (action[0], action[1], action[0]+action_w, action[1]+action_h),]
prev_scans = []

for i in range(len(scan_areas)):
  prev_scans.append(ImageGrab.grab(bbox=scan_areas[i]))

card_ml_stream = os.popen('docker exec card_ml python /src/main.py')
ocr_stream = os.popen('docker exec ocr python3 /src/ocr.py')
ev_stream = os.popen('expected_value/target/release/poker_ev loop')

prev_ml_output = ""
prev_ocr_output = ""
prev_ocr_output2 = ""
while (True):
  changed = False

  # check if anything has changed
  for i in range(len(scan_areas)):
    im2 = ImageGrab.grab(bbox=scan_areas[i])
    if ImageChops.difference(prev_scans[i], im2).getbbox() is not None:
      changed = True
      break

  if changed:
    #print("Detected change")
    time.sleep(1.0) # wait for animations to finish
    for i in range(len(scan_areas)):
      im2 = ImageGrab.grab(bbox=scan_areas[i])
      if i==7:
        im2 = ImageOps.invert(im2) # ocr prefers light background dark text
        im2.save("ocr/data/ocr.png")
      elif i==8:
        im2 = ImageOps.invert(im2) # ocr prefers light background dark text
        im2.save("ocr/data/ocr2.png")
      else:
        im2.save("card_recognizer_ml/data/test/{}.png".format(i+1))
      prev_scans[i] = im2

    os.system("touch card_recognizer_ml/data/trigger")
    #print("Touched trigger. Waiting for ML")
    card_ml_output = card_ml_stream.readline()
    card_ml_output = card_ml_output.rstrip('\n')
    #print("ML output: ", card_ml_output)

    os.system("touch ocr/data/trigger")
    ocr_output = ""
    while True:
      line = ocr_stream.readline()
      if line.strip() == "END" or line == "":
        break
      ocr_output += line

    os.system("touch ocr/data/trigger2")
    ocr_output2 = ""
    while True:
      line = ocr_stream.readline()
      if line.strip() == "END" or line == "":
        break
      ocr_output2 += line

    #print("OCR: ", ocr_output)
    #print("end ocr")

    if prev_ml_output != card_ml_output or prev_ocr_output != ocr_output or prev_ocr_output2 != ocr_output2:
      prev_ml_output = card_ml_output
      prev_ocr_output = ocr_output
      prev_ocr_output2 = ocr_output2

      if len(ocr_output.strip())>0 and "Total pot" not in ocr_output:
        print("Malformed ocr: ", ocr_output)
        epoch_time = int(time.time())
        os.system("mkdir ocr/data/malformed/{}".format(epoch_time))
        os.system("cp ocr/data/ocr.png ocr/data/malformed/{}".format(epoch_time))


      with open("expected_value/data/input_hand", "w") as input_hand_file:
        print("{}".format(card_ml_output), file=input_hand_file)
      with open("expected_value/data/input_pot", "w") as input_pot_file:
        print("{}".format(ocr_output), file=input_pot_file)
      with open("expected_value/data/input_action", "w") as input_action_file:
        print("{}".format(ocr_output2), file=input_action_file)
      print("----------------------------------")
      os.system("touch expected_value/data/trigger")
      while True:
        line = ev_stream.readline()
        if line.strip() == "END" or line == "":
          break
        print(line.strip())
        if 'Detected duplicate card' in line:
          epoch_time = int(time.time())
          os.system("mkdir card_recognizer_ml/data/add_to_train/{}".format(epoch_time))
          os.system("cp card_recognizer_ml/data/test/* card_recognizer_ml/data/add_to_train/{}".format(epoch_time))
  else:
    #print("Sleeping before next run")
    time.sleep(0.1) 

