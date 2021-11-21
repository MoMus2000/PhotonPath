#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define WIDTH 200
#define HEIGHT 400

struct Image{
	int (*pixels)[HEIGHT];
};

void set_pixel(struct Image *image, int i, int j, int color){
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

	printf("HELLO");

	fprintf(ptr,"P3 %d %d \n255\n", width, height);

	for(int i=0;i<width;i++){
		for(int j=0;j<height;j++){
			// int color = image->pixels[i][j];
			int color = float_rand(0.001, 0.009);
			int x = to_byte(color);
			fprintf(ptr,"%d", x);
		}
		fprintf(ptr,"\n");
	}
	fclose(ptr);
}

int main(int argc, char const *argv[]){
	int pixels[WIDTH][HEIGHT] = {{1, 2, 3, 4}, {5, 6, 7, 8}};
	struct Image *img = malloc(sizeof(struct Image));
	img->pixels = pixels;
	write_ppm(img);
	// struct Image = {co, 5, pixels}
}

