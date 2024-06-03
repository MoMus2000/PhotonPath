use pyo3::prelude::*;
use pyo3::types::*;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
#[pyclass]
pub struct Vector{
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub z: f32
}

#[pymethods]
impl Vector{
    #[new]
    pub fn new(x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Self{
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
            } else if let Ok(other_float) = other.extract::<f32>(py) {
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
            } else if let Ok(other_float) = other.extract::<f32>(py) {
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
            } else if let Ok(other_float) = other.extract::<f32>(py) {
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
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Dot product
    pub fn dot_product(&self, other: &Vector) -> f32 {
        (self.x * other.x as f32) + (self.y * other.y as f32) + (self.z * other.z as f32)
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

impl Mul<f32> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn mul(self, other: f32) -> Vector {
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

impl Add<f32> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn add(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
