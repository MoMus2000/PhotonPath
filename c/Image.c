#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "test.h"


#define WIDTH 2
#define HEIGHT 4

struct Image{
	struct Vector pixels[WIDTH][HEIGHT];
};

void set_pixel(struct Image *image, int i, int j, struct Vector color){
	image->pixels[i][j] = color;
}

float float_rand( float min, float max ){
    float scale = rand() / (float) RAND_MAX; /* [0, 1.0] */
    return min + scale * ( max - min );      /* [min, max] */
}

int to_byte(float c){
	if (c*255 > 255){
		return round(255);
	}

	else if (c*255 < 0){
		return round(0);
	}
	return round(c);
}

void write_ppm(struct Image *image){
	int width = WIDTH;
	int height = HEIGHT;

	FILE *ptr;

	ptr = fopen("/Users/a./Desktop/ray/ray-tracer/c/test.ppm", "w");

	if(ptr == NULL){
		exit(1);
	}

	fprintf(ptr,"P3 %d %d \n255\n", width, height);

	for(int i=0;i<width;i++){
		for(int j=0;j<height;j++){
			struct Vector color = image->pixels[i][j];
			int x = to_byte(color.x);
			int y = to_byte(color.y);
			int z = to_byte(color.z);
			fprintf(ptr,"%d %d %d ", x, y , z);
		}
		fprintf(ptr,"\n");
	}
	fclose(ptr);
}

int main(int argc, char const *argv[]){
	// int pixels[WIDTH][HEIGHT] = {{1, 2, 3, 4}, {5, 6, 7, 8}};
	struct Vector v1 = {2,2,2};
	struct Vector v2 = {2,3,4};
	struct Vector arr[WIDTH][HEIGHT] = {v1, v2}; 
	struct Vector pixels[WIDTH][HEIGHT] = {v1, v2};
	struct Image *img = malloc(sizeof(struct Image));
	for(int i=0;i<WIDTH; i++){
		for(int j=0;j<HEIGHT; j++){
			img->pixels[i][j] = pixels[i][j];
		}
	}
	// img->pixels[0][0] = pixels[0][0];
	write_ppm(img);
	// struct Image = {co, 5, pixels}
}

