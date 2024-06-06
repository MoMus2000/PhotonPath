use pyo3::prelude::*;

use crate::vector;
use crate::light;
use crate::scene;
use crate::color;
use crate::ray;

use vector::Vector;
use light::Light;
use scene::*;
use color::Color;
use ray::Ray;

const DEPTH : f64 = 3.0;


#[pyclass]
pub struct Raytrace;

#[pymethods]
impl Raytrace{

    #[new]
    pub fn new()->Raytrace{
        Raytrace{}
    }

    pub fn color_at_py(&self, intersect_pos: Vector, intersect_ray_direction: Vector, light_arr: Vec<Light>, scene_arr: Vec<Scene>,
         closest_obj_index: i32, accuracy: f64, ambient: f64, depth: f64, mut final_color: Color) -> Option<Color>{
            if depth > DEPTH {
                return Some(final_color)
            }
        let scene: &Scene = match scene_arr.get(closest_obj_index as usize) {
            Some(scene_ref) => scene_ref,
            None => return None
        };
        let closest_obj_normal = scene.normal_at(&intersect_pos).clone();
        let mut closest_obj_color : Color = scene.color();

        if closest_obj_color.special == 2f64{
            let square = (intersect_pos.x.floor() + intersect_pos.z.floor()) as i32;
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
        final_color = closest_obj_color.scale(ambient).clone();
        if 0f64 < closest_obj_color.special &&
            closest_obj_color.special <= 1f64 {
            let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
            let scalar1 = &closest_obj_normal * dot;
            let add1 = &scalar1 + &intersect_ray_direction;
            let scalar2 = &add1 * 2f64;
            let add2 = &intersect_ray_direction.negative() + &scalar2;
            let reflect_direction = add2.normalize();
            let reflection_ray = Ray{origin: intersect_pos.clone(), direction: reflect_direction.clone()};
            let mut reflect_intersections= Vec::<f64>::new();
            
            for scene in &scene_arr{
                    let t = scene.intersect(&reflection_ray);
                    reflect_intersections.push(t);
            }

            let index_closest_with_reflection = closest_object_index(&reflect_intersections);


            if index_closest_with_reflection != -1{
                let acc = reflect_intersections
                    .get(index_closest_with_reflection as usize)
                    .unwrap_or_else(|| panic!("Failed to get element at index {}", index_closest_with_reflection));
                if *acc > accuracy {
                    let inner = &reflect_direction *
                    *(reflect_intersections.get(index_closest_with_reflection as usize).expect("NO"));
                    let reflect_intersection_pos = &intersect_pos + &inner;
                    let reflect_intersection_ray_direction = reflect_direction;
                    
                    let reflect_intersection_color = self.color_at_py(
                        reflect_intersection_pos.clone(), reflect_intersection_ray_direction.clone(),
                        light_arr.clone(), scene_arr.clone(),
                        index_closest_with_reflection.clone() as i32, accuracy.clone(), ambient.clone(), depth+1.0, final_color.clone());
                    

                    match reflect_intersection_color{
                        Some(res) => {
                            final_color = final_color + res.scale(closest_obj_color.special);
                        }
                        None => {
                            // println!("None")
                        }
                    }
                }
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
                
                for j in &scene_arr{
                    if shadowed{
                        break
                    }
                    else{
                        let val = j.intersect(&shadow_ray);
                        secondary_intersects.push(val);
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
                    let c = &closest_obj_color;
                    final_color = final_color.clone() + c.clone() * s1;

                    if 0f64 < closest_obj_color.special && closest_obj_color.special <= 1f64{
                        let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
                        let scalar1 = &closest_obj_normal * dot;
                        let add1 = &scalar1 + &intersect_ray_direction;
                        let scalar2 = &add1 * 2f64;
                        let add2 = &intersect_ray_direction.negative() + &scalar2;
                        let reflect_direction = add2.normalize();
                        let specular = reflect_direction.dot_product(&light_direction);
                        
                        if specular > 0f64{
                            final_color = final_color + light.color.scale(specular*closest_obj_color.special)
                        }
                    }
                }
            }
        }
        return Some(final_color.clip())
    }



}

fn closest_object_index(intersections: &Vec<f64>) -> isize {
    let mut min_index = -1;

    if intersections.is_empty() {
        return min_index;
    } else if intersections.len() == 1 {
        if intersections[0] > 0.0 {
            min_index = 0;
        }
    }
    else {
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
