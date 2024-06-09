use pyo3::prelude::*;
use pyo3::types::*;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
#[pyclass]
pub struct Vector{
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
    #[pyo3(get, set)]
    pub z: f64
}

#[pymethods]
impl Vector{
    #[new]
    pub fn new(x: Option<f64>, y: Option<f64>, z: Option<f64>) -> Self{
        let mut v = Vector { x: 0.0, y: 0.0, z: 0.0 };
        if x.is_some() {
            let x_val = x.unwrap();
            v.x = x_val;
        }
        if y.is_some() {
            let y_val = y.unwrap();    
            v.y = y_val;
        }
        if z.is_some() {
            let z_val = z.unwrap();    
            v.z = z_val;
        }
        v
    }

    fn __deepcopy__(&self, _memo: &PyDict) -> Self {
        self.clone()
    }

    fn __str__(&self) -> String{
        let x = format!("x= {}", self.x);
        let y = format!("y= {}", self.y);
        let z = format!("z= {}", self.z);

        format!("{} {} {}", x, y, z)
    }

    fn __sub__(&self, other: Py<PyAny>) -> PyResult<Self> {
        Python::with_gil(|py| {
            if let Ok(other_vec) = other.extract::<Vector>(py) {
                Ok(Vector {
                    x: self.x - other_vec.x,
                    y: self.y - other_vec.y,
                    z: self.z - other_vec.z,
                })
            } else if let Ok(other_float) = other.extract::<f64>(py) {
                Ok(Vector {
                    x: self.x - other_float,
                    y: self.y - other_float,
                    z: self.z - other_float
                })
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Operand must be a Vector or a float",
                ))
            }
        })
    }

    fn __add__(&self, other: Py<PyAny>) -> PyResult<Self> {
        Python::with_gil(|py| {
            if let Ok(other_vec) = other.extract::<Vector>(py) {
                Ok(Vector {
                    x: self.x + other_vec.x,
                    y: self.y + other_vec.y,
                    z: self.z + other_vec.z,
                })
            } else if let Ok(other_float) = other.extract::<f64>(py) {
                Ok(Vector {
                    x: self.x + other_float,
                    y: self.y + other_float,
                    z: self.z + other_float
                })
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Operand must be a Vector or a float",
                ))
            }
        })
    }

    fn __mul__(&self, other: Py<PyAny>) -> PyResult<Self>{
        Python::with_gil(|py| {
            if let Ok(other_vec) = other.extract::<Vector>(py) {
                Ok(Vector {
                    x: self.x * other_vec.x,
                    y: self.y * other_vec.y,
                    z: self.z * other_vec.z,
                })
            } else if let Ok(other_float) = other.extract::<f64>(py) {
                Ok(Vector {
                    x: self.x * other_float,
                    y: self.y * other_float,
                    z: self.z * other_float
                })
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Operand must be a Vector or a float",
                ))
            }
        })
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
        Vector{x: -1.0*self.x, y: -1.0*self.y, z: -1.0*self.z}
    }

    // Magnitude calculation
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Dot product
    pub fn dot_product(&self, other: &Vector) -> f64 {
        (self.x * other.x as f64) + (self.y * other.y as f64) + (self.z * other.z as f64)
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
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_vector_addition(){
        let a = Vector{x:1.0, y:1.0, z:1.0};
        let b = Vector{x:1.0, y:1.0, z:1.0};
        let c = &a + &b;

        assert_eq!(c.x, 2.0, "Expected 2.0, but got {}", c.x);
        assert_eq!(c.y, 2.0, "Expected 2.0, but got {}", c.y);
        assert_eq!(c.z, 2.0, "Expected 2.0, but got {}", c.z);
    }

    #[test]
    fn test_vector_multiplication(){
        let a = Vector{x:1.0, y:1.0, z:1.0};
        let b = Vector{x:1.0, y:1.0, z:1.0};
        let c = &a * &b;

        assert_eq!(c.x, 1.0, "Expected 1.0, but got {}", c.x);
        assert_eq!(c.y, 1.0, "Expected 1.0, but got {}", c.y);
        assert_eq!(c.z, 1.0, "Expected 1.0, but got {}", c.z);
    }

    #[test]
    fn test_vector_subtraction(){
        let a = Vector{x:1.0, y:1.0, z:1.0};
        let b = Vector{x:1.0, y:1.0, z:1.0};
        let c = &a - &b;

        assert_eq!(c.x, 0.0, "Expected 0.0, but got {}", c.x);
        assert_eq!(c.y, 0.0, "Expected 0.0, but got {}", c.y);
        assert_eq!(c.z, 0.0, "Expected 0.0, but got {}", c.z);
    }

    #[test]
    fn test_scalar_addition(){
        let a = Vector{x:1.0, y:1.0, z:1.0};
        let b = &a + 10.0;

        assert_eq!(b.x, 11.0, "Expected 11.0, but got {}", b.x);
        assert_eq!(b.y, 11.0, "Expected 11.0, but got {}", b.y);
        assert_eq!(b.z, 11.0, "Expected 11.0, but got {}", b.z);
    }

    #[test]
    fn test_scalar_mult(){
        let a = Vector{x:5.0, y:3.0, z:2.0};
        let b = &a * 10.0;

        assert_eq!(b.x, 50.0, "Expected 50.0, but got {}", b.x);
        assert_eq!(b.y, 30.0, "Expected 30.0, but got {}", b.y);
        assert_eq!(b.z, 20.0, "Expected 20.0, but got {}", b.z);
    }

    #[test]
    fn test_negation(){
        let a = Vector{x:-5.0, y:3.0, z:-2.0};
        let b = a.negative();

        assert_eq!(b.x, 5.0, "Expected 5.0, but got {}", b.x);
        assert_eq!(b.y, -3.0, "Expected -3.0, but got {}", b.y);
        assert_eq!(b.z, 2.0, "Expected 2.0, but got {}", b.z);
    }

    #[test]
    fn test_magnitude(){
        let a = Vector{x:-5.0, y:3.0, z:-2.0};
        let magnitude = a.magnitude();

        let expected : f64 = 38.0_f64.sqrt();

        assert_eq!(expected, magnitude, "Expected {}, but got {}", expected, magnitude);
    }

    #[test]
    fn test_normalize(){
        let a = Vector{x:6.0, y:8.0, z:0.0};
        let b = a.normalize();

        assert_eq!(b.x, 0.6, "Expected {}, but got {}", b.x, 3/5);
        assert_eq!(b.y, 0.8, "Expected {}, but got {}", b.y, 4/5);
        assert_eq!(b.z, 0.0, "Expected {}, but got {}", b.z, 0.0);
    }

    #[test]
    fn test_dot_product(){
        let a = Vector{x:6.0, y:8.0, z:0.0};
        let b = Vector{x:-5.0, y:3.0, z:-2.0};
        let dot_product = a.dot_product(&b);

        assert_eq!(-6_f64, dot_product, "Expected {}, but got {}", -6_f64, dot_product);
    }

    #[test]
    fn test_cross_product(){
        let a = Vector{x:6.0, y:8.0, z:0.0};
        let b = Vector{x:-5.0, y:3.0, z:-2.0};
        let c = a.cross_product(&b);

        assert_eq!(-16_f64, c.x, "Expected {}, but got {}", -16_f64, c.x);
        assert_eq!(12_f64, c.y, "Expected {}, but got {}", 12_f64, c.y);
        assert_eq!(58_f64, c.z, "Expected {}, but got {}", 58_f64, c.z);
    }

}