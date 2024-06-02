__author__ = 'halileohalilei'

from Vector3D import Vector3D


class Ray:
    def __init__(self, origin=Vector3D(0, 0, 0), direction=Vector3D(1, 0, 0)):
        self.origin = origin
        self.direction = direction