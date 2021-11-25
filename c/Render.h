struct find_near{
	struct Sphere *obj;
	float dist;
};

struct Vector *color_at(struct Sphere obj, struct Vector *hit_pos, struct Vector *hit_normal, struct Scene scene){
	struct Material *mt = obj.material;
	struct Vector obj_color = material_color_at(*mt);
	struct Vector to_cam = *sub(*scene.camera, *hit_pos);
	float specular_k = 50;
	struct Vector *color = malloc(sizeof (struct Vector));
	struct Vector vec = *static_mul(*from_hex("#000000"), mt->ambient);
	color->x = vec.x;
	color->y = vec.y;
	color->z = vec.z;
	for(int i=0;i<3;i++){
		struct Ray lgt = {hit_pos, sub(scene.light[i].position, *hit_pos)};
		struct Vector v = *static_mul(obj_color, mt->diffuse);
		float w = dot_prod(*hit_normal, *lgt.direction);
		if(w <= 0){
			w = 0;
		}
		color = add(*color, *static_mul(v, w));
		struct Vector half_vector = *normalize(*add(*lgt.direction, to_cam));
		struct Vector temp1 = *static_mul(*scene.light[i].color, mt->specular);
		float temp2 = dot_prod(*hit_normal, half_vector);
		if(temp2 <= 0){
			temp2 = 0;
		}
		color = add(*color, *static_mul(temp1, temp2));
		color = static_mul(*color, specular_k * specular_k);
	}

	return color;
}

struct find_near find_nearest(struct Ray *ray, struct Scene *scene){
	float dist_min = INFINITY;
	struct Sphere *obj_hit = NULL;
	for(int i=0; i<1;i++){
		float dist = intersects(&scene->objects[i], ray);
		if(dist != 0){
			if(obj_hit != NULL || dist < dist_min){
				dist_min = dist;
				obj_hit = &scene->objects[i];
			}
		}
	}
	struct find_near fn2 = {obj_hit, dist_min};
	return fn2;
}

struct Vector ray_trace(struct Ray *ray, struct Scene scene){
	struct Vector color = {0, 0, 0};
	struct find_near fn = find_nearest(ray, &scene);
	float dist = fn.dist;
	struct Sphere *obj = fn.obj;

	if(obj == NULL){
		return color;
	}
	struct Vector *add_two = add(*ray->origin, *ray->direction);
	struct Vector *hit_pos = static_mul(*add_two, dist);

	struct Vector *hit_normal = normalize(*hit_pos);
	color = *add(color, *color_at(*obj, hit_pos, hit_normal, scene));

	free(hit_pos);
	free(add_two);

	return color;
}

struct Image *render(struct Scene scene){
	float width = scene.width;
	float height = scene.height;
	float aspect = width/height;

	float x0 = -1.0;
	float x1 =  1.0;
	float x_step = (x1 - x0)/(width -1);
	float y0 = -1.0/aspect;
	float y1 =  1.0/aspect;

	float y_step = (y1 - y0)/(height -1);

	struct Vector cam = *scene.camera;
	struct Image *img = malloc(sizeof(struct Image));

	for(int j= 0; j< HEIGHT; j++){
		float y = y0 + j*y_step;
		for(int i=0; i< WIDTH; i++){
			float x = x0 + i*x_step;
			struct Ray *ray = malloc(sizeof(struct Image));
			struct Vector point = {x,y,0.0};
			ray->origin = &cam;
			struct Vector *subtract = sub(point,cam);
			ray->direction = normalize(*subtract);
			set_pixel(img, i,j, ray_trace(ray, scene));
			free(ray->direction);
			free(ray);
			free(subtract);
		}
	}

	return img;
}

