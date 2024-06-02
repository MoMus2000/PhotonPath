__author__ = 'halileohalilei'

import math


class Vector3D:
    def __init__(self, x=0, y=0, z=0):
        self.x = x
        self.y = y
        self.z = z

    def __add__(self, other):
        return Vector3D(self.x + other.x, self.y + other.y, self.z + other.z)

    def __mul__(self, other):
        return Vector3D(self.x * other, self.y * other, self.z * other)

    def __sub__(self, other):
        return Vector3D(self.x - other.x, self.y - other.y, self.z - other.z)

    def __str__(self):
        return "x: %.2f y: %.2f z: %.2f" % (self.x, self.y, self.z)

    def magnitude(self):
        return math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z)

    def normalize(self):
        magnitude = self.magnitude()
        return Vector3D(self.x/magnitude, self.y/magnitude, self.z/magnitude)

    def negative(self):
        return Vector3D(-self.x, -self.y, -self.z)

    def dot_product(self, other):
        return self.x * other.x + self.y * other.y + self.z * other.z

    def cross_product(self, other):
        return Vector3D(self.y * other.z - self.z * other.y,
                        self.z * other.x - self.x * other.z,
                        self.x * other.y - self.y * other.x)
