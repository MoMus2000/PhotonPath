use pyo3::prelude::*;
use crate::vector::Vector;

#[pyclass]
pub struct Ray{
    pub origin: Vector,
    pub direction: Vector
}