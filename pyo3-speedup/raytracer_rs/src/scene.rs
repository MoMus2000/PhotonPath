use pyo3::prelude::*;
use crate::color::Color;
use crate::vector::Vector;
use crate::ray::Ray;

#[derive(Clone)]
#[pyclass]
pub struct Triangle{
    #[pyo3(get, set)]
    pub a: Vector,
    #[pyo3(get, set)]
    pub b: Vector,
    #[pyo3(get, set)]
    pub c: Vector,
    #[pyo3(get, set)]
    pub ca: Vector,
    #[pyo3(get, set)]
    pub ba: Vector,
    #[pyo3(get, set)]
    pub normal: Vector,
    #[pyo3(get, set)]
    pub distance: f64,
    #[pyo3(get, set)]
    pub color: Color
}


#[pymethods]
impl Triangle{
    #[new]
    pub fn new() -> Triangle{
        let a = Vector{x:1.00, y:0.0, z:0.0}.clone();
        let b = Vector{x:0.00, y:1.0, z:0.0}.clone();
        let c = Vector{x:0.00, y:0.0, z:1.0}.clone();
        let color = Color::new(1.0, 1.0, 1.0, 0.0).clone();

        let ca = &c - &a;
        let ba = &b - &a;

        let normal = ca.cross_product(&ba).normalize();

        let distance = normal.dot_product(&a);

        Triangle{
            a,
            b,
            c,
            ca,
            ba,
            normal,
            distance,
            color
        }

    }

    pub fn normal_at(&self) -> Vector{
        self.normal.clone()
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        let dot = ray.direction.dot_product(&self.normal);

        if dot == 0.0 {
            return -1.0;
        } else {
            let dum = &self.normal * self.distance;
            let dum = dum.negative();
            let dum = &ray.origin + &dum;
            let dummy = self.normal.dot_product(&dum);
            let dist_to_triangle = -1.0 * dummy / dot;

            let dum2 = &ray.direction * dist_to_triangle;
            let q = &dum2 + &ray.origin;

            let ca = &self.c - &self.a;
            let qa = &q - &self.a;

            let bc = &self.b - &self.c;
            let qc = &q - &self.c;

            let ab = &self.a - &self.b;
            let qb = &q - &self.b;

            let inside = ca.cross_product(&qa).dot_product(&self.normal) >= 0.0 &&
                         bc.cross_product(&qc).dot_product(&self.normal) >= 0.0 &&
                         ab.cross_product(&qb).dot_product(&self.normal) >= 0.0;

            if inside {
                return dist_to_triangle;
            } else {
                return -1.0;
            }
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Plane{
    #[pyo3(get, set)]
    pub normal: Vector,
    #[pyo3(get, set)]
    pub distance: f64,
    #[pyo3(get, set)]
    pub color: Color
}

#[pymethods]
impl Plane{
    pub fn normal_at(&self) -> Vector{
        self.normal.clone()
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        let dot = ray.direction.dot_product(&self.normal);
        if dot == 0.0 {
            -1.0
        } else {
            let inner = &(&self.normal * self.distance).negative();
            let outer = &ray.origin + inner;
            let dummy = self.normal.dot_product(&outer);
            -dummy / dot
        }
    }
}

#[derive(Clone)]
pub struct Scene{
    pub triangle: Option<Triangle>,
    pub plane: Option<Plane>
}
