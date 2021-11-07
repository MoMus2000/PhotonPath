import numpy as np
from PIL import Image as im
class Image:
	def __init__(self, width, height):
		self.width = width
		self.height = height
		self.pixels = [[None]* width for _ in range(height)]

	def set_pixel(self, i, j, Color):
		self.pixels[j][i] = Color


	def write_ppm(self, img):
		def to_byte(c):
			if c*255 > 255: return round(255)
			if c*255 < 0:   return round(0)
			return round(c*255)

		array = []
		for i in range(0, len(self.pixels)):
			for j in range(0, len(self.pixels[0])):
				color = self.pixels[i][j]
				array.append(to_byte(color.x))
				array.append(to_byte(color.y))
				array.append(to_byte(color.z))

		array = np.array(array, dtype=np.uint8).reshape((int(self.height),int(self.width),3))
		im.fromarray(array, 'RGB').save(img)
		# imgs.close()



if __name__ == "__main__":
	im = Image(5, 10)