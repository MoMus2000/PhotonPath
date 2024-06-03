use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct Color{
    #[pyo3(get, set)]
    pub r: f32,
    #[pyo3(get, set)]
    pub g: f32,
    #[pyo3(get, set)]
    pub b: f32,
    #[pyo3(get, set)]
    pub special: f32,
}

#[pymethods]
impl Color {
    #[new]
    pub fn new(r: Option<f32>, g: Option<f32>, b: Option<f32>, special: Option<f32>) -> Self {
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

    pub fn brightness(&self) -> f32 {
        (self.r + self.g + self.b) / 3.0
    }

    pub fn scale(&self, scalar: f32) -> Color {
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