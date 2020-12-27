from PIL import Image, ImageGrab 
from PIL import ImageChops
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

hand1 = (1962,599)
hand2 = (2015,599)
flop1 = (1815,437)
flop2 = (1898,437)
flop3 = (1977,437)
turn1 = (2064,437)
rive1 = (2150,437)
pot   = (1935,356)

scan_areas = [(hand1[0], hand1[1], hand1[0]+hand_w, hand1[1]+hand_h), 
              (hand2[0], hand2[1], hand2[0]+hand_w, hand2[1]+hand_h), 
              (flop1[0], flop1[1], flop1[0]+card_w, flop1[1]+card_h), 
              (flop2[0], flop2[1], flop2[0]+card_w, flop2[1]+card_h), 
              (flop3[0], flop3[1], flop3[0]+card_w, flop3[1]+card_h), 
              (turn1[0], turn1[1], turn1[0]+card_w, turn1[1]+card_h), 
              (rive1[0], rive1[1], rive1[0]+card_w, rive1[1]+card_h),
              (pot[0], pot[1], pot[0]+pot_w, pot[1]+pot_h)]
prev_scans = []

for i in range(len(scan_areas)):
  prev_scans.append(ImageGrab.grab(bbox=scan_areas[i]))

card_ml_stream = os.popen('docker exec card_ml python /src/main.py')
ocr_stream = os.popen('docker exec ocr python3 /src/ocr.py')

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
        im2.save("ocr/data/ocr.png")
      else:
        im2.save("card_recognizer_ml/data/test/{}.png".format(i+1))
      prev_scans[i] = im2

    os.system("touch card_recognizer_ml/data/trigger")
    #print("Touched trigger. Waiting for ML")
    card_ml_output = card_ml_stream.readline()
    card_ml_output = card_ml_output.rstrip('\n')
    #print("ML output: ", card_ml_output)

    os.system("touch ocr/data/trigger")
    #print("Touched trigger. Waiting for OCR")
    ocr_output = ""
    while True:
      line = ocr_stream.readline()
      if line.strip() == "END" or line == "":
        break
      ocr_output += line
    
    #print("OCR: ", ocr_output)
    #print("end ocr")

    poker_ev_stream = os.popen("expected_value/target/release/poker_ev '{}' '{}'".format(card_ml_output, ocr_output))
    poker_ev_output = poker_ev_stream.read()
    print(poker_ev_output)
  else:
    #print("Sleeping before next run")
    time.sleep(0.1) 

