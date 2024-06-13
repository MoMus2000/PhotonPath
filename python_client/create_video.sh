#!/bin/bash

echo "Creating video"
python3 video.py

echo "Deleting images"

cd /Users/mmuhammad/Desktop/projects/ray-tracer/python_client/images
rm *

echo "Done"
