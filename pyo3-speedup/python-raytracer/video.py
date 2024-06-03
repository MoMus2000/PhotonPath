import cv2
import numpy as np
import glob
from tqdm import tqdm
import ffmpeg  # Make sure ffmpeg is installed

img_array = []

FPS = 60

def sorter(item):
	items = item.split("_")
	item = items[len(items)-1]
	IntVar = int("".join(filter(str.isdigit, item)))
	return IntVar

files = sorted(glob.glob('./images/test_img*'), key=sorter)

for filename in tqdm(files):
	img = cv2.imread(filename)
	height, width, layers = img.shape
	size = (width,height)
	img_array.append(img)

print(len(img_array))

output_video_path = './images/ray_traced.mp4'

out = cv2.VideoWriter(output_video_path, cv2.VideoWriter_fourcc(*'mp4v'), FPS, size)

for i in tqdm(range(len(img_array))):
    out.write(img_array[i])

out.release()

# Convert the video to MP4 using ffmpeg
ffmpeg.input(output_video_path).output('./ray_traced_final.mp4').run(overwrite_output=True)
