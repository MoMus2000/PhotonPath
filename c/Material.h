struct Material{
	struct Vector *color;
	float ambient;
	float diffuse;
	float specular;
};

struct Vector material_color_at(struct Material mt){
	return *mt.color;
};

struct Checkered_Material{
	struct Vector color1;
	struct Vector color2;
	struct Vector ambient;
	struct Vector diffuse;
	struct Vector specular;
};

struct Vector checkered_material_color_at(struct Checkered_Material mt){
	return mt.color1;
};