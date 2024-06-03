use pyo3::prelude::*;
use crate::vector::Vector;

#[pyclass]
#[derive(Debug)]
pub struct Ray{
    #[pyo3(get, set)]
    pub origin: Vector,
    #[pyo3(get, set)]
    pub direction: Vector
}

#[pymethods]
impl Ray{
    #[new]
    fn new(origin: Vector, direction: Vector) -> Self{
        Ray { origin, direction }
    }
}