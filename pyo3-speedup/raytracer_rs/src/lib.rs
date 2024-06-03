use pyo3::{prelude::*, wrap_pymodule};

mod color;
mod light;
mod scene;
mod vector;
mod object_parser;
mod ray;
mod raytrace;

use vector::Vector;
use color::Color;
use light::Light;
use object_parser::*;
use scene::*;
use ray::Ray;


fn color_at_py(intersect_pos: Vector, intersect_ray_direction: Vector, light_arr: &Vec<Light>, scene_arr: &Vec<Scene>, closest_obj_index: i32, accuracy: f32, ambient: f32) -> Option<Color>{
    let mut sc = scene_arr.clone();
    let scene: &mut Scene = sc.get_mut(closest_obj_index as usize).unwrap();

    if scene.triangle.is_some(){
        let closest_obj_normal = scene.triangle.as_mut().unwrap().normal_at().clone();
        let closest_obj_color : &mut Color = &mut scene.triangle.as_mut().unwrap().color;
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
        if closest_obj_color.special < 0f32 {
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

                    if 0f32 < closest_obj_color.special && closest_obj_color.special <= 1f32{
                        let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
                        let scalar1 = &closest_obj_normal * dot;
                        let add1 = &scalar1 + &intersect_ray_direction;
                        let scalar2 = &add1 * 2f32;
                        let add2 = &intersect_ray_direction.negative() + &scalar2;
                        let reflect_direction = add2.normalize();
                        let mut specular = reflect_direction.dot_product(&light_direction);
                        
                        if specular > 0f32{
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
        if closest_obj_color.special == 2f32{
            let square = (intersect_pos.x.floor() + intersect_pos.z.floor()) as i64;
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
        if closest_obj_color.special < 0f32 {
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

                    if 0f32 < closest_obj_color.special && closest_obj_color.special <= 1f32{
                        let dot = closest_obj_normal.dot_product(&intersect_ray_direction.negative());
                        let scalar1 = &closest_obj_normal * dot;
                        let add1 = &scalar1 + &intersect_ray_direction;
                        let scalar2 = &add1 * 2f32;
                        let add2 = &intersect_ray_direction.negative() + &scalar2;
                        let reflect_direction = add2.normalize();
                        let mut specular = reflect_direction.dot_product(&light_direction);
                        
                        if specular > 0f32{
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
    closest_obj_index: i32, lights: Vec<PyObject>, accuracy: f32, ambient: f32) -> PyResult<Color>{
        let (intersect_pos, intersect_ray_direction, light_arr, scene_arr) = Python::with_gil(|py|{
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
    m.add_class::<vector::Vector>()?;
    m.add_class::<scene::Triangle>()?;
    m.add_class::<color::Color>()?;
    m.add_class::<ray::Ray>()?;
    m.add_class::<scene::Plane>()?;
    m.add_class::<raytrace::Raytrace>()?;
    m.add_class::<light::Light>()?;
    m.add_class::<scene::Scene>()?;
    Ok(())
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