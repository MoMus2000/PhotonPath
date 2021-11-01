from Color import Color
from Image import Image
from Point import Point
from Sphere import Sphere
from Scene import Scene
from RenderEngine import RenderEngine
from Vector import Vector

# Render a 3d ball without shading

# Ray tracing algorithm simplified and coded in the engine
# 1. Shoot a ray towards each pixel
# 2. Find the nearest object hit by the ray in the scene
# 3. If hit then find color at surface of the object

"""
sphere_to_ray = ray_origin - sphere_center
a = 1
b = 2*ray_dir dot sphere_to_ray
c = sphere_to_ray dot sphere_to_ray - sphere_center**2
discriminant = b**2 - 4ac

when calculating the distance we only consider the negative
sqrt of the quadratic formula because the camera is in the
negative x.

ray = origin + ray_dir * t
replace t with distance to get the hit position.
hit_pos = ray_origin + ray_direction * dist

aspect ratio is calculated because the dimensions of width
and height might be different
"""
# import numpy as np
# from tqdm import tqdm

def main():
	lower = 0
	upper = 0.5
	length = 100
	arr = [lower + x*(upper-lower)/length for x in range(length)]
	three_sixty = arr
	i  = 0
	length=length*2
	for val in three_sixty:
		print(f"{(i/length * 100)}.{2} %", end = "\r")
		if i < 10:
			create_image(val, f"/Users/a./Desktop/ray/animated/rotating_circle0{i}.ppm")
		else:
			create_image(val, f"/Users/a./Desktop/ray/animated/rotating_circle{i}.ppm")
		i+=1
	list.reverse(three_sixty)
	for val in three_sixty:
		print(f"{(i/length * 100)}.{2} %", end = "\r")
		create_image(val, f"/Users/a./Desktop/ray/animated/rotating_circle{i}.ppm")
		i+=1
	print("", end="\r")
	print("")



def create_image(first_arg, img_name):
	WIDTH =  512
	HEIGHT = 256

	camera = Vector(first_arg, 0, -1)
	objects = [Sphere(Point(0, 0, 0), first_arg, Color.from_hex("#FF0000"))]
	scene = Scene(camera, objects, WIDTH, HEIGHT)
	engine = RenderEngine()
	im = engine.render(scene)

	with open(img_name, "w") as img_file:
		im.write_ppm(img_file)


if __name__ == "__main__":
	main()