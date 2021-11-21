#include <stdio.h>
#include "Vec.h"
#include "Img.h"

int main(int argc, char const *argv[])
{
	struct Vector red = {1,0,0};
	struct Vector blue = {0,1,0};
	struct Vector green = {0,0,1};
	struct Image *img = malloc(sizeof(struct Image));
	
	set_pixel(img, 0, 0, red);
	set_pixel(img, 1, 0, red);
	set_pixel(img, 2, 0, red);

	set_pixel(img, 0, 1, *add(red, green));
	set_pixel(img, 1, 1, *add(red, blue));
	set_pixel(img, 2, 1, *add(blue, green));

	set_pixel(img, 0, 2, *static_mul(red, 0.01));
	set_pixel(img, 1, 2, *static_mul(blue, 2));
	set_pixel(img, 2, 2, *add(blue, *add(red, green)));

	write_ppm(img);
	return 0;
}