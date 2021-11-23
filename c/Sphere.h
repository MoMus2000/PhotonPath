struct Sphere{
	struct Vector *center;
	float radius;
	struct Vector *material;
};


float intersects(struct Sphere *sphere, struct Ray *ray){
	struct Vector *sphere_to_ray = sub(*ray->origin, *sphere->center);
	float a = 1;
	float b = 2 * dot_prod(*ray->direction, *sphere_to_ray);
	float c = dot_prod(*sphere_to_ray, *sphere_to_ray) - (sphere->radius*sphere->radius);
	float discriminant = b*b -4*a*c;
	if(discriminant >= 0){
		float dist = (-b - sqrt(discriminant))/2.0;
		if (dist != 0){
			return dist;
		}
	}
	free(sphere_to_ray);
	return 0;
}

struct Vector normal(struct Vector surface_point, struct Sphere sp){
	struct Vector neg = *sub(surface_point, *sp.center);
	return *normalize(neg);
}