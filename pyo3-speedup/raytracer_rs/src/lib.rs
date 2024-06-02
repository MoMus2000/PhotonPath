use pyo3::prelude::*;
use std::ops::{Add, Sub, Mul};


#[derive(Debug, Clone)]
struct Vector{
    x: f64,
    y: f64,
    z: f64
}

impl Vector{
    // Normalization
    fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn negative(&self) -> Vector{
        let magnitude = self.magnitude();
        Vector{x: self.x/magnitude, y: self.y/magnitude, z: self.z/magnitude}
    }

    // Magnitude calculation
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Dot product
    fn dot_product(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product
    fn cross_product(&self, other: &Vector) -> Vector {
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

impl Mul<f64> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn mul(self, other: f64) -> Vector {
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

impl Add<f64> for &Vector {
    type Output = Vector;

    // Implementing the subtraction operation for references to Vector
    fn add(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Color{
    #[pyo3(get, set)]
    r: f64,
    #[pyo3(get, set)]
    g: f64,
    #[pyo3(get, set)]
    b: f64,
    #[pyo3(get, set)]
    special: f64,
}

#[pymethods]
impl Color {
    #[new]
    fn new(r: f64, g: f64, b: f64, special: f64) -> Self {
        Color { r, g, b, special }
    }

    fn average(&self, other: Color) -> Color {
        Color::new(
            (self.r + other.r) / 2.0,
            (self.g + other.g) / 2.0,
            (self.b + other.b) / 2.0,
            self.special,
        )
    }

    fn brightness(&self) -> f64 {
        (self.r + self.g + self.b) / 3.0
    }

    fn scale(&self, scalar: f64) -> Color {
        Color::new(self.r * scalar, self.g * scalar, self.b * scalar, self.special)
    }

    fn clip(&mut self) -> Color {
        let all_light = self.r + self.g + self.b;
        let excess_light = all_light - 3.0;

        if excess_light > 0.0 {
            self.r += excess_light * (self.r + all_light);
            self.g += excess_light * (self.g + all_light);
            self.b += excess_light * (self.b + all_light);
        }

        if self.r > 1.0 {
            self.r = 1.0;
        }
        if self.r < 0.0 {
            self.r = 0.0;
        }

        if self.g > 1.0 {
            self.g = 1.0;
        }
        if self.g < 0.0 {
            self.g = 0.0;
        }

        if self.b > 1.0 {
            self.b = 1.0;
        }
        if self.b < 0.0 {
            self.b = 0.0;
        }

        Color { r: self.r, g: self.g, b: self.b, special: self.special }

    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
            self.special,
        )
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b,
            self.special,
        )
    }
}

#[derive(Debug)]
struct Light{
    position: Vector,
    color: Color
}

fn parse_vector(py_vec: PyObject, py: &Python) -> Vector{
    let object_ref: &PyAny = py_vec.extract(*py).unwrap();
    Vector{
        x: object_ref.getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("y").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("z").unwrap().extract::<f64>().unwrap(),
    }
}

fn parse_light(py_light: &PyObject, py: &Python) -> Light{
    let object_ref: &PyAny = py_light.extract(*py).unwrap();
    let v = Vector{
        x: object_ref.getattr("position").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("position").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("position").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
    };
    let c = Color{
        r: object_ref.getattr("color").unwrap().getattr("r").unwrap().extract::<f64>().unwrap(),
        g: object_ref.getattr("color").unwrap().getattr("g").unwrap().extract::<f64>().unwrap(),
        b: object_ref.getattr("color").unwrap().getattr("b").unwrap().extract::<f64>().unwrap(),
        special: object_ref.getattr("color").unwrap().getattr("special").unwrap().extract::<f64>().unwrap(),
    };
    Light{
        position: v,
        color: c
    }
}

#[derive(Clone)]
struct Triangle{
    a: Vector,
    b: Vector,
    c: Vector,
    ca: Vector,
    ba: Vector,
    normal: Vector,
    distance: f64,
    color: Color
}

impl Triangle{
    fn normal_at(&self) -> &Vector{
        &self.normal
    }

    fn intersect(&self, ray: &Ray) -> f64 {
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
struct Plane{
    normal: Vector,
    distance: f64,
    color: Color
}

impl Plane{
    fn normal_at(&self) -> &Vector{
        &self.normal
    }

    fn intersect(&self, ray: &Ray) -> f64 {
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
struct Scene{
    triangle: Option<Triangle>,
    plane: Option<Plane>
}

fn parse_triangle(py_object: &PyObject, py: &Python) -> Triangle{
    let object_ref: &PyAny = py_object.extract(*py).unwrap();
    let a = Vector{
        x: object_ref.getattr("a").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("a").unwrap().getattr("y").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("a").unwrap().getattr("z").unwrap().extract::<f64>().unwrap(),
    };
    let b = Vector{
        x: object_ref.getattr("b").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("b").unwrap().getattr("y").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("b").unwrap().getattr("z").unwrap().extract::<f64>().unwrap(),
    };
    let c = Vector{
        x: object_ref.getattr("c").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("c").unwrap().getattr("y").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("c").unwrap().getattr("z").unwrap().extract::<f64>().unwrap(),
    };

    let ca = &c - &a;
    let ba = &b - &a;

    let normal = Vector{
        x: object_ref.getattr("normal").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("normal").unwrap().getattr("y").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("normal").unwrap().getattr("z").unwrap().extract::<f64>().unwrap(),
    };

    let distance = object_ref.getattr("distance").unwrap().extract::<f64>().unwrap();

    let color = Color{
        r: object_ref.getattr("color").unwrap().getattr("r").unwrap().extract::<f64>().unwrap(),
        g: object_ref.getattr("color").unwrap().getattr("g").unwrap().extract::<f64>().unwrap(),
        b: object_ref.getattr("color").unwrap().getattr("b").unwrap().extract::<f64>().unwrap(),
        special: object_ref.getattr("color").unwrap().getattr("special").unwrap().extract::<f64>().unwrap(),
    };

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

fn parse_plane(py_object: &PyObject, py: &Python) -> Plane{
    let object_ref: &PyAny = py_object.extract(*py).unwrap();
    let normal = Vector{
        x: object_ref.getattr("normal").unwrap().getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("normal").unwrap().getattr("y").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("normal").unwrap().getattr("z").unwrap().extract::<f64>().unwrap(),
    };

    let distance = object_ref.getattr("distance").unwrap().extract::<f64>().unwrap();

    let color = Color{
        r: object_ref.getattr("color").unwrap().getattr("r").unwrap().extract::<f64>().unwrap(),
        g: object_ref.getattr("color").unwrap().getattr("g").unwrap().extract::<f64>().unwrap(),
        b: object_ref.getattr("color").unwrap().getattr("b").unwrap().extract::<f64>().unwrap(),
        special: object_ref.getattr("color").unwrap().getattr("special").unwrap().extract::<f64>().unwrap(),
    };

    Plane{
        normal,
        distance,
        color
    }
}

fn parse_scene(py_object: &PyObject, py: &Python) -> Scene{
    let obtype =  py_object.as_ref(*py).get_type().name().unwrap();
    let mut scene = Scene{triangle: None, plane: None};

    if obtype == "Triangle"{
        scene.triangle = Some(parse_triangle(py_object, py));
    }
    else if obtype == "Plane" {
        scene.plane = Some(parse_plane(py_object, py));
    } 

    scene
}

struct Ray{
    origin: Vector,
    direction: Vector
}

fn color_at_py(intersect_pos: Vector, intersect_ray_direction: Vector, light_arr: &Vec<Light>, mut scene_arr: &Vec<Scene>, closest_obj_index: i32, accuracy: f64, ambient: f64) -> Option<Color>{
    let mut sc = scene_arr.clone();
    let scene: &mut Scene = sc.get_mut(closest_obj_index as usize).unwrap();

    if scene.triangle.is_some(){
        let closest_obj_normal = scene.triangle.as_mut().unwrap().normal_at().clone();
        let closest_obj_color : &mut Color = &mut scene.triangle.as_mut().unwrap().color;
        if closest_obj_color.special == 2f64{
            let square = (intersect_pos.x.floor() + intersect_pos.z.floor()) as i64;
            if square % 2 == 0{
                closest_obj_color.r = 0f64;
                closest_obj_color.g = 0f64;
                closest_obj_color.b = 0f64;
            }
            else{
                closest_obj_color.r = 1f64;
                closest_obj_color.g = 1f64;
                closest_obj_color.b = 1f64;
            }
        }
        let mut final_color = closest_obj_color.scale(ambient);
        if closest_obj_color.special < 0f64 {
            let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
            let scalar1 = &closest_obj_normal * dot;
            let add1 = &scalar1 + &intersect_ray_direction;
            let scalar2 = &add1 * 2f64;
            let add2 = &intersect_ray_direction.negative() + &scalar2;
            let reflect_direction = add2.normalize();
            let reflection_ray = Ray{origin: intersect_pos.clone(), direction: reflect_direction.clone()};
            let mut reflect_intersections= Vec::<f64>::new();
            
            let sc = scene_arr.clone();
            
            for scene in &sc{
                if scene.triangle.is_some(){
                    let t = scene.triangle.as_ref().unwrap().intersect(&reflection_ray);
                    reflect_intersections.push(t);
                }
            }
            
            let index_closest_with_reflection = closest_object_index(&reflect_intersections);
            let index_closest_w_r = index_closest_with_reflection.clone();
            
            let acc = reflect_intersections.get(index_closest_w_r as usize).unwrap();
            
            if index_closest_with_reflection > -1 && acc > &accuracy {
                let inner = &reflect_direction *
                *(reflect_intersections.get(index_closest_with_reflection as usize).unwrap());
                let reflect_intersection_pos = &intersect_pos + &inner;
                let reflect_intersection_ray_direction = reflect_direction;
                
                let reflect_intersection_color = color_at_py(
                    reflect_intersection_pos, reflect_intersection_ray_direction,
                    &light_arr, &scene_arr,
                    closest_obj_index, accuracy, ambient).unwrap();
                    
                    let final_color = final_color.clone() + reflect_intersection_color.scale(closest_obj_color.special);
                    
                }
                
        }
        for light in light_arr{
            let light_direction = (&light.position + &intersect_pos.negative()).normalize();
            let cos = closest_obj_normal.dot_product(&light_direction);
            
            if cos > 0f64{
                let mut shadowed = false;
                let dist_to_light = (&light.position + &intersect_pos.negative()).magnitude();
                let shadow_ray = Ray{origin: intersect_pos.clone(), direction:light_direction.clone()};
                let mut secondary_intersects = Vec::<f64>::new();
                
                for j in scene_arr{
                    if shadowed{
                        break
                    }
                    else{
                        if j.triangle.is_some(){
                            let val = j.triangle.as_ref().unwrap().intersect(&shadow_ray);
                            secondary_intersects.push(val);
                        }
                    }
                }
                for j in secondary_intersects{
                    let pred1 = accuracy < j;
                    let pred2 = j <= dist_to_light;
                    if pred1 && pred2{
                        shadowed = true;
                    }
                }
                if !shadowed{
                    let s1 = light.color.scale(cos);
                    let c : &Color = closest_obj_color;
                    final_color = final_color.clone() + c.clone();
                    final_color = final_color * s1;

                    if 0f64 < closest_obj_color.special && closest_obj_color.special <= 1f64{
                        let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
                        let scalar1 = &closest_obj_normal * dot;
                        let add1 = &scalar1 + &intersect_ray_direction;
                        let scalar2 = &add1 * 2f64;
                        let add2 = &intersect_ray_direction.negative() + &scalar2;
                        let reflect_direction = add2.normalize();
                        let mut specular = reflect_direction.dot_product(&light_direction);
                        
                        if specular > 0f64{
                            specular = specular.powi(10);
                            final_color = final_color + light.color.scale(specular*closest_obj_color.special)
                        }
                    }
                }
            }
            
        }

        // panic!("");
        return Some(final_color.clip());
    }
    else if scene.plane.is_some(){
        let closest_obj_normal = scene.plane.as_mut().unwrap().normal_at().clone();
        let closest_obj_color : &mut Color = &mut scene.plane.as_mut().unwrap().color;
        if closest_obj_color.special == 2f64{
            let square = (intersect_pos.x.floor() + intersect_pos.z.floor()) as i64;
            if square % 2 == 0{
                closest_obj_color.r = 0f64;
                closest_obj_color.g = 0f64;
                closest_obj_color.b = 0f64;
            }
            else{
                closest_obj_color.r = 1f64;
                closest_obj_color.g = 1f64;
                closest_obj_color.b = 1f64;
            }
        }
        let mut final_color = closest_obj_color.scale(ambient);
        if closest_obj_color.special < 0f64 {
            let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
            let scalar1 = &closest_obj_normal * dot;
            let add1 = &scalar1 + &intersect_ray_direction;
            let scalar2 = &add1 * 2f64;
            let add2 = &intersect_ray_direction.negative() + &scalar2;
            let reflect_direction = add2.normalize();
            let reflection_ray = Ray{origin: intersect_pos.clone(), direction: reflect_direction.clone()};
            let mut reflect_intersections= Vec::<f64>::new();
            
            let sc = scene_arr.clone();
            
            for scene in &sc{
                if scene.plane.is_some(){
                    let t = scene.plane.as_ref().unwrap().intersect(&reflection_ray);
                    reflect_intersections.push(t);
                }
            }
            
            let index_closest_with_reflection = closest_object_index(&reflect_intersections);
            let index_closest_w_r = index_closest_with_reflection.clone();
            
            let acc = reflect_intersections.get(index_closest_w_r as usize).unwrap();
            
            if index_closest_with_reflection > -1 && acc > &accuracy {
                let inner = &reflect_direction *
                *(reflect_intersections.get(index_closest_with_reflection as usize).unwrap());
                let reflect_intersection_pos = &intersect_pos + &inner;
                let reflect_intersection_ray_direction = reflect_direction;
                
                let reflect_intersection_color = color_at_py(
                    reflect_intersection_pos, reflect_intersection_ray_direction,
                    &light_arr, &scene_arr,
                    closest_obj_index, accuracy, ambient).unwrap();
                    
                    let final_color = final_color.clone() + reflect_intersection_color.scale(closest_obj_color.special);
                    
                }
                
        }
        for light in light_arr{
            let light_direction = (&light.position + &intersect_pos.negative()).normalize();
            let cos = closest_obj_normal.dot_product(&light_direction);
            
            if cos > 0f64{
                let mut shadowed = false;
                let dist_to_light = (&light.position + &intersect_pos.negative()).magnitude();
                let shadow_ray = Ray{origin: intersect_pos.clone(), direction:light_direction.clone()};
                let mut secondary_intersects = Vec::<f64>::new();
                
                for j in scene_arr{
                    if shadowed{
                        break
                    }
                    else{
                        if j.plane.is_some(){
                            let val = j.plane.as_ref().unwrap().intersect(&shadow_ray);
                            secondary_intersects.push(val);
                        }
                    }
                }
                for j in secondary_intersects{
                    let pred1 = accuracy < j;
                    let pred2 = j <= dist_to_light;
                    if pred1 && pred2{
                        shadowed = true;
                    }
                }
                if !shadowed{
                    let s1 = light.color.scale(cos);
                    let c : &Color = closest_obj_color;
                    final_color = final_color.clone() + c.clone();
                    final_color = final_color * s1;

                    if 0f64 < closest_obj_color.special && closest_obj_color.special <= 1f64{
                        let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
                        let scalar1 = &closest_obj_normal * dot;
                        let add1 = &scalar1 + &intersect_ray_direction;
                        let scalar2 = &add1 * 2f64;
                        let add2 = &intersect_ray_direction.negative() + &scalar2;
                        let reflect_direction = add2.normalize();
                        let mut specular = reflect_direction.dot_product(&light_direction);
                        
                        if specular > 0f64{
                            specular = specular.powi(10);
                            final_color = final_color + light.color.scale(specular*closest_obj_color.special)
                        }
                    }
                }
            }
            
        }

        // panic!("");
        return Some(final_color.clip());
    }
    None
}

#[pyfunction]
fn color_at(intersect_pos: PyObject, intersect_ray_direction: PyObject, scene_objects: Vec<PyObject>,
    closest_obj_index: i32, lights: Vec<PyObject>, accuracy: f64, ambient: f64) -> PyResult<Color>{
        let (intersect_pos, intersect_ray_direction, light_arr, mut scene_arr) = Python::with_gil(|py|{
            let object_ref: &PyAny = intersect_pos.extract(py).unwrap();
            
            let intersect_pos = parse_vector(intersect_pos, &py);
            let intersect_ray_direction = parse_vector(intersect_ray_direction, &py);
            let mut light_arr = Vec::<Light>::new();
            let mut scene_arr = Vec::<Scene>::new();

            for arr in lights{
                let res = parse_light(&arr, &py);
                light_arr.push(res);
            }

            for arr in scene_objects{
                let res = parse_scene(&arr, &py);
                scene_arr.push(res);
            }

            (intersect_pos, intersect_ray_direction, light_arr, scene_arr)
        });

        let out = color_at_py(intersect_pos, intersect_ray_direction, &light_arr, &scene_arr, closest_obj_index, accuracy, ambient);

        if out.is_none(){
        }
        else{
        }

        Ok(out.unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn raytracer_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(color_at, m)?)?;
    Ok(())
}

fn closest_object_index(intersections: &Vec<f64>) -> isize {
    let mut min_index = -1;

    if intersections.is_empty() {
        return min_index;
    } else if intersections.len() == 1 {
        if intersections[0] > 0.0 {
            min_index = 0;
        }
    } else {
        let mut max_val = 0.0;
        for i in 0..intersections.len() {
            if max_val < intersections[i] {
                max_val = intersections[i];
            }
        }

        if max_val > 0.0 {
            for i in 0..intersections.len() {
                if 0.0 < intersections[i] && intersections[i] <= max_val {
                    max_val = intersections[i];
                    min_index = i as isize;
                }
            }
        }
    }

    min_index
}