__author__ = 'halileohalilei'

from PIL import Image
import RayTracer
from tqdm import tqdm
import numpy as np
from raytracer_rs import Vector, Triangle, Color, Ray, Plane, Light, Scene, Sphere, Camera, RenderScene

import time
import copy

start = time.time()
print("TRACIN' DEM RAYS...")

width = int(640*2)
height = int(480*2)

origin = Vector(0, 0, 0)
unit_x = Vector(1, 0, 0)
unit_y = Vector(0, 1, 0)
unit_z = Vector(0, 0, 1)

aa_depth = 1
aa_threshold = 0.1
aspect_ratio = float(width)/float(height)
ambient = 0.1
accuracy = 0.00000001

sphere_center = Vector(3, -0.5, 0)

cam_position = Vector(5, 2.5, 5)

look_at = copy.deepcopy(origin)
dist = cam_position - look_at

cam_direction = dist.negative().normalize()
cam_right = unit_y.cross_product(cam_direction).normalize()
cam_down = cam_right.cross_product(cam_direction)

camera = Camera(cam_position, cam_direction, cam_right, cam_down)

white_light = Color(1.0, 1.0, 1.0, 0)
yellow_light = Color(1.0, 1.0, 0.25, 0)
green = Color(0.5, 1.0, 0.5, 0.3)
maroon = Color(0.5, 0.25, 0.25, 0.1)
tile_floor = Color(1, 1, 1, 2)
orange = Color(1.0, 0.5, 0.0, 0.0)
gray = Color(0.5, 0.5, 0.5, 0)
black = Color(0.0, 0.0, 0.0, 0)
blue = Color(0.2, 0.2, 1, 0.1)
reflective_black = Color(0.0, 0.0, 0.0, 0.3)

light_pos = Vector(0, 5, 10)
light = Light(light_pos, white_light)
light2 = Light(Vector(5, 5, 5), yellow_light)
lights = [light, light2]

sphere = Sphere(origin, 1, green)
sphere2 = Sphere(sphere_center, 0.5, maroon)
sphere3 = Sphere(Vector(0, 2, 0), 1, blue)
plane = Plane(unit_y, -1, tile_floor)

triangle = Triangle(Vector(3, 4, -3), Vector(3, -1, -3),Vector(-3, -1, -3), orange)
triangle2 = Triangle(Vector(3, 4, -3), Vector(-3, -1, -3), Vector(-3, 4, -3), orange)
triangle3 = Triangle(Vector(-3, -1, -3), Vector(-3, -1, 3), Vector(-3, 4, -3), orange)
triangle4 = Triangle(Vector(-3, -1, 3), Vector(-3, 4, 3), Vector(-3, 4, -3), orange)


scene_objects = [
    Scene(None, None, sphere), 
    Scene(None, None, sphere2),
    Scene(None, None, sphere3),
    Scene(None, plane, None),
]



image = Image.new("RGB", (width, height))

o_count = 0
for i in tqdm(range(0, 5)):
    sub = np.linspace(i, i+1, 60)
    for idx, val in enumerate(sub):
        renderer = RenderScene(scene_objects,camera,lights, width, height, ambient, accuracy)
        img_colors = renderer.render()
        count = 0 
        for x in range(0, width):
            for y in range(0, height):

                color = img_colors[count]
                count += 1

                image.putpixel((x, height - y - 1), color)

        filename = f"./images/rs_rt_{o_count}.png"
        image.save(filename)
        if i <=5:
            cam_position = Vector(5*val, 2.5, val*5)
        camera = Camera(cam_position, cam_direction, cam_right, cam_down)

        o_count += 1

for i in tqdm(range(5, 0, -1)):
    sub = np.linspace(i, i-1, 60)
    for idx, val in enumerate(sub):
        renderer = RenderScene(scene_objects,camera,lights, width, height, ambient, accuracy)
        img_colors = renderer.render()
        count = 0 
        for x in range(0, width):
            for y in range(0, height):

                color = img_colors[count]
                count += 1

                image.putpixel((x, height - y - 1), color)

        filename = f"./images/rs_rt_{o_count}.png"
        image.save(filename)
        if i <=5:
            cam_position = Vector(5*val, 2.5, val*5)
        camera = Camera(cam_position, cam_direction, cam_right, cam_down)

        o_count += 1