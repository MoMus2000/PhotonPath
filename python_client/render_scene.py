from PIL import Image
from tqdm import tqdm
import numpy as np
from photon_path import Vector, Triangle, Color, Ray, Plane, Light, Scene, Sphere, Camera, RenderScene
from photon_path.image import HD, UHD, FHD, SD, VLR, NHD, LR
import time
import copy


start = time.time()

width = NHD[0]
height = NHD[1]

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
# sphere_t = Sphere(Vector(3, 0, -3), 1, green)
sphere_t = Sphere(Vector(0, 0, -0), 1, green)
sphere2 = Sphere(sphere_center, 0.5, maroon)
sphere3 = Sphere(Vector(0, 2, 0), 1, blue)
plane = Plane(unit_y, -1, reflective_black)

triangle = Triangle(Vector(3, 4, -3), Vector(3, -1, -3),Vector(-3, -1, -3), orange)
triangle2 = Triangle(Vector(3, 4, -3), Vector(-3, -1, -3), Vector(-3, 4, -3), orange)
triangle3 = Triangle(Vector(-3, -1, -3), Vector(-3, -1, 3), Vector(-3, 4, -3), orange)
triangle4 = Triangle(Vector(-3, -1, 3), Vector(-3, 4, 3), Vector(-3, 4, -3), orange)


scene_objects = [
    Scene(None, None, sphere), 
    Scene(None, None, sphere_t), 
    Scene(None, None, sphere2),
    Scene(None, None, sphere3),
    Scene(None, plane, None),
]

o_count = 0

render_objects = []

for i in range(0, 20):
    sub = np.linspace(i, i+1, 120)
    for idx, val in enumerate(sub):
        renderer = RenderScene(scene_objects,camera,lights, width, height, ambient, accuracy)
        render_objects.append(renderer)
        cam_position = Vector(5*val, 2.5, val*5)
        camera = Camera(cam_position, cam_direction, cam_right, cam_down)
        sphere = scene_objects[1].sphere
        sphere.center = Vector(val*4.45, 0, val*4.45)
        scene_objects[1] = Scene(None, None, sphere)

        o_count += 1


    if i % 6 == 0:
        light = Light(Vector(i, i, i), white_light)
        lights.append(light)

RenderScene.par_render(render_objects, width, height, folder_path="/Users/mmuhammad/Desktop/projects/ray-tracer/python_client")
