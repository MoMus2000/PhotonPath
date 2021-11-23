#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

// Headers
#include "Vec.h"
#include "Img.h"
#include "Ray.h"
#include "Sphere.h"
#include "Scene.h"
#include "Render.h"
#include "Light.h"
#include "Material.h"


int main(int argc, char const *argv[])
{
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

	int n = 200;
	char s1[100] = "";
	char s2[100] = "";

	for(int i =0; i<200;i++){
		fprintf(stderr, "Progress: %.*s%.*s %02f\r", i, s1, n-i, s2, i/(float)n);
		struct Vector camera = {0, 0, -1};
		struct Vector center = {0, 0, 0};
		struct Vector *col = from_hex("#FF0000");

		struct Sphere sp = {&center, 0.5, col};
		struct Sphere *objects = {&sp};

		struct Scene scene = {&camera, objects, WIDTH, HEIGHT};

		struct Image *im = render(scene); 

		write_ppm(im, "ok123.ppm");

		free(im);
		free(col);
		fflush(stderr);
	}
	fflush(stderr);
	return 0;
}