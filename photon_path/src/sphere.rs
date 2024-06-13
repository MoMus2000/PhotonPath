use pyo3::prelude::*;

use crate::{color::Color, vector::Vector, ray::Ray};

#[derive(Debug, Clone)]
#[pyclass]
pub struct Sphere{
    #[pyo3(get, set)]
    pub center: Vector,
    #[pyo3(get, set)]
    pub radius: f64,
    #[pyo3(get, set)]
    pub color: Color
}

#[pymethods]
impl Sphere{
    #[new]
    pub fn new(center: Vector, radius: f64, color: Color) -> Sphere{
        Sphere { center, radius, color }
    }

    pub fn normal_at(&self, point: &Vector) -> Vector{
        (point + &self.center.negative()).normalize().clone()
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        let ray_origin = &ray.origin;
        let ray_direction = &ray.direction;

        let a = 1.0;
        let b = 2.0 * ((ray_origin.x - self.center.x) * ray_direction.x
            + (ray_origin.y - self.center.y) * ray_direction.y
            + (ray_origin.z - self.center.z) * ray_direction.z);
        let c = (ray_origin.x - self.center.x).powi(2)
            + (ray_origin.y - self.center.y).powi(2)
            + (ray_origin.z - self.center.z).powi(2)
            - self.radius.powi(2);

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let first_root = (-1.0 * b - discriminant.sqrt()) / 2.0  - 0.000001;
            if first_root > 0.0 {
                return first_root;
            } else {
                let second_root = (-1.0 * b + discriminant.sqrt()) / 2.0 - 0.000001;
                return second_root;
            }
        } else {
            -1.0
        }
    }
}