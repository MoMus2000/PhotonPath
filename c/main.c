#define NUM_SPHERES 4
#define NUM_LIGHTS 2

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
		struct Vector suface = {0.0, 10000.5, 1.0};
		struct Vector *surface_color = from_hex("#e6b87d");
		struct Material sp_mt = {surface_color, 0.05, 1.0, 1.0, 0.05};
		struct Sphere surface_sp = {&suface, 10000.0, &sp_mt};

		// surface
		struct Vector suface_1 = {0.0, 10000.5, 1.0};
		struct Vector *surface_color_1 = from_hex("#e6b87d");
		struct Material sp_mt_1 = {surface_color_1, 0.05, 1.0, 1.0, 0.05};
		struct Sphere surface_sp_1 = {&suface_1, 10000.0, &sp_mt_1};

		struct Sphere *objects[NUM_SPHERES] = {
			&surface_sp_1, &surface_sp, &sp, &sp_1
		};

		struct Vector light_vec = {1.5, -0.5, -10.0};
		struct Light light_1 = {light_vec, from_hex("#FFFFFF")};

		struct Vector light_vec_2 = {-0.5, -10.5, 0};
		struct Light light_2 = {light_vec_2, from_hex("#E6E6E6")};

		struct Light *lights[NUM_LIGHTS] = {&light_1, &light_2};

		struct Scene scene = {&camera, *objects, *lights, WIDTH, HEIGHT};

		// Copy all values in the scene array;
		for(int j=1; j<NUM_SPHERES;j++){
			scene.objects[j].center = objects[j]->center;
			scene.objects[j].radius = objects[j]->radius;
			scene.objects[j].material = objects[j]->material;
		}

		for(int j=0; j<NUM_LIGHTS;j++){
			scene.light[j].position = lights[j]->position;
			scene.light[j].color = lights[j]->color;
		}

		printf("CAMERA\n");
		print(scene.camera);

		printf("SPHERES:\n");
		for(int i=1; i<NUM_SPHERES; i++){
			print(scene.objects[i].center);
			printf("radius %f\n",scene.objects[i].radius);
		}

		printf("LIGHTS:\n");
		for(int i=0; i<NUM_LIGHTS; i++){
			print(&scene.light[i].position);
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
