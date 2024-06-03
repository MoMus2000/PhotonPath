__author__ = 'halileohalilei'

from raytracer_rs import Vector, Color

class Light:
    def __init__(self, position=Vector(0, 0, 0), color=Color(1, 1, 1, 0)):
        self.position = position
        self.color = color