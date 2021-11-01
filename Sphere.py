import math
class Sphere:
	def __init__(self, center, radius, material):
		self.center = center
		self.radius = radius
		self.material = material

	def intersects(self, ray):
		# Calculate discriminant
		sphere_to_ray = ray.origin - self.center
		a = 1
		b = 2 * ray.direction.dot_prod(sphere_to_ray)
		c = sphere_to_ray.dot_prod(sphere_to_ray) - self.radius**2
		discriminant = b**2 -4*a*c

		if discriminant >=0:
			dist = (-b - math.sqrt(discriminant))/2
			if dist:
				return dist

		return None

	def normal(self, surface_point):
		return (surface_point - self.center).normalize()
