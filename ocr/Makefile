
test:
	touch data/trigger
	docker run -it -v $$(pwd)/data:/data -v $$(pwd):/src --rm paveltumik/tesseract:latest python3 /src/ocr.py

run:
	docker run --name ocr -t -d -v $$(pwd)/data:/data -v $$(pwd):/src --rm paveltumik/tesseract:latest bash

image:
	docker build -t paveltumik/tesseract:latest .
