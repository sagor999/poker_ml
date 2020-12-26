
training: 
	docker run --runtime=nvidia -it -v /home/pavel/nvme/GitHub/poker_ml/card_recognizer_ml/data:/data -p 8888:8888 --rm tensorflow/tensorflow:latest-gpu-jupyter 

test:
	docker run --runtime=nvidia -it -v /home/pavel/nvme/GitHub/poker_ml/card_recognizer_ml/data:/data -v /home/pavel/nvme/GitHub/poker_ml/card_recognizer_ml:/src --rm paveltumik/tf-gpu:latest python /src/main.py

run:
	docker run --name card_ml --runtime=nvidia -t -d -v /home/pavel/nvme/GitHub/poker_ml/card_recognizer_ml/data:/data -v /home/pavel/nvme/GitHub/poker_ml/card_recognizer_ml:/src --rm paveltumik/tf-gpu:latest bash

image:
	docker build -t paveltumik/tf-gpu:latest .

#exec:
#