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
    pub distance: f32,
    #[pyo3(get, set)]
    pub color: Color
}


#[pymethods]
impl Triangle{
    #[new]
    pub fn new(a_: Option<Vector>, b_: Option<Vector>, c_: Option<Vector>, color_: Option<Color>) -> Triangle{

        let a = Vector{x:1.00, y:0.0, z:0.0}.clone();
        let b = Vector{x:0.00, y:1.0, z:0.0}.clone();
        let c = Vector{x:0.00, y:0.0, z:1.0}.clone();
        let color = Color{r: 1.0, g: 1.0, b:1.0, special:0.0}.clone();

        let ca = &c - &a;
        let ba = &b - &a;

        let normal = ca.cross_product(&ba).normalize();

        let distance = normal.dot_product(&a);

        let mut t = Triangle{
                                    a,
                                    b,
                                    c,
                                    ca,
                                    ba,
                                    normal,
                                    distance,
                                    color
                                };

        if a_.is_some(){
            t.a = a_.unwrap()
        }
        else if b_.is_some(){
            t.b = b_.unwrap()
        }
        else if c_.is_some(){
            t.c = c_.unwrap()
        }
        else if color_.is_some(){
            t.color = color_.unwrap()
        }


        t.ca = &t.c - &t.a;
        t.ba = &t.b - &t.a;

        t.normal = t.ca.cross_product(&t.ba).normalize();

        t.distance = t.normal.dot_product(&t.a);

        t
    }

    pub fn normal_at(&self) -> Vector{
        self.normal.clone()
    }

    pub fn intersect(&self, ray: &Ray) -> f32 {
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
    pub distance: f32,
    #[pyo3(get, set)]
    pub color: Color
}

#[pymethods]
impl Plane{
    #[new]
    pub fn new(normal: Option<Vector>, distance: Option<f32>, color: Option<Color>) -> Self{
        let  norm = Vector::new(None, None, None);
        let  dist = 0.0;
        let colo = Color{r: 0.5, g: 0.5, b:0.5, special:0.0}.clone();

        let mut p = Plane{
            normal: norm,
            distance: dist,
            color: colo
        };

        if normal.is_some(){
            p.normal = normal.unwrap();
        }
        if distance.is_some(){
            p.distance = distance.unwrap();
        }
        if color.is_some(){
            p.color = color.unwrap();
        }

        p
    }

    pub fn normal_at(&self) -> Vector{
        self.normal.clone()
    }

    pub fn intersect(&self, ray: &Ray) -> f32 {
        let dot = ray.direction.dot_product(&self.normal);
        if dot == 0.0 {
            -1.0
        } else {
            let inner = &(&self.normal * self.distance).negative();
            let outer = &ray.origin + inner;
            let dummy = self.normal.dot_product(&outer);
            -1.0*dummy / dot
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Scene{
    pub triangle: Option<Triangle>,
    pub plane: Option<Plane>
}

#[pymethods]
impl Scene{
    #[new]
    pub fn new(triangle: Option<Triangle>, plane: Option<Plane>) -> Scene{
        Scene { triangle, plane }
    }

    pub fn intersect(&self,ray: &Ray) -> f32{
        if self.triangle.is_some(){
            self.triangle.as_ref().unwrap().intersect(ray)
        }
        else{
            self.plane.as_ref().unwrap().intersect(ray)
        }
    }
}