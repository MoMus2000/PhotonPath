__author__ = 'halileohalilei'

from PIL import Image
import RayTracer
from tqdm import tqdm
from raytracer_rs import Vector, Triangle, Color, Ray, Plane, Light, Scene, Sphere, Camera

import time
import copy

import faulthandler

faulthandler.enable()

start = time.time()
print("TRACIN' DEM RAYS...")

filename = "test_img5.png"

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
    Scene(triangle, None, None),
    Scene(triangle2, None, None),
    Scene(triangle3, None, None),
    Scene(triangle4, None, None)
]

image = Image.new("RGB", (width, height))

for x in tqdm(range(0, width)):
    for y in range(0, height):
        temp_red = [None] * aa_depth * aa_depth
        temp_green = [None] * aa_depth * aa_depth
        temp_blue = [None] * aa_depth * aa_depth

        for aax in range(0, aa_depth):
            for aay in range(0, aa_depth):
                aa_index = aay * aa_depth + aax

                if aa_depth == 1:
                    if width > height:
                        xamnt = ((x + 0.5) / width) * aspect_ratio - (((width - height) / float(height))/2)
                        yamnt = ((height - y) + 0.5) / height
                    elif height > width:
                        xamnt = (x + 0.5) / width
                        yamnt = (((height - y) + 0.5)/height)/aspect_ratio - (((height - width)/float(width))/2)
                    else:
                        xamnt = (x + 0.5) / width
                        yamnt = ((height - y) + 0.5) / height
                else:
                    if width > height:
                        xamnt = ((x + float(aax) / (float(aa_depth) - 1)) / width) * aspect_ratio - \
                                (((width - height) / float(height)) / 2)
                        yamnt = ((height - y) + float(aax)/(float(aa_depth) - 1))/height
                    elif height > width:
                        xamnt = (x + float(aax)/(float(aa_depth) - 1)) / width
                        yamnt = (((height - y) + float(aax) / (float(aa_depth) - 1)) / height) / aspect_ratio - \
                                (((height - width) / float(width))/2)
                    else:
                        xamnt = (x + float(aax) / (float(aa_depth) - 1)) / width
                        yamnt = ((height - y) + float(aax) / (float(aa_depth) - 1)) / height
                

                # print("XAMNT, YAMNT: ", xamnt, yamnt)

                cam_ray_origin = camera.position
                cam_ray_direction = (cam_direction +
                                     cam_right * (xamnt - 0.5) +
                                     cam_down * (yamnt - 0.5)).normalize()

                cam_ray = Ray(cam_ray_origin, cam_ray_direction)


                intersections = []
                for i in range(0, len(scene_objects)):
                    intersections.append(scene_objects[i].intersect(cam_ray))

                closest_obj_index = RayTracer.closest_object_index(intersections)


                if closest_obj_index == -1:
                    temp_red[aa_index] = 0
                    temp_green[aa_index] = 0
                    temp_blue[aa_index] = 0
                else:
                    if intersections[closest_obj_index] > accuracy:
                        intersect_pos = cam_ray_origin + cam_ray_direction * intersections[closest_obj_index]
                        intersect_ray_direction = cam_ray_direction

                        intersection_color = RayTracer.color_at(intersect_pos, intersect_ray_direction,
                                                                scene_objects, closest_obj_index, lights,
                                                                accuracy, ambient)
                        
                        temp_red[aa_index] = intersection_color.r
                        temp_green[aa_index] = intersection_color.g
                        temp_blue[aa_index] = intersection_color.b

        avg_red = sum(temp_red) / (aa_depth * aa_depth)
        avg_green = sum(temp_green) / (aa_depth * aa_depth)
        avg_blue = sum(temp_blue) / (aa_depth * aa_depth)

        image.putpixel((x, height - y - 1), (int(round(avg_red * 255)),
                                             int(round(avg_green * 255)),
                                             int(round(avg_blue * 255))))

image.save(filename)

end = time.time() - start
print("%d seconds" % end)
