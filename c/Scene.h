struct Scene{
	struct Vector *camera;
	struct Sphere *objects;
	struct Light* light;
	int width;
	int height;
};

