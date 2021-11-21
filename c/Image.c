#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define WIDTH 10
#define HEIGHT 10

struct Image{
	int (*pixels)[HEIGHT];
};


void set_pixel(struct Image *image, int i, int j, int color){
	image->pixels[i][j] = color;
}

void write_ppm(struct Image *image){
	int width = WIDTH;
	int height = HEIGHT;

	FILE *ptr;

	ptr = fopen("/Users/a./Desktop/ray/ray-tracer/c/test.ppm", "w");

	if(ptr == NULL){
		exit(1);
	}

	for(int i=0;i<width;i++){
		for(int j=0;j<height;j++){
			fprintf(ptr,"%d",image->pixels[i][j]);
		}
	}
	fclose(ptr);
}

int main(int argc, char const *argv[]){
	int pixels[WIDTH][HEIGHT] = {{1}};
	struct Image *img = malloc(sizeof(struct Image));
	img->pixels = pixels;
	write_ppm(img);
	// struct Image = {co, 5, pixels}
}

