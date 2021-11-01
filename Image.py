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

		img.write(f"P3 {self.width} {self.height} \n255\n")
		for i in range(0, len(self.pixels)):
			for j in range(0, len(self.pixels[0])):
				color = self.pixels[i][j]
				img.write(f"{to_byte(color.x)} {to_byte(color.y)} {to_byte(color.z)} ")
			img.write("\n")



if __name__ == "__main__":
	im = Image(5, 10)