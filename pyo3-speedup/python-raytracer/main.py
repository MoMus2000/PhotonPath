__author__ = 'halileohalilei'

from PIL import Image
from tqdm import tqdm
import RayTracer
import raytracer_rs
from raytracer_rs import Vector, Triangle, Color, Ray, Plane, Light, Scene, Camera
# from Camera import Camera

import time
import copy

start = time.time()
print("TRACIN' DEM RAYS...")


width = int(1920*0.20)
height = int(1080*0.20)

origin = Vector(0, 0, 0)
unit_x = Vector(1, 0, 0)
unit_y = Vector(0, 1, 0)
unit_z = Vector(0, 0, 1)

aa_depth = 1
aa_threshold = 0.1
aspect_ratio = float(width)/float(height)
ambient = 0.3
accuracy = 0.00000001

# sphere_center = Vector(3, -0.5, 0)

cam_position = Vector(5, 2.5, 10)

look_at = copy.deepcopy(origin)
dist = cam_position - look_at
cam_direction = dist.negative().normalize()
cam_right = unit_y.cross_product(cam_direction).normalize()
cam_down = cam_right.cross_product(cam_direction)

camera = Camera(cam_position, cam_direction, cam_right, cam_down)

white_light = Color(1.0, 1.0, 1.0, 0)
white_light_2 = Color(2.0, 2.0, 2.0, 0)
yellow_light = Color(1.0, 1.0, 0.25, 0)
yellow_light = Color(1.0, 1.0, 0.25, 0)
green = Color(0.5, 1.0, 0.5, 10)
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
light3 = Light(light_pos, white_light_2)
lights = [light]

plane = Plane(unit_y, -8, green)
sky = Plane(unit_y, 20, gray)

vertices = [
    Vector(-1, -1, -1),
    Vector(1, -1, -1),
    Vector(1, 1, -1),
    Vector(-1, 1, -1),
    Vector(-1, -1, 1),
    Vector(1, -1, 1),
    Vector(1, 1, 1),
    Vector(-1, 1, 1)
]

# Create triangles for each face of the cube
triangles = [
    Triangle(vertices[0], vertices[1], vertices[2], orange),
    Triangle(vertices[0], vertices[2], vertices[3], orange),
    Triangle(vertices[4], vertices[5], vertices[6], orange),
    Triangle(vertices[4], vertices[6], vertices[7], orange),
    Triangle(vertices[0], vertices[1], vertices[5], orange),
    Triangle(vertices[0], vertices[5], vertices[4], orange),
    Triangle(vertices[2], vertices[3], vertices[7], orange),
    Triangle(vertices[2], vertices[7], vertices[6], orange),
    Triangle(vertices[1], vertices[5], vertices[6], orange),
    Triangle(vertices[1], vertices[6], vertices[2], orange),
    Triangle(vertices[0], vertices[4], vertices[7], orange),
    Triangle(vertices[0], vertices[7], vertices[3], orange)
]

triangle = Triangle(Vector(3, 4, -3), Vector(3, -1, -3),Vector(-3, -1, -3), orange)
triangle2 = Triangle(Vector(3, 4, -3), Vector(-3, -1, -3), Vector(-3, 4, -3), orange)
triangle3 = Triangle(Vector(-3, -1, -3), Vector(-3, -1, 3), Vector(-3, 4, -3), orange)
triangle4 = Triangle(Vector(-3, -1, 3), Vector(-3, 4, 3), Vector(-3, 4, -3), orange)

scene_objects = [Scene(None, plane)]

def parse_obj_file(file_path):
    vertices = []
    triangles = []

    with open(file_path, 'r') as file:
        for line in file:
            parts = line.strip().split()

            if not parts:
                continue  # Skip empty lines

            if parts[0] == 'v':
                # Vertex information
                vertex = list(map(float, parts[1:]))
                vertices.append(vertex)

            elif parts[0] == 'f':
                # Face information
                face = [int(index.split('/')[0]) for index in parts[1:]]
                triangles.append(face)

    return vertices, triangles

# Example usage
i_ind = 1
y_ind = 1
for iter_num in range(10, 200):
    if i_ind > 15:
        i_ind = 1
    # print(i_ind)
    scene_objects = [Scene(None, plane), Scene(None, sky)]

    import random

    random_number = random.randint(-15, 15)
    filename = f"./images/test_img_{y_ind}.png"
    obj_file_path = f'./dancing-man/{i_ind}.obj'
    vertices, triangles = parse_obj_file(obj_file_path)

    triangle_objects = []
    for val in triangles:
        i1 = val[0]
        i2 = val[1]
        i3 = val[2]
        try:
            # Ensure indices are within bounds
            i1, i2, i3 = i1 - 1, i2 - 1, i3 - 1

            # Check if indices are valid

            if 0 <= i1 < len(vertices) and 0 <= i2 < len(vertices) and 0 <= i3 < len(vertices):
                triangle = Triangle(
                    Vector(*vertices[i1]) + Vector(random_number, 0, random_number),
                    Vector(*vertices[i2]) - Vector(-random_number, 0, -random_number),
                    Vector(*vertices[i3]) - Vector(-random_number, 0, -random_number),
                    reflective_black
                )

                triangle_objects.append(Scene(triangle, None))
            else:
                print("Invalid indices:", i1, i2, i3)

        except Exception as e:
            print(e)

    scene_objects.extend(triangle_objects)

    # Define a pyramid using triangles
    base_triangle1 = Triangle(Vector(-2, 0, -2), Vector(2, 0, -2), Vector(0, 4, 0), orange)  # Base
    side_triangle1 = Triangle(Vector(-2, 0, -2), Vector(0, 4, 0), Vector(-2, 0, 2), orange)  # Side 1
    side_triangle2 = Triangle(Vector(2, 0, -2), Vector(0, 4, 0), Vector(2, 0, 2), orange)   # Side 2
    side_triangle3 = Triangle(Vector(-2, 0, 2), Vector(0, 4, 0), Vector(2, 0, 2), orange)   # Side 3

    # Create a list of triangles to represent the pyramid
    # pyramid_triangles = [base_triangle1, side_triangle1, side_triangle2, side_triangle3]


    # scene_objects.extend(pyramid_triangles)

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

                    cam_ray_origin = camera.position
                    cam_ray_direction = (cam_direction +
                                        cam_right * (xamnt - 0.5) +
                                        cam_down * (yamnt - 0.5)).normalize()

                    cam_ray = Ray(cam_ray_origin, cam_ray_direction)

                    intersections = []
                    for i in range(0, len(scene_objects)):
                        a = scene_objects[i].intersect(cam_ray)
                        intersections.append(a)


                    # Looks like an issue in the triangle intersection
                    # print(intersections[72])
                    # print(scene_objects[72].triangle)

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
    i_ind += 1
    y_ind += 1
