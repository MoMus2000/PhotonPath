from Vector import Vector

class Color(Vector):
	@classmethod
	def from_hex(cls, hexcolor="#000000"):
		x = int(hexcolor[1:3], 16)/255.0
		y = int(hexcolor[3:5], 16)/255.0
		z = int(hexcolor[5:7], 16)/255.0
		return cls(x, y, z)

if __name__ == "__main__":
	col = Color()
	print(col.from_hex("#FF5733"))