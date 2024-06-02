use crate::vector::Vector;
use crate::color::Color;

#[derive(Debug)]
pub struct Light{
    pub position: Vector,
    pub color: Color
}