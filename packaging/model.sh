#!/usr/bin/env bash
set -e

# Export YOLOv8n model with embedded NMS

# Fetch YOLOv8 nano detection model
wget https://github.com/ultralytics/assets/releases/download/v8.4.0/yolov8n.pt

# Build Python environment to contain Ultralytics
python3 -m venv "$PWD/model-env"
source "$PWD/model-env/bin/activate"

# Tested using ultralytics-8.4.104
pip install ultralytics

# Export the YOLO model to ONNX with embedded NMS
yolo export model=yolov8n.pt format=onnx nms=True
