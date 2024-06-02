
use crate::vector::Vector;
use crate::color::Color;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Triangle{
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,
    pub ca: Vector,
    pub ba: Vector,
    pub normal: Vector,
    pub distance: f64,
    pub color: Color
}

impl Triangle{
    pub fn normal_at(&self) -> &Vector{
        &self.normal
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
pub struct Plane{
    pub normal: Vector,
    pub distance: f64,
    pub color: Color
}

impl Plane{
    pub fn normal_at(&self) -> &Vector{
        &self.normal
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
