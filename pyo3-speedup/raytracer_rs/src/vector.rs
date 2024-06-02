use pyo3::prelude::*;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
#[pyclass]
pub struct Vector{
    pub x: f64,
    pub y: f64,
    pub z: f64
}


#[pymethods]
impl Vector{
    #[new]
    pub fn new() -> Self{
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn negative(&self) -> Vector{
        let magnitude = self.magnitude();
        Vector{x: self.x/magnitude, y: self.y/magnitude, z: self.z/magnitude}
    }

    // Magnitude calculation
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Dot product
    pub fn dot_product(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product
    pub fn cross_product(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    // Implementing the subtraction operation for Vector
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<&Vector> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn mul(self, other: &Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn add(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
