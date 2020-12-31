run:
	-docker stop $$(docker ps -aq)
	cd card_recognizer_ml; make run
	cd ocr; make run
	python3 orchestrator/main.py