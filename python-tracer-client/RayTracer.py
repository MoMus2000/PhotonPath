__author__ = 'halileohalilei'

import raytracer_rs
from raytracer_rs import Raytrace, Color

def closest_object_index(intersections):
    min_index = -1

    if len(intersections) == 0:
        return min_index
    elif len(intersections) == 1:
        if intersections[0] > 0:
            min_index = 0
    else:
        max_val = 0
        for i in range(0, len(intersections)):
            if max_val < intersections[i]:
                max_val = intersections[i]

        if max_val > 0:
            for i in range(0, len(intersections)):
                if 0 < intersections[i] <= max_val:
                    max_val = intersections[i]
                    min_index = i

    return min_index


def color_at(intersect_pos, intersect_ray_direction, scene_objects,
             closest_obj_index, lights, accuracy, ambient):

    rt = Raytrace()

    result = rt.color_at_py(intersect_pos, intersect_ray_direction, lights,
             scene_objects, closest_obj_index, accuracy, ambient, 0, Color(0.5, 0.25, 0.25, 0.1))


    return result