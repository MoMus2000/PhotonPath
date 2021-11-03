from Color import Color
from Image import Image
from Point import Point
from Sphere import Sphere
from Scene import Scene
from RenderEngine import RenderEngine
from Vector import Vector
from Light import Light
from Material import Material, ChequeredMaterial
import os
import time
from multiprocessing import cpu_count, Pool
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

Light
Apply shading algorithms to the ball
Diffuse Shading - Reflections from a matte or rough surface
Various shading models - most popular is the lambert shading model
relationship is properly captured by the cosine function

Lambert's cosine law - when light falls obliquely on a surface, 
the illumination of the surface is directly proportional to the cosine
of the angle theta between the direction of the  incident light snd the 
surface normal.

dot product and cosine
the cosine of the angle between two normalized vectors is their
dot product
cos theta = L dot N

Colorsurface = L dot N M_diffuse C_object

where 
L - Ray towwards light direction,
N - normal at hit pos.
M_diffuse - Materials diffuse coefficient
C_object - Color of object

dot product should not be negative

Specular shading - Reflections from shiny plastics
Blinn-Phong Shading model

Phong_Term = (V dot R)^k
where
V - Ray towards viewer,
R - Light ray reflected direction,
k - material's specular coefficient (shininess)

Phong-Blinn Modification
We replace V with H- halfway vecotr between the viewer
light source

H = L + V
H = norm(H)

Blinn_Term = (H dot R)^k

where 
H - Halfway angle between view and light
R - Light ray reflected direction
k - Material's specular coefficient (shininess)

Finding Reflected Ray - R

R_dir = L -2 (L dot N)N
R_pos = H_pos + N * delta

Ground plane - Hint earth is a giant sphere

"""
# import numpy as np
# from tqdm import tqdm

def main():
	lower = 0.0
	upper = 4.0
	length = 250
	arr = [lower + x*(upper-lower)/length for x in range(length)]



	lower_z = -10.6
	upper_z = -1.0

	arr_z = [lower_z + x*(upper_z-lower_z)/length for x in range(length)]

	lower_y = -0.350
	upper_y = +1.000
	arr_y = [lower_y + x*(upper_y-lower_y)/length for x in range(length)]

	three_sixty = arr
	i  = 0
	length=length*2
	for val in three_sixty:
		print(f"{int(i/length * 100)}%", end = "\r")
		if i < 10:
			create_image((val, arr_y[i], -arr_z[i]), f"/Users/a./Desktop/ray/animated/rotating_circle0{i}")
		else:
			create_image((val, arr_y[i], -arr_z[i]), f"/Users/a./Desktop/ray/animated/rotating_circle{i}")
		i+=1
	list.reverse(three_sixty)
	list.reverse(arr_y)
	list.reverse(arr_z)
	j = 0
	k = 0
	for val in three_sixty:
		print(f"{int(i/length * 100)}%", end = "\r")
		create_image((val, arr_y[j], -arr[k]), f"/Users/a./Desktop/ray/animated/rotating_circle{i}")
		i+=1
		j+=1
		k+=1
	print("", end="\r")
	print("")




def create_image(camera, img_name, compress=False):
	WIDTH = 1280
	HEIGHT = 720

	x_cam, y_cam, z_cam = camera

	camera = Vector(x_cam, y_cam, z_cam)
	objects = [
		Sphere(Point(0, 10000.5, 1), 10000.0, ChequeredMaterial(color1 = Color.from_hex("#000000"), color2 = Color.from_hex("#FFFFFF"), ambient=0.2, reflection=0.2)),
		Sphere(Point(0.75, -0.1, 1), 0.6, Material(Color.from_hex("#0000FF"))),
		Sphere(Point(-0.75, -0.1, 2.25), 0.6, Material(Color.from_hex("#803980"))),
		Sphere(Point(-2.75, -0.1, 2.25), 0.6, Material(Color.from_hex("#803980"))),
		Sphere(Point(-4.75, -0.1, 2.25), 0.6, Material(Color.from_hex("#803980"))),
		Sphere(Point(0, -10001.5, -1), 10000.0, ChequeredMaterial(color1 = Color.from_hex("#FFFFFF"), color2 = Color.from_hex("#FFFFFF"), ambient=0.01, reflection=0.1))
	]	

	lights = [
	Light(Point(1.5, -0.5, -10), Color.from_hex("#FFFFFF")),
	Light(Point(-0.5, -10.5, 0), Color.from_hex("#E6E6E6")),
	]

	scene = Scene(camera, objects, lights, WIDTH, HEIGHT)
	engine = RenderEngine()
	im = engine.render(scene)

	with open(img_name+".ppm", "w") as img_file:
		im.write_ppm(img_file)
	
	if compress:
		os.chdir("/Users/a./Desktop/ray/animated/")
		os.system(f"pnmtopng {img_name}.ppm > {img_name}.png")
		os.system(f"rm {img_name}.ppm")  


if __name__ == "__main__":
	main()