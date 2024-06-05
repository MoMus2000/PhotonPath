__author__ = 'halileohalilei'

# from raytracer_rs import Vector, Color
from Vector3D import Vector3D
from Color import Color

class Light:
    def __init__(self, position=Vector3D(0, 0, 0), color=Color(1, 1, 1, 0)):
        self.position = position
        self.color = color