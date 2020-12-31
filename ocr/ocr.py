from PIL import Image
import pytesseract

import sys
import pathlib
import time
from unidecode import unidecode

#print(pytesseract.image_to_string(Image.open('/data/ocr6.png')))

while True:
  trigger_file = pathlib.Path("/data/trigger")
  if trigger_file.is_file():
    trigger_file.unlink()
    output = unidecode(pytesseract.image_to_string(Image.open('/data/ocr.png'), timeout=1, lang='eng'))
    [print(line) for line in output.split('\n') if line.strip() != '']
    print('END')
    sys.stdout.flush()
  trigger_file = pathlib.Path("/data/trigger2")
  if trigger_file.is_file():
    trigger_file.unlink()
    output = unidecode(pytesseract.image_to_string(Image.open('/data/ocr2.png'), timeout=1, lang='eng'))
    [print(line) for line in output.split('\n') if line.strip() != '']
    print('END')
    sys.stdout.flush()

  time.sleep(0.1)