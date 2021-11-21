struct Image{
	int width;
	int height;
	int pixels[width][height];
}


void set_pixel(struct Image *image, int i, int j, int color){
	image[i][j] = color;
}

void write_ppm(struct Image *image){
	
}

