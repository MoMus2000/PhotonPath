__author__ = 'halileohalilei'

from Vector3D import Vector3D
# import raytracer_rs
# from raytracer_rs import Vector


class Camera:
    def __init__(self, position=Vector3D(0, 0, 0),
                 direction=Vector3D(0, 0, 1),
                 right=Vector3D(0, 0, 0),
                 down=Vector3D(0, 0, 0)):
        self.position = position
        self.direction = direction
        self.right = right
        self.down = down