
test:
	touch data/trigger
	docker run -it -v /home/pavel/nvme/GitHub/poker_ml/ocr/data:/data -v /home/pavel/nvme/GitHub/poker_ml/ocr:/src --rm paveltumik/tesseract:latest python3 /src/ocr.py

run:
	docker run --name ocr -t -d -v /home/pavel/nvme/GitHub/poker_ml/ocr/data:/data -v /home/pavel/nvme/GitHub/poker_ml/ocr:/src --rm paveltumik/tesseract:latest bash

image:
	docker build -t paveltumik/tesseract:latest .
