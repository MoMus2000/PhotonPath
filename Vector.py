import math
class Vector:
	def __init__(self, x=0.0, y=0.0, z=0.0):
		self.x = x
		self.y = y
		self.z = z

	def magnitude(self):
		return math.sqrt(self.x**2 + self.y**2 + self.z**2)

	def normalize(self):
		mag = self.magnitude()
		x = self.x/mag
		y = self.y/mag
		z = self.z/mag
		return Vector(x, y, z)

	def dot_prod(self, vec2):
		return self.x * vec2.x + self.y * vec2.y + self.z * vec2.z

	def __add__(self, vec2):
		return Vector(self.x+vec2.x, self.y+vec2.y, self.z+vec2.z)

	def __sub__(self, vec2):
		return Vector(self.x-vec2.x, self.y-vec2.y, self.z-vec2.z)

	def __mul__(self, factor):
		return Vector(self.x*factor, self.y*factor, self.z*factor)

	def __str__(self):
		return f"x = {self.x}, y = {self.y}, z = {self.z}"


if __name__ == "__main__":
	vec = Vector(1,2,3)
	vec2 = Vector(4, 5, 6)
	print(vec)
	print(vec + vec2)
	print(vec * 5)
	print(vec.magnitude())
	print(vec.normalize())
	print(vec.dot_prod(vec2))

