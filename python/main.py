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
from multiprocessing import Pool,cpu_count, Process, Semaphore
from PIL import Image as Im
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

PROCESSES = []
# sema = Semaphore(3)
multi = True
# pool = Pool(cpu_count()-1)

def main():
	print("Running")
	lower = 0.0
	upper = 4.0
	length = 1
	arr = [lower + x*(upper-lower)/length for x in range(length)]



	lower_z = -10.6
	upper_z = -1.0

	arr_z = [lower_z + x*(upper_z-lower_z)/length for x in range(length)]

	lower_y = -0.350
	upper_y = +0.0
	arr_y = [lower_y + x*(upper_y-lower_y)/length for x in range(length)]

	three_sixty = arr
	i  = 0
	length=length*3
    
	for val in three_sixty:
		print("Running")
		# print(f"{int(i/length * 100)}%", end = "\r")
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
		create_image((val, arr_y[j], -arr[k]), f"/Users/a./Desktop/ray/animated/rotating_circle{i}", sphere_move=(val, 0 ,0))
		i+=1
		j+=1
		k+=1

	list.reverse(three_sixty)
	list.reverse(arr_y)
	list.reverse(arr_z)
	
	for k in range(int(len(three_sixty))):
		print(f"{int(i/length * 100)}%", end = "\r")
		create_image((three_sixty[k], arr_y[k], -arr[k]), f"/Users/a./Desktop/ray/animated/rotating_circle{i}")
		i+=1


	print("", end="\r")
	print("")




def create_image(camera, img_name, sphere_move=None, compress=True):
	WIDTH =  int(1280*0.9)
	HEIGHT = int(512*0.9)

	x_cam, y_cam, z_cam = camera
	if sphere_move is not None:
		sphere_x, sphere_y, sphere_z = sphere_move
	else:
		sphere_x = 0
		sphere_y = 0
		sphere_z = 0

	camera = Vector(x_cam, y_cam, z_cam)
	objects = [
		Sphere(Point(0, 10000.5, 1), 10000.0, ChequeredMaterial(color1 = Color.from_hex("#000000"), color2 = Color.from_hex("#FFFFFF"), ambient=0.2, reflection=0.2)),
		Sphere(Point(0.75, -0.1, 1), 0.6, Material(Color.from_hex("#0000FF"))),
		Sphere(Point(-0.75+sphere_x, -0.1, 2.25), 0.6, Material(Color.from_hex("#008E97"))),
		Sphere(Point(-2.75, -0.1, 2.25-sphere_x), 0.6, Material(Color.from_hex("#FC8EAC"))),
		Sphere(Point(-4.75+sphere_x, -0.1, 2.25), 0.6, Material(Color.from_hex("#CCCCCC"))),
		Sphere(Point(0, -10001.5, -1), 10000.0, ChequeredMaterial(color1 = Color.from_hex("#FFFFFF"), color2 = Color.from_hex("#FFFFFF"), ambient=0.01, reflection=0.1))
	]	

	lights = [
	Light(Point(1.5, -0.5, -10), Color.from_hex("#FFFFFF")),
	Light(Point(-0.5, -10.5, 0), Color.from_hex("#E6E6E6")),
	]

	scene = Scene(camera, objects, lights, WIDTH, HEIGHT)
	scene_list = (scene, img_name+".png")

	
	engine = RenderEngine()
	
	global multi
	global pool
	if multi:
		# sema.acquire()
		# with pool as p:
			# p.starmap_async(engine.render, scene_list)
		# p = Process(target=engine.render, args=(scene, img_name+".png", sema))
		PROCESSES.append(scene_list)
		# p.start()
	else:
		engine.render(scene, img_name+".png")
	
	# if compress:
	# 	os.chdir("/Users/a./Desktop/ray/animated/")
	# 	os.system(f"pnmtopng {img_name}.ppm > {img_name}.png")
	# 	os.system(f"rm {img_name}.ppm")  

def create_image_custom(camera, img_name, sphere_move=None, compress=True):
	WIDTH =  64
	HEIGHT = 256

	x_cam, y_cam, z_cam = camera

	camera = Vector(x_cam, y_cam, z_cam)
	objects = [
		Sphere(Point(0, 10000.5, 1), 10000.0, ChequeredMaterial(color1 = Color.from_hex("#000000"), color2 = Color.from_hex("#FFFFFF"), ambient=0.2, reflection=0.2)),
		Sphere(Point(0.75, -0.1, 1), 0.6, Material(Color.from_hex("#0000FF"))),
		Sphere(Point(-0.75, -0.1, 2.25), 0.6, Material(Color.from_hex("#008E97"))),
		Sphere(Point(-2.75, -0.1, 2.25), 0.6, Material(Color.from_hex("#FC8EAC"))),
		Sphere(Point(-4.75, -0.1, 2.25), 0.6, Material(Color.from_hex("#CCCCCC"))),
		Sphere(Point(0, -10001.5, -1), 10000.0, ChequeredMaterial(color1 = Color.from_hex("#FFFFFF"), color2 = Color.from_hex("#FFFFFF"), ambient=0.01, reflection=0.1))
	]	

	lights = [
	Light(Point(1.5, -0.5, -10), Color.from_hex("#FFFFFF")),
	Light(Point(-0.5, -10.5, 0), Color.from_hex("#E6E6E6")),
	]

	scene = Scene(camera, objects, lights, WIDTH, HEIGHT)
	engine = RenderEngine()
	im = engine.render(scene)

	im.write_ppm(img_file)

def run_render(procs):
	engine = RenderEngine()
	p = Pool(cpu_count()-1)
	p.starmap_async(engine.render, procs)
	p.close()
	p.join()
	# for proc in procs:
		# a, b = proc
		# print(a, b)
		# engine.render(a, b)
		# a = p.starmap_async(engine.render, (a, b))
		# a.get()
	# p.close()
	# p.join()
	# with Pool(cpu_count()-1) as p:
	# p.starmap_async(engine.render, scenes)
	return

if __name__ == "__main__":
	start = time.time()
	main()
	run_render(PROCESSES)
	end = time.time()
	print(end - start)
