use pyo3::prelude::*;
use image::{ImageBuffer, Rgb, DynamicImage, ImageFormat};


pub const VLR: (i32, i32) = (240, 144);
pub const LR: (i32, i32) = (480, 270);
pub const NHD: (i32, i32) = (640, 360);
pub const SD: (i32, i32) = (640, 480);
pub const HD: (i32, i32) = (1280, 720);
pub const FHD: (i32, i32) = (1920, 1080);
pub const QHD: (i32, i32) = (2560, 1440);
pub const UHD: (i32, i32) = (3840, 2160);

pub fn add_image_submodule(py: Python) -> &PyModule{
    let submodule = PyModule::new(py, "Image").expect("Expected to create image submodule");
    submodule.add("VLR", VLR).expect("Expected to add constant");
    submodule.add("LR", LR).expect("Expected to add constant");
    submodule.add("NHD", NHD).expect("Expected to add constant");
    submodule.add("SD", SD).expect("Expected to add constant");
    submodule.add("HD", HD).expect("Expected to add constant");
    submodule.add("FHD", FHD).expect("Expected to add constant");
    submodule.add("QHD", QHD).expect("Expected to add constant");
    submodule.add("UHD", UHD).expect("Expected to add constant");
    submodule
}

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

        let img_dynamic = DynamicImage::ImageRgb8(img);
        img_dynamic.save_with_format(self.file_name.clone(), ImageFormat::Jpeg).unwrap();
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

        let file_path = "./output.jpeg";

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