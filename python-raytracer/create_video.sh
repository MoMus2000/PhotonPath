#!/bin/bash

echo "Creating video"
python3 video.py

echo "Deleting images"

cd /Users/mmuhammad/Desktop/projects/ray-tracer/pyo3-speedup/python-raytracer/images
rm *

echo "Done"
