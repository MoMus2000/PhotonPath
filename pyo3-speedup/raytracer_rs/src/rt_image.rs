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
    use super::*;
    #[test]
    fn test_image_creation(){
        let width = 1200 as u32;  // Example width
        let height = 1200 as u32; // Example height
        
        let mut img_vec: Vec<(u8, u8, u8)> = Vec::with_capacity(height as usize *width as usize);

        for _ in 0..height*width {
            img_vec.push((0, 0, 0));
        }

        let img = Image::new("./output.png".to_string(), img_vec, width, height);

        img.convert_to_image();
    }

}