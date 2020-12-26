from PIL import Image, ImageGrab 
import time
import os
import subprocess

#hard coded for now
# 7 bboxes of relevant cards
hand_w = 48
hand_h = 70
card_w = 70
card_h = 100

hand1 = (1977,579)
hand2 = (2028,579)
flop1 = (1836,426)
flop2 = (1916,426)
flop3 = (1992,426)
turn1 = (2078,426)
rive1 = (2154,426)

docker_inst = "1de9063ce839"

while (True):
  im2 = ImageGrab.grab(bbox =(hand1[0], hand1[1], hand1[0]+hand_w, hand1[1]+hand_h)) 
  im2.save("card_recognizer_ml/data/test/1.png")

  im2 = ImageGrab.grab(bbox =(hand2[0], hand2[1], hand2[0]+hand_w, hand2[1]+hand_h)) 
  im2.save("card_recognizer_ml/data/test/2.png")

  im2 = ImageGrab.grab(bbox =(flop1[0], flop1[1], flop1[0]+card_w, flop1[1]+card_h)) 
  im2.save("card_recognizer_ml/data/test/3.png")

  im2 = ImageGrab.grab(bbox =(flop2[0], flop2[1], flop2[0]+card_w, flop2[1]+card_h)) 
  im2.save("card_recognizer_ml/data/test/4.png")

  im2 = ImageGrab.grab(bbox =(flop3[0], flop3[1], flop3[0]+card_w, flop3[1]+card_h)) 
  im2.save("card_recognizer_ml/data/test/5.png")

  im2 = ImageGrab.grab(bbox =(turn1[0], turn1[1], turn1[0]+card_w, turn1[1]+card_h)) 
  im2.save("card_recognizer_ml/data/test/6.png")

  im2 = ImageGrab.grab(bbox =(rive1[0], rive1[1], rive1[0]+card_w, rive1[1]+card_h)) 
  im2.save("card_recognizer_ml/data/test/7.png")

  stream = os.popen('docker exec '+docker_inst+' python /src/main.py')
  output = stream.read()
  output = output.rstrip('\n')
  
  print("ML output: ", output)

  stream = os.popen('expected_value/target/release/poker_ev "'+output+'"')
  output2 = stream.read()
  print("EV: ", output2)

  #print("Sleeping before next run")
  #time.sleep(1) #sleep 1 second