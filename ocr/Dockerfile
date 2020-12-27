FROM ubuntu:18.04

RUN apt-get update
RUN apt-get install software-properties-common -y


RUN add-apt-repository ppa:alex-p/tesseract-ocr-devel
RUN add-apt-repository ppa:deadsnakes/ppa
RUN apt-get update

RUN apt-get install python3.8 -y
RUN apt-get install tesseract-ocr -y
RUN apt-get install python3-pil -y
RUN apt-get install python3-pip -y

RUN pip3 install Pillow
RUN pip3 install pytesseract