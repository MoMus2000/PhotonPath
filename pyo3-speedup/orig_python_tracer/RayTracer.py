__author__ = 'halileohalilei'

import math
from Ray import Ray


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
    closest_obj_color = scene_objects[closest_obj_index].color
    closest_obj_normal = scene_objects[closest_obj_index].normal_at(intersect_pos)

    if closest_obj_color.special == 2:
        square = math.floor(intersect_pos.x) + math.floor(intersect_pos.z)

        if square % 2 == 0:
            closest_obj_color.r = 0
            closest_obj_color.g = 0
            closest_obj_color.b = 0
        else:
            closest_obj_color.r = 1
            closest_obj_color.g = 1
            closest_obj_color.b = 1

    #initialize the color with the ambient light of the scene
    final_color = closest_obj_color.scale(ambient)

    if 0 < closest_obj_color.special <= 1:
        dot = closest_obj_normal.dot_product(intersect_ray_direction.negative())

        scalar1 = closest_obj_normal * dot
        add1 = scalar1 + intersect_ray_direction
        scalar2 = add1 * 2
        add2 = intersect_ray_direction.negative() + scalar2
        reflect_direction = add2.normalize()

        reflection_ray = Ray(intersect_pos, reflect_direction)

        reflect_intersections = []

        for i in range(0, len(scene_objects)):
            reflect_intersections.append(scene_objects[i].intersect(reflection_ray))

        index_closest_with_reflection = closest_object_index(reflect_intersections)

        if index_closest_with_reflection != -1 \
                and reflect_intersections[index_closest_with_reflection] > accuracy:

            reflect_intersection_pos = intersect_pos + (reflect_direction *
                                                        (reflect_intersections[index_closest_with_reflection]))
            reflect_intersection_ray_direction = reflect_direction

            reflect_intersection_color = color_at(reflect_intersection_pos, reflect_intersection_ray_direction,
                                                  scene_objects, index_closest_with_reflection,
                                                  lights, accuracy, ambient)
            final_color = final_color + reflect_intersection_color.scale(closest_obj_color.special)

    for i in range(0, len(lights)):
        light_direction = (lights[i].position + intersect_pos.negative()).normalize()
        cos = closest_obj_normal.dot_product(light_direction)

        if cos > 0:
            shadowed = False
            dist_to_light = (lights[i].position + intersect_pos.negative()).magnitude()

            shadow_ray = Ray(intersect_pos, light_direction)

            secondary_intersects = []

            for j in range(0, len(scene_objects)):
                if shadowed:
                    break
                else:
                    secondary_intersects.append(scene_objects[j].intersect(shadow_ray))

            for j in range(0, len(secondary_intersects)):
                if accuracy < secondary_intersects[j] <= dist_to_light:
                    shadowed = True

            if not shadowed:
                final_color = final_color + closest_obj_color * lights[i].color.scale(cos)
                if 0 < closest_obj_color.special <= 1:
                    dot = closest_obj_normal.dot_product(intersect_ray_direction.negative())

                    scalar1 = closest_obj_normal * dot
                    add1 = scalar1 + intersect_ray_direction
                    scalar2 = add1 * 2
                    add2 = intersect_ray_direction.negative() + scalar2
                    reflect_direction = add2.normalize()

                    specular = reflect_direction.dot_product(light_direction)

                    if specular > 0:
                        #specular = pow(specular, 10)
                        final_color = final_color + lights[i].color.scale(specular*closest_obj_color.special)

    return final_color.clip()