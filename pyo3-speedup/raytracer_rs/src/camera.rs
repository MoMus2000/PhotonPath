use pyo3::prelude::*;
use crate::vector::Vector;


#[pyclass]
pub struct Camera{
    #[pyo3(get, set)]
    position: Vector,
    #[pyo3(get, set)]
    direction: Vector,
    #[pyo3(get, set)]
    right: Vector,
    #[pyo3(get, set)]
    down: Vector
}

#[pymethods]
impl Camera{
    #[new]
    pub fn new(position: Option<Vector>, direction: Option<Vector>,
         right: Option<Vector>, down: Option<Vector>) -> Self{
        let mut cam = Camera{
            position: Vector{
                x: 0.0, y:0.0, z:0.0
            },
            direction: Vector{
                x: 0.0, y:0.0, z:1.0
            },
            right: Vector{
                x: 0.0, y:0.0, z:0.0
            },
            down: Vector{
                x: 0.0, y:0.0, z:0.0
            },
        };

        if position.is_some(){
            cam.position = position.unwrap();
        }

        if direction.is_some(){
            cam.direction = direction.unwrap();
        }

        if right.is_some(){
            cam.right = right.unwrap();
        }

        if down.is_some(){
            cam.down = down.unwrap();
        }

        cam
    }
}