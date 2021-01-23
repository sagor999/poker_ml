
training: 
	docker run --runtime=nvidia -it -v $$(pwd)/data:/data -p 8888:8888 --rm tensorflow/tensorflow:latest-gpu-jupyter 

test:
	docker run --runtime=nvidia -it -v $$(pwd)/data:/data -v $$(pwd):/src --rm paveltumik/tf-gpu:latest python /src/main.py

run:
	docker run --name hand_ml --runtime=nvidia -t -d -v $$(pwd)/data:/data -v $$(pwd):/src --rm paveltumik/tf-gpu:latest bash
