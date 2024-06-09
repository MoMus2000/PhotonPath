import numpy as np
from PIL import Image
import glob
import pickle

files = glob.glob('/Users/a./Desktop/ray/animated/*')

for i, file in enumerate(files):
	with open(file, 'rb') as f:
		arr = pickle.load(f)

	arr = np.array(arr)
	im = Image.fromarray((arr * 255).astype(np.uint8))
	im.save(f"/Users/a./Desktop/ray/animated/test{i}.png")
