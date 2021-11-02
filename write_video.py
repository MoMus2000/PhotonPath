import cv2
import numpy as np
import glob
from tqdm import tqdm

img_array = []

def sorter(item):
	items = item.split("/")
	item = items[len(items)-1]
	IntVar = int("".join(filter(str.isdigit, item)))
	return IntVar

files = sorted(glob.glob('/Users/a./Desktop/ray/animated/rotating_circle*'), key=sorter)

for filename in tqdm(files):
	img = cv2.imread(filename)
	height, width, layers = img.shape
	size = (width,height)
	img_array.append(img)
 
out = cv2.VideoWriter('/Users/a./Desktop/ray/ray_traced.avi',cv2.VideoWriter_fourcc(*'mp4v'), 60, size)
 
for i in tqdm(range(len(img_array))):
	out.write(img_array[i])
out.release()