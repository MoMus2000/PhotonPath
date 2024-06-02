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
    pub fn new(r: f64, g: f64, b: f64, special: f64) -> Self {
        Color { r, g, b, special }
    }

    pub fn average(&self, other: Color) -> Color {
        Color::new(
            (self.r + other.r) / 2.0,
            (self.g + other.g) / 2.0,
            (self.b + other.b) / 2.0,
            self.special,
        )
    }

    pub fn brightness(&self) -> f64 {
        (self.r + self.g + self.b) / 3.0
    }

    pub fn scale(&self, scalar: f64) -> Color {
        Color::new(self.r * scalar, self.g * scalar, self.b * scalar, self.special)
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
        Color::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
            self.special,
        )
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b,
            self.special,
        )
    }
}