use pyo3::prelude::*;
use crate::vector::Vector;
use crate::color::Color;

#[derive(Debug, Clone)]
#[pyclass]
pub struct Light{
    pub position: Vector,
    pub color: Color
}

#[pymethods]
impl Light{
    #[new]
    pub fn new(position: Vector, color: Color) -> Light{
        Light{position, color}
    }
}