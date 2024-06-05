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


#[pyclass]
pub struct Raytrace;

#[pymethods]
impl Raytrace{

    #[new]
    pub fn new()->Raytrace{
        Raytrace{}
    }

    pub fn color_at_py(&self, intersect_pos: Vector, intersect_ray_direction: Vector, light_arr: Vec<Light>, scene_arr: Vec<Scene>, closest_obj_index: i32, accuracy: f32, ambient: f32) -> Option<Color>{
        let mut sc = scene_arr.clone();
        let scene: &mut Scene = sc.get_mut(closest_obj_index as usize).unwrap();
        let closest_obj_normal = scene.normal_at(&intersect_pos).clone();
        let closest_obj_color : &mut Color = &mut scene.color();
        if closest_obj_color.special == 2f32{
            let square = (intersect_pos.x.floor() + intersect_pos.z.floor()) as i32;
            if square % 2 == 0{
                closest_obj_color.r = 0f32;
                closest_obj_color.g = 0f32;
                closest_obj_color.b = 0f32;
            }
            else{
                closest_obj_color.r = 1f32;
                closest_obj_color.g = 1f32;
                closest_obj_color.b = 1f32;
            }
        }
        let mut final_color = closest_obj_color.scale(ambient);
        if closest_obj_color.special < 0f32{
            let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
            let scalar1 = &closest_obj_normal * dot;
            let add1 = &scalar1 + &intersect_ray_direction;
            let scalar2 = &add1 * 2f32;
            let add2 = &intersect_ray_direction.negative() + &scalar2;
            let reflect_direction = add2.normalize();
            let reflection_ray = Ray{origin: intersect_pos.clone(), direction: reflect_direction.clone()};
            let mut reflect_intersections= Vec::<f32>::new();
            
            let sc = scene_arr.clone();
            
            for scene in &sc{
                    let t = scene.intersect(&reflection_ray);
                    reflect_intersections.push(t);
            }
            
            let index_closest_with_reflection = closest_object_index(&reflect_intersections);
            let index_closest_w_r = index_closest_with_reflection.clone();
            
            let acc = reflect_intersections.get(index_closest_w_r as usize).unwrap();
            
            if index_closest_with_reflection != -1 && acc > &accuracy {
                let inner = &reflect_direction *
                *(reflect_intersections.get(index_closest_with_reflection as usize).unwrap());
                let reflect_intersection_pos = &intersect_pos + &inner;
                let reflect_intersection_ray_direction = reflect_direction;
                
                let reflect_intersection_color = self.color_at_py(
                    reflect_intersection_pos, reflect_intersection_ray_direction,
                    light_arr.clone(), scene_arr.clone(),
                    closest_obj_index, accuracy, ambient).unwrap();
                    
                    final_color = final_color.clone() + reflect_intersection_color.scale(closest_obj_color.special);
                    
                }
                
        }
        for light in light_arr{
            let light_direction = (&light.position + &intersect_pos.negative()).normalize();
            let cos = closest_obj_normal.dot_product(&light_direction);
            
            if cos > 0f32{
                let mut shadowed = false;
                let dist_to_light = (&light.position + &intersect_pos.negative()).magnitude();
                let shadow_ray = Ray{origin: intersect_pos.clone(), direction:light_direction.clone()};
                let mut secondary_intersects = Vec::<f32>::new();
                
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
                    let c : &Color = closest_obj_color;
                    final_color = final_color.clone() + c.clone() * s1;

                    if 0f32 < closest_obj_color.special && closest_obj_color.special <= 1f32{
                        let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
                        let scalar1 = &closest_obj_normal * dot;
                        let add1 = &scalar1 + &intersect_ray_direction;
                        let scalar2 = &add1 * 2f32;
                        let add2 = &intersect_ray_direction.negative() + &scalar2;
                        let reflect_direction = add2.normalize();
                        let specular = reflect_direction.dot_product(&light_direction);
                        
                        if specular > 0f32{
                            final_color = final_color + light.color.scale(specular*closest_obj_color.special)
                        }
                    }
                }
            }
        }
        return Some(final_color.clip());
    }



}

fn closest_object_index(intersections: &Vec<f32>) -> isize {
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