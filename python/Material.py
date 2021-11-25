from Color import Color
import math

class Material:
	def __init__(self, color=Color.from_hex("#FFFFFF"), ambient=0.05, diffuse=1.0, specular=1.0, reflection=0.5):
		self.color = color
		self.ambient = ambient
		self.diffuse = diffuse
		self.specular = specular
		self.reflection = reflection

	def color_at(self, position):
		return self.color


class ChequeredMaterial:
	def __init__(self, color1=Color.from_hex("#FFFFFF"), color2=Color.from_hex("#000000"), ambient=0.05, diffuse=1.0, specular=1.0, reflection=0.5):
		self.color1 = color1
		self.color2 = color2
		self.ambient = ambient
		self.diffuse = diffuse
		self.specular = specular
		self.reflection = reflection

	def color_at(self, position):
		if math.floor((position.x + 5.0) * 3.0) % 2 == math.floor(position.z * 3.0) %2:
			return self.color1
		return self.color2
