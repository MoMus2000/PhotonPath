use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct Color{
    #[pyo3(get, set)]
    pub r: f64,
    #[pyo3(get, set)]
    pub g: f64,
    #[pyo3(get, set)]
    pub b: f64,
    #[pyo3(get, set)]
    pub special: f64,
}

#[pymethods]
impl Color {
    #[new]
    pub fn new(r: Option<f64>, g: Option<f64>, b: Option<f64>, special: Option<f64>) -> Self {
        let mut c = Color { r:0.5, g:0.5, b:0.5, special: 0.0};

        if r.is_some(){
            c.r = r.unwrap();
        }
        if g.is_some(){
            c.g = g.unwrap();
        }
        if b.is_some(){
            c.b = b.unwrap();
        }
        if special.is_some(){
            c.special = special.unwrap();
        }
        c
    }


    pub fn __str__(&self) -> String{
        format!("r = {} g = {} b = {} special = {}", self.r, self.g, self.b, self.special)
    }

    pub fn average(&self, other: Color) -> Color {
        Color{
            r: (self.r + other.r) / 2.0,
            g: (self.g + other.g) / 2.0,
            b: (self.b + other.b) / 2.0,
            special: self.special,
        }
    }

    pub fn brightness(&self) -> f64 {
        (self.r + self.g + self.b) / 3.0
    }

    pub fn scale(&self, scalar: f64) -> Color {
        Color{
            r: self.r * scalar, g: self.g * scalar, b: self.b * scalar, special: self.special
        }
    }

    pub fn clip(&mut self) -> Color {
        let all_light = self.r + self.g + self.b;
        let excess_light = all_light - 3.0;

        if excess_light > 0.0 {
            self.r += excess_light * (self.r + all_light);
            self.g += excess_light * (self.g + all_light);
            self.b += excess_light * (self.b + all_light);
        }

        if self.r > 1.0 {
            self.r = 1.0;
        }
        if self.r < 0.0 {
            self.r = 0.0;
        }

        if self.g > 1.0 {
            self.g = 1.0;
        }
        if self.g < 0.0 {
            self.g = 0.0;
        }

        if self.b > 1.0 {
            self.b = 1.0;
        }
        if self.b < 0.0 {
            self.b = 0.0;
        }

        Color { r: self.r, g: self.g, b: self.b, special: self.special }

    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color{
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            special: self.special,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color{
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            special: self.special,
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_average(){
        let green = Color{r: 0.0, g: 255.0, b: 0.0, special: 0.0};
        let red  = Color{r: 255.0, g: 0.0, b: 0.0, special: 0.0};
        let average = green.average(red);

        assert_eq!(127.5, average.r, "Expected {}, but got {}", 127.5, average.r);
        assert_eq!(127.5, average.g, "Expected {}, but got {}", 127.5, average.g);
        assert_eq!(0.0, average.b, "Expected {}, but got {}", 127.5, average.b);
        assert_eq!(0.0, average.special, "Expected {}, but got {}", 127.5, average.special);
    }

    #[test]
    fn test_brightness(){
        let green = Color{r: 0.0, g: 255.0, b: 0.0, special: 0.0};
        let brightness = green.brightness();
        assert_eq!(255.0/3.0, brightness, "Expected {}, but got {}", 255.0/3.0, brightness);
    }
    
    #[test]
    fn test_scale(){
        let green = Color{r: 0.0, g: 255.0, b: 0.0, special: 0.0};
        let scaled = green.scale(0.5);

        assert_eq!(0.0, scaled.r, "Expected {}, but got {}", 0.0, scaled.r);
        assert_eq!(127.5, scaled.g, "Expected {}, but got {}", 127.5, scaled.b);
        assert_eq!(0.0, scaled.b, "Expected {}, but got {}", 0.0, scaled.b);
    }

    #[test]
    fn test_clip(){
        let mut c = Color{r: 69.0, g: 255.0, b: 55.0, special: 11.0};
        let clipped = c.clip();

        assert_eq!(1.0, clipped.r, "Expected {}, but got {}", 1.0, clipped.r);
        assert_eq!(1.0, clipped.g, "Expected {}, but got {}", 1.0, clipped.g);
        assert_eq!(1.0, clipped.b, "Expected {}, but got {}", 1.0, clipped.b);
        assert_eq!(11.0, clipped.special, "Expected {}, but got {}", 11.0, clipped.special);
    }

    #[test]
    fn add_color(){
        let green = Color{r: 0.0, g: 255.0, b: 0.0, special: 0.0};
        let c = Color{r: 69.0, g: 255.0, b: 55.0, special: 11.0};

        let add = green + c;

        assert_eq!(69.0, add.r, "Expected {}, but got {}", 69.0, add.r);
        assert_eq!(510.0, add.g, "Expected {}, but got {}", 510.0, add.g);
        assert_eq!(55.0, add.b, "Expected {}, but got {}", 55.0, add.b);
    }

    #[test]
    fn multiply_color(){
        let green = Color{r: 0.0, g: 255.0, b: 0.0, special: 0.0};
        let c = Color{r: 69.0, g: 255.0, b: 55.0, special: 11.0};

        let mult = green * c;

        assert_eq!(0.0, mult.r, "Expected {}, but got {}", 0.0, mult.r);
        assert_eq!(65025.0, mult.g, "Expected {}, but got {}", 65025.0, mult.g);
        assert_eq!(0.0, mult.b, "Expected {}, but got {}", 0.0, mult.b);
    }

}
    
    
