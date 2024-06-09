use image::{ImageBuffer, Rgb};

pub struct Image{
    pub file_name: String,
    pub img_vec: Vec<(u8, u8, u8)>,
    pub width: u32,
    pub height: u32
}

impl Image{
    pub fn new(file_name: String, img_vec: Vec<(u8, u8, u8)>, width: u32, height: u32) -> Image{
        Image{file_name, img_vec, width, height}
    }

    pub fn convert_to_image(&self){
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.width, self.height);
        let mut count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let color = self.img_vec[count];
                let color = Rgb([color.0, color.1, color.2]);
                count += 1;
                img.put_pixel(x, self.height - y - 1, color); // Assuming `Rgb` constructor accepts a tuple (r, g, b)
            }
        }
        img.save(self.file_name.clone()).unwrap();
    }
}

#[cfg(test)]
mod test{
    use image::GenericImageView;
    use image::io::Reader as ImageReader;
    use image::DynamicImage;

    use super::*;
    use std::fs::remove_file;

    #[test]
    fn test_image_creation(){
        let width = 1200 as u32;  // Example width
        let height = 1200 as u32; // Example height
        
        let mut img_vec: Vec<(u8, u8, u8)> = Vec::with_capacity(height as usize *width as usize);

        for _ in 0..height*width {
            img_vec.push((255, 180, 69));
        }

        let file_path = "./output.png";

        let img = Image::new(file_path.to_string(), img_vec.clone(), width, height);

        img.convert_to_image();

        let img: DynamicImage = ImageReader::open(file_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

        let dimensions = img.dimensions();

        assert_eq!(width, img.dimensions().0, "Expected width {} got {}", width, dimensions.0);
        assert_eq!(height, img.dimensions().1, "Expected height {} got {}", height, dimensions.1);

        for (_, _, pixel) in img.to_rgb8().enumerate_pixels(){
            let image::Rgb(data) = pixel;
            let (r, g, b) = (data[0], data[1], data[2]);

            assert_eq!(r, data[0], "Expected r = {}, got {}", r, data[0]);
            assert_eq!(g, data[1], "Expected g = {}, got {}", g, data[1]);
            assert_eq!(b, data[2], "Expected b = {}, got {}", b, data[2]);

        }

        remove_file(file_path).expect("Should have removed the created file");
    }

}