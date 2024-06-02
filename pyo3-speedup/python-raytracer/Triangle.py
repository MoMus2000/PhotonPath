__author__ = 'halileohalilei'

from Vector3D import Vector3D
from Color import Color


class Triangle:
    def __init__(self, a=Vector3D(1, 0, 0), b=Vector3D(0, 1, 0), c=Vector3D(0, 0, 1), color=Color(1, 1, 1)):
        self.a = a
        self.b = b
        self.c = c

        ca = self.c - self.a
        ba = self.b - self.a
        self.normal = ca.cross_product(ba).normalize()

        self.distance = self.normal.dot_product(self.a)
        self.color = color

    def normal_at(self, point):
        return self.normal

    def intersect(self, ray):
        dot = ray.direction.dot_product(self.normal)

        if dot == 0:
            return -1
        else:
            dummy = self.normal.dot_product(ray.origin + (self.normal * self.distance).negative())
            dist_to_triangle = -1 * dummy / dot

            q = ray.direction * dist_to_triangle + ray.origin

            ca = self.c - self.a
            qa = q - self.a

            bc = self.b - self.c
            qc = q - self.c

            ab = self.a - self.b
            qb = q - self.b

            inside = ca.cross_product(qa).dot_product(self.normal) >= 0 and \
                     bc.cross_product(qc).dot_product(self.normal) >= 0 and \
                     ab.cross_product(qb).dot_product(self.normal) >= 0

            if inside:
                return dist_to_triangle
            else:
                return -1