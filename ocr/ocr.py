from PIL import Image
import pytesseract

import sys
import pathlib
import time
from unidecode import unidecode

# psm: 
#  0    Orientation and script detection (OSD) only.
#  1    Automatic page segmentation with OSD.
#  2    Automatic page segmentation, but no OSD, or OCR. (not implemented)
#  3    Fully automatic page segmentation, but no OSD. (Default)
#  4    Assume a single column of text of variable sizes.
#  5    Assume a single uniform block of vertically aligned text.
#  6    Assume a single uniform block of text.
#  7    Treat the image as a single text line.
#  8    Treat the image as a single word.
#  9    Treat the image as a single word in a circle.
# 10    Treat the image as a single character.
# 11    Sparse text. Find as much text as possible in no particular order.
# 12    Sparse text with OSD.
# 13    Raw line. Treat the image as a single text line,
#       bypassing hacks that are Tesseract-specific.
# oem:
#  0    Legacy engine only.
#  1    Neural nets LSTM engine only.
#  2    Legacy + LSTM engines.
#  3    Default, based on what is available.
custom_oem_psm_config = r'--oem 3 --psm 4'

#print(unidecode(pytesseract.image_to_string(Image.open('/data/ocr6.png'), timeout=1, lang='eng', config=custom_oem_psm_config)))

while True:
  trigger_file = pathlib.Path("/data/trigger")
  if trigger_file.is_file():
    trigger_file.unlink()
    output = unidecode(pytesseract.image_to_string(Image.open('/data/ocr.png'), timeout=1, lang='eng', config=custom_oem_psm_config))
    [print(line) for line in output.split('\n') if line.strip() != '']
    print('END')
    sys.stdout.flush()
  trigger_file = pathlib.Path("/data/trigger2")
  if trigger_file.is_file():
    trigger_file.unlink()
    output = unidecode(pytesseract.image_to_string(Image.open('/data/ocr2.png'), timeout=1, lang='eng', config=custom_oem_psm_config))
    [print(line) for line in output.split('\n') if line.strip() != '']
    print('END')
    sys.stdout.flush()

  time.sleep(0.1)