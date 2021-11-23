struct Material{
	struct Vector color;
	struct Vector ambient;
	struct Vector diffuse;
	struct Vector specular;
};

struct Vector material_color_at(struct Material mt){
	return mt.color;
}