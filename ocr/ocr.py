from PIL import Image
import pytesseract

import sys
import pathlib
import time


#print(pytesseract.image_to_string(Image.open('/data/ocr6.png')))

while True:
  trigger_file = pathlib.Path("/data/trigger")
  if trigger_file.is_file():
    trigger_file.unlink()
    output = pytesseract.image_to_string(Image.open('/data/ocr.png'), timeout=3, lang='eng')
    [print(line) for line in output.split('\n') if line.strip() != '']
    print('END')
    sys.stdout.flush()
  else:
    time.sleep(0.1)