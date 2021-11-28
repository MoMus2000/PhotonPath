#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

// Headers
#include "Vec.h"
#include "Img.h"
#include "Ray.h"
#include "Sphere.h"
#include "Light.h"
#include "Material.h"
#include "Scene.h"
#include "Render.h"

#define NUM_SPHERES 2
#define NUM_LIGHTS 2


int main(int argc, char const *argv[])
{
	int n = 1;
	char s1[100] = "";
	char s2[100] = "";

	for(int i =0; i<1;i++){
		// fprintf(stderr, "Progress: %.*s%.*s %02f\r", i, s1, n-i, s2, i/(float)n);
		struct Vector camera = {0, -0.35, -1};

		//blue sphere
		struct Vector center = {0.75, -0.1, 1};
		struct Vector *col = from_hex("#0000FF");
		struct Material mt = {col, 0.05, 1.0, 1.0, 0.05};
		struct Sphere sp = {&center, 0.6, &mt};

		//pink sphere
		struct Vector center_1 = {-0.75, -0.1, 2.25};
		struct Vector *col_1 = from_hex("#803980");
		struct Material mt_1 = {col_1, 0.05, 1.0, 1.0, 0.05};
		struct Sphere sp_1 = {&center_1, 0.6, &mt_1};
		
		// surface
		struct Vector suface = {0, 10000.5, 1};
		struct Vector *surface_color = from_hex("#e6b87d");
		struct Material sp_mt = {surface_color, 0.05, 1.0, 1.0, 0.05};
		struct Sphere surface_sp = {&suface, 10000.0, &sp_mt};

		struct Sphere *objects[NUM_SPHERES] = {
			&sp, &sp_1
		};

		struct Vector light_vec = {1.5, -0.5, -10.0};
		struct Light light_1 = {light_vec, from_hex("#E6E6E6")};

		struct Vector light_vec_2 = {-0.5, -10.5, 0};
		struct Light light_2 = {light_vec_2, from_hex("#E6E6E6")};

		struct Light *lights[NUM_LIGHTS] = {&light_1, &light_2};

		struct Scene scene = {&camera, *objects, *lights, WIDTH, HEIGHT};

		// Copy all values in the scene array;
		for(int j=0; j<NUM_SPHERES;j++){
			scene.objects[j].center = objects[j]->center;
			scene.objects[j].radius = objects[j]->radius;
			scene.objects[j].material = objects[j]->material;
		}

		for(int j=0; j<NUM_LIGHTS;j++){
			scene.light[j].position = lights[j]->position;
			scene.light[j].color = lights[j]->color;
		}
		
		struct Image *im = render(scene); 

		write_ppm(im, "moodle.ppm");

		// free(im);
		// free(col);
		// fflush(stderr);
	}
	fflush(stderr);
	return 0;
}





// struct Vector red = {1,0,0};
	// struct Vector blue = {0,1,0};
	// struct Vector green = {0,0,1};
	// struct Image *img = malloc(sizeof(struct Image));
	
	// set_pixel(img, 0, 0, red);
	// set_pixel(img, 1, 0, blue);
	// set_pixel(img, 2, 0, green);

	// set_pixel(img, 0, 1, *add(red, green));
	// set_pixel(img, 1, 1, *add(red, blue));
	// set_pixel(img, 2, 1, *add(blue, green));

	// set_pixel(img, 0, 2, *static_mul(red, 0.01));
	// set_pixel(img, 1, 2, *static_mul(blue, 2));
	// set_pixel(img, 2, 2, *add(blue, *add(red, green)));

	// struct Ray ray = {&red, normalize(red)};

	// write_ppm(img);
