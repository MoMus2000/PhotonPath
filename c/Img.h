#include <png.h>
#define WIDTH 720
#define HEIGHT 480

struct Image{
	struct Vector pixels[HEIGHT][WIDTH];
};

void set_pixel(struct Image *image, int i, int j, struct Vector color){
	image->pixels[j][i] = color;
}

int to_byte(float c){
	if (c*255 > 255){
		return round(255);
	}

	else if (c*255 < 0){
		return round(0);
	}
	return round(c*255);
}

void write_ppm(struct Image *image, char * file_name){
	int width = WIDTH;
	int height = HEIGHT;

	FILE *ptr;

	char path[100] = "/Users/a./Desktop/ray/ray-tracer/c/";
	strcat(path, file_name);

	ptr = fopen(path, "w");

	if(ptr == NULL){
		exit(1);
	}

	fprintf(ptr,"P3 %d %d\n255\n", width, height);

	for(int i=0;i<height;i++){
		for(int j=0;j<width;j++){
			struct Vector color = image->pixels[i][j];
			int x = to_byte(color.x);
			int y = to_byte(color.y);
			int z = to_byte(color.z);

			fprintf(ptr,"%d %d %d ", x, y , z);
		}
		fprintf(ptr,"\n");
	}
	fclose(ptr);

	// for(int i=0;i<width;i++){
	// 	for(int j=0;j<height;j++){
	// 		struct Vector color = image->pixels[i][j];
	// 		int x = to_byte(color.x);
	// 		int y = to_byte(color.y);
	// 		int z = to_byte(color.z);
	// 		printf("%d %d %d\n", x, y, z);
	// 	}
	// 	// fprintf(ptr,"\n");
	// }
}
