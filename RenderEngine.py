from Image import Image
from Ray import Ray
from Point import Point
from Color import Color
# from tqdm import tqdm

class RenderEngine:

	def render(self, scene):
		width = scene.width
		height = scene.height
		aspect = float(width)/height

		x0 = -1.0
		x1 =  1.0

		x_step = (x1 - x0) / (width-1)
		y0 = -1.0/aspect
		y1 =  1.0/aspect

		ystep = (y1 - y0)/(height-1)

		camera = scene.camera
		pixels = Image(width, height)

		for j in range(height):
			y = y0 + j*ystep
			for i in range(width):
				x = x0 + i*x_step
				ray = Ray(camera, Point(x,y)-camera)
				pixels.set_pixel(i, j, self.ray_trace(ray, scene))
		
		return pixels

	def ray_trace(self, ray, scene):
		color = Color(0, 0, 0)
		# Find the nearest object hit the ray in the scene
		dist , obj = self.find_nearest(ray, scene)
		if obj == None:
			return color

		hit_pos = ray.origin + ray.direction * dist

		color += self.color_at(obj, hit_pos, scene)

		return color

	def find_nearest(self, ray, scene):
		dist_min = None
		obj_hit = None
		for obj in scene.objects:
			dist = obj.intersects(ray)
			if dist != None:
				if obj_hit == None or dist < dist_min:
					dist_min = dist
					obj_hit = obj

		return dist_min, obj_hit

	def color_at(self, obj, hit_pos, scene):
		return obj.material


