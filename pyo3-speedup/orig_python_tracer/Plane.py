__author__ = 'halileohalilei'

from Color import Color
from Vector3D import Vector3D


class Plane:
    def __init__(self, normal=Vector3D(1, 0, 0), distance=0.0, color=Color(0.5, 0.5, 0.5, 0)):
        self.normal = normal
        self.distance = distance
        self.color = color

    def normal_at(self, point):
        return self.normal

    def intersect(self, ray):
        dot = ray.direction.dot_product(self.normal)
        if dot == 0:
            return -1
        else:
            dummy = self.normal.dot_product(ray.origin + (self.normal * self.distance).negative())
            return -1 * dummy / dot