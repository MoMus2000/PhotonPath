__author__ = 'halileohalilei'

import math


class Sphere:
    def __init__(self, center, radius, color):
        self.center = center
        self.radius = radius
        self.color = color

    def normal_at(self, point):
        return (point + self.center.negative()).normalize()

    def intersect(self, ray):
        ray_origin = ray.origin
        ray_direction = ray.direction

        a = 1
        b = (2*(ray_origin.x - self.center.x)*ray_direction.x) \
            + (2*(ray_origin.y - self.center.y)*ray_direction.y) \
            + (2*(ray_origin.z - self.center.z)*ray_direction.z)
        c = pow(ray_origin.x - self.center.x, 2) + \
            pow(ray_origin.y - self.center.y, 2) + \
            pow(ray_origin.z - self.center.z, 2) - \
            pow(self.radius, 2)

        discriminant = b*b - 4*a*c

        if discriminant >= 0:
            first_root = (-1 * b - math.sqrt(discriminant)) / 2 - 0.000001
            if first_root > 0:
                return first_root
            else:
                second_root = (math.sqrt(discriminant) - b) / 2 - 0.000001
                return second_root
        else:
            return -1
