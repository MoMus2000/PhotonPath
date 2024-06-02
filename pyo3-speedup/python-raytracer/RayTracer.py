import raytracer_rs

def color_at(intersect_pos, intersect_ray_direction, scene_objects,
             closest_obj_index, lights, accuracy, ambient):

    result = raytracer_rs.color_at(intersect_pos, intersect_ray_direction, scene_objects,
             closest_obj_index, lights, accuracy, ambient)

    return result
