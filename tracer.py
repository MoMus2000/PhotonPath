from Color import Color
from Image import Image
from Point import Point
from Sphere import Sphere
from Scene import Scene
from RenderEngine import RenderEngine
from Vector import Vector
from Light import Light
from Material import Material, ChequeredMaterial

WIDTH = 320
HEIGHT = 200

RENDERED = "2balls.ppm"
CAMERA = Vector(0, -0.35, -1)
OBJECTS = [
	Sphere(Point(0, 10000.5, 1), 10000.0, ChequeredMaterial(color1 = Color.from_hex("#420500"), color2 = Color.from_hex("e6b87d"), ambient=0.2, reflection=0.2)),
	Sphere(Point(0.75, -0.1, 1), 0.6, Material(Color.from_hex("#0000FF"))),
	Sphere(Point(-0.75, -0.1, 2.25), 0.6, Material(Color.from_hex("#803980")))

]
LIGHTS = [Light(Point(1.5, -0.5, -10), Color.from_hex("#FFFFFF")),
Light(Point(-0.5, -10.5, 0), Color.from_hex("#E6E6E6"))
]

