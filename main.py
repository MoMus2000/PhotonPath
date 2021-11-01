from Color import Color
from Image import Image
from Point import Point
from Sphere import Sphere
from Scene import Scene
from RenderEngine import RenderEngine
from Vector import Vector
from Light import Light
from Material import Material

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


"""
# import numpy as np
# from tqdm import tqdm

def main():
	lower = 0
	upper = 0.5
	length = 150
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
	WIDTH =  1024
	HEIGHT = 600

	camera = Vector(0, 0, -1)
	objects = [Sphere(Point(0, -first_arg/4, first_arg), first_arg, Material(Color.from_hex("#FF0000")))]
	lights = [Light(Point(1.5, -0.5, -10.0), Color.from_hex("#FFFFFF"))]

	scene = Scene(camera, objects, lights, WIDTH, HEIGHT)
	engine = RenderEngine()
	im = engine.render(scene)

	with open(img_name, "w") as img_file:
		im.write_ppm(img_file)


if __name__ == "__main__":
	main()
