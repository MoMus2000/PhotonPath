from Image import Image
from Ray import Ray
from Point import Point
from Color import Color
# from tqdm import tqdm

class RenderEngine:
	MAX = 5
	MIN_DISPLACE = 0.0001

	def render(self, scene, img_name, sema=None):
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
		
		pixels.write_ppm(img_name)
		if sema is not None:
			sema.release()
		return 

	def ray_trace(self, ray, scene, depth=0):
		color = Color(0, 0, 0)
		# Find the nearest object hit the ray in the scene
		dist , obj = self.find_nearest(ray, scene)
		if obj == None:
			return color

		hit_pos = ray.origin + ray.direction * dist

		hit_normal = obj.normal(hit_pos)

		color += self.color_at(obj, hit_pos, hit_normal, scene)

		if depth < self.MAX:
			new_ray_position = hit_pos + hit_normal * self.MIN_DISPLACE
			new_ray_dir = (ray.direction -2 * ray.direction.dot_prod(hit_normal) * hit_normal)
			new_ray = Ray(new_ray_position, new_ray_dir)
			color += (self.ray_trace(new_ray,  scene, depth+1) * obj.material.reflection)
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

	def color_at(self, obj, hit_pos, hit_normal, scene):
		material = obj.material
		obj_color = material.color_at(hit_pos)
		to_cam = scene.camera - hit_pos
		specular_k = 50
		color = material.ambient * Color.from_hex("#FFFFFF")
		for light in scene.lights:
			to_light = Ray(hit_pos, light.position - hit_pos)
			color += obj_color * material.diffuse * max(hit_normal.dot_prod(to_light.direction), 0)
			half_vector = (to_light.direction + to_cam).normalize()
			color += light.color * material.specular * max(hit_normal.dot_prod(half_vector), 0) ** specular_k

		return color


