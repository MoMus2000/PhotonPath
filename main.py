from Color import Color
from Image import Image

def main():
	im = Image(3, 3)
	red = Color(1, 0, 0)
	green = Color(0, 1 , 0)
	blue = Color(0, 0, 1)
	im.set_pixel(0,0, red)
	im.set_pixel(1,0, red)
	im.set_pixel(2,0, red)

	im.set_pixel(0,1, red+green)
	im.set_pixel(1,1, red+blue)
	im.set_pixel(2,1, blue + green)

	im.set_pixel(0,2, red*0.01)
	im.set_pixel(1,2, blue*2)
	im.set_pixel(2,2, red+blue+green)

	with open("file_name.ppm", "w") as img_file:
		im.write_ppm(img_file)


if __name__ == "__main__":
	main()