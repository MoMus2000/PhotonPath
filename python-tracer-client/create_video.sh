#!/bin/bash

echo "Creating video"
python3 video.py

echo "Deleting images"

cd /Users/mmuhammad/Desktop/projects/ray-tracer/python-tracer-client/images
rm *

echo "Done"
