#!/bin/bash

maturin build -i /opt/homebrew/bin/pypy3.10 --release
pypy3 -m pip install --force-reinstall /Users/mmuhammad/Desktop/projects/python-raytracer/raytracer_rs/target/wheels/raytracer_rs-0.1.0-pp310-pypy310_pp73-macosx_11_0_arm64.whl
