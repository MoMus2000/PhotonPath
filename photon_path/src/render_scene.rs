use pyo3::prelude::*;
use std::path::Path;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::light::Light;
use crate::ray::Ray;
use crate::raytrace::{Raytrace, self};
use crate::color::Color;
use crate::rt_image::Image;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};

const AA_DEPTH: i32 = 1;

#[derive(Clone)]
#[pyclass]
pub struct RenderScene{
    pub scene_objects: Vec<Scene>,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub width: f64,
    pub height: f64,
    pub ambient: f64,
    pub accuracy: f64
}

#[pymethods]
impl RenderScene{
    #[new]
    pub fn new(scene_objects: Vec<Scene>, camera: Camera, lights: Vec<Light>, width: i32, height: i32, ambient: Option<f64>, accuracy: Option<f64>) -> RenderScene{
        let accuracy = accuracy.unwrap_or(0.00000001);
        let ambient = ambient.unwrap_or(0.1);
        let width = width as f64;
        let height = height as f64;
        RenderScene{
            scene_objects,
            camera,
            lights,
            width,
            height,
            ambient,
            accuracy
        }
    }

    pub fn singular_par_render(&self) -> Vec<(u8, u8, u8)> {
        let result: Arc<Mutex<Vec<(u8, u8, u8)>>> = Arc::new(Mutex::new(Vec::new()));
        let aspect_ratio = self.width / self.height;

        (0..self.width as i32).into_par_iter().for_each(|i|{
            (0..self.height as i32).into_par_iter().for_each(|j|{
                let size = AA_DEPTH * AA_DEPTH;
                let mut temp_red : Vec<f64>= Vec::with_capacity(size as usize);
                temp_red.resize(size as usize, 0.0);
                let mut temp_green : Vec<f64>= Vec::with_capacity(size as usize);
                temp_green.resize(size as usize, 0.0);
                let mut temp_blue : Vec<f64>= Vec::with_capacity(size as usize);
                temp_blue.resize(size as usize, 0.0);

                for aax in 0..AA_DEPTH {
                    for aay in 0..AA_DEPTH {
                        let aa_index = aay * AA_DEPTH + aax;

                        let (x, y) = (i as f64, j as f64); // example values
                        let (xamnt, yamnt);
            
                        if AA_DEPTH == 1 {
                            if self.width > self.height {
                                xamnt = ((x + 0.5) / self.width) * aspect_ratio - (((self.width - self.height) / self.height) / 2.0);
                                yamnt = ((self.height - y) + 0.5) / self.height;
                            } else if self.height > self.width {
                                xamnt = (x + 0.5) / self.width;
                                yamnt = (((self.height - y) + 0.5) / self.height) / aspect_ratio - (((self.height - self.width) / self.width) / 2.0);
                            } else {
                                xamnt = (x + 0.5) / self.width;
                                yamnt = ((self.height - y) + 0.5) / self.height;
                            }
                        } else {
                            if self.width > self.height {
                                xamnt = ((x + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.width) * aspect_ratio -
                                        (((self.width - self.height) / self.height) / 2.0);
                                yamnt = ((self.height - y) + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.height;
                            } else if self.height > self.width {
                                xamnt = (x + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.width;
                                yamnt = (((self.height - y) + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.height) / aspect_ratio -
                                        (((self.height - self.width) / self.width) / 2.0);
                            } else {
                                xamnt = (x + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.width;
                                yamnt = ((self.height - y) + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.height;
                            }
                        }

                        let camera = self.camera.clone();

                        let cam_ray_origin = camera.position;

                        let p0 = xamnt-0.5;
                        let p1 = yamnt-0.5;

                        let p2 = &camera.right * p0;
                        let p3 = &camera.down * p1;

                        let cam_ray_direction = (&camera.direction + &(&p2 + &p3)).normalize();

                        let cam_ray = Ray{origin: cam_ray_origin.clone(), direction: cam_ray_direction.clone()};

                        let mut intersections: Vec<f64> = Vec::new();

                        for scene in &self.scene_objects{
                            intersections.push(scene.intersect(&cam_ray));
                        }

                        let closest_obj_index = raytrace::closest_object_index(&intersections);

                        if closest_obj_index == -1{
                            temp_red[aa_index as usize] = 0.0;
                            temp_blue[aa_index as usize] = 0.0;
                            temp_green[aa_index as usize] = 0.0;
                        }
                        else{
                            if intersections[closest_obj_index as usize] > self.accuracy{
                                let intersect_pos = &cam_ray_origin + &(&cam_ray_direction * intersections[closest_obj_index as usize]);
                                let intersect_ray_direction = cam_ray_direction.clone();
                                let colorz= Color{r: 0.5, g: 0.25, b: 0.25, special: 0.1};
                                let rt = Raytrace{};
                                let color = rt.color_at_py(
                                    intersect_pos,
                                    intersect_ray_direction,
                                    self.lights.clone(), self.scene_objects.clone(),
                                    closest_obj_index as i32,
                                    self.accuracy,
                                    self.ambient,
                                    0.0,
                                    colorz,
                                ).unwrap();

                                temp_red[aa_index as usize] = color.r;
                                temp_green[aa_index as usize] = color.g;
                                temp_blue[aa_index as usize] = color.b;

                            }
                        }
                    }
                }

                let avg_red = temp_red.iter().sum::<f64>() / (AA_DEPTH * AA_DEPTH) as f64;
                let avg_blue = temp_blue.iter().sum::<f64>() / (AA_DEPTH * AA_DEPTH) as f64;
                let avg_green = temp_green.iter().sum::<f64>() / (AA_DEPTH * AA_DEPTH) as f64;

                let mut result_guard = result.lock().unwrap();

                result_guard.push((
                    (avg_red * 255.0).round() as u8,
                    (avg_green * 255.0).round() as u8,
                    (avg_blue * 255.0).round() as u8,
                ));
            });
        });
        Arc::try_unwrap(result).unwrap().into_inner().unwrap()
    }

    pub fn render(&self) -> Vec<(u8, u8, u8)>{
        let mut result : Vec<(u8, u8, u8)> = Vec::new();

        let aspect_ratio = self.width / self.height;

        for i in 0..self.width as i32{
            for j in 0..self.height as i32{
                let size = AA_DEPTH * AA_DEPTH;
                let mut temp_red : Vec<f64>= Vec::with_capacity(size as usize);
                temp_red.resize(size as usize, 0.0);
                let mut temp_green : Vec<f64>= Vec::with_capacity(size as usize);
                temp_green.resize(size as usize, 0.0);
                let mut temp_blue : Vec<f64>= Vec::with_capacity(size as usize);
                temp_blue.resize(size as usize, 0.0);

                for aax in 0..AA_DEPTH {
                    for aay in 0..AA_DEPTH {
                        let aa_index = aay * AA_DEPTH + aax;

                        let (x, y) = (i as f64, j as f64); // example values
                        let (xamnt, yamnt);
            
                        if AA_DEPTH == 1 {
                            if self.width > self.height {
                                xamnt = ((x + 0.5) / self.width) * aspect_ratio - (((self.width - self.height) / self.height) / 2.0);
                                yamnt = ((self.height - y) + 0.5) / self.height;
                            } else if self.height > self.width {
                                xamnt = (x + 0.5) / self.width;
                                yamnt = (((self.height - y) + 0.5) / self.height) / aspect_ratio - (((self.height - self.width) / self.width) / 2.0);
                            } else {
                                xamnt = (x + 0.5) / self.width;
                                yamnt = ((self.height - y) + 0.5) / self.height;
                            }
                        } else {
                            if self.width > self.height {
                                xamnt = ((x + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.width) * aspect_ratio -
                                        (((self.width - self.height) / self.height) / 2.0);
                                yamnt = ((self.height - y) + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.height;
                            } else if self.height > self.width {
                                xamnt = (x + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.width;
                                yamnt = (((self.height - y) + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.height) / aspect_ratio -
                                        (((self.height - self.width) / self.width) / 2.0);
                            } else {
                                xamnt = (x + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.width;
                                yamnt = ((self.height - y) + aax as f64 / (AA_DEPTH as f64 - 1.0)) / self.height;
                            }
                        }

                        let camera = self.camera.clone();

                        let cam_ray_origin = camera.position;

                        let p0 = xamnt-0.5;
                        let p1 = yamnt-0.5;

                        let p2 = &camera.right * p0;
                        let p3 = &camera.down * p1;

                        let cam_ray_direction = (&camera.direction + &(&p2 + &p3)).normalize();

                        let cam_ray = Ray{origin: cam_ray_origin.clone(), direction: cam_ray_direction.clone()};

                        let mut intersections: Vec<f64> = Vec::new();

                        for scene in &self.scene_objects{
                            intersections.push(scene.intersect(&cam_ray));
                        }

                        let closest_obj_index = raytrace::closest_object_index(&intersections);

                        if closest_obj_index == -1{
                            temp_red[aa_index as usize] = 0.0;
                            temp_blue[aa_index as usize] = 0.0;
                            temp_green[aa_index as usize] = 0.0;
                        }
                        else{
                            if intersections[closest_obj_index as usize] > self.accuracy{
                                let intersect_pos = &cam_ray_origin + &(&cam_ray_direction * intersections[closest_obj_index as usize]);
                                let intersect_ray_direction = cam_ray_direction.clone();
                                let colorz= Color{r: 0.5, g: 0.25, b: 0.25, special: 0.1};
                                let rt = Raytrace{};
                                let color = rt.color_at_py(
                                    intersect_pos,
                                    intersect_ray_direction,
                                    self.lights.clone(), self.scene_objects.clone(),
                                    closest_obj_index as i32,
                                    self.accuracy,
                                    self.ambient,
                                    0.0,
                                    colorz,
                                ).unwrap();

                                temp_red[aa_index as usize] = color.r;
                                temp_green[aa_index as usize] = color.g;
                                temp_blue[aa_index as usize] = color.b;

                            }
                        }
                    }
                }

                let avg_red = temp_red.iter().sum::<f64>() / (AA_DEPTH * AA_DEPTH) as f64;
                let avg_blue = temp_blue.iter().sum::<f64>() / (AA_DEPTH * AA_DEPTH) as f64;
                let avg_green = temp_green.iter().sum::<f64>() / (AA_DEPTH * AA_DEPTH) as f64;

                result.push((
                    (avg_red * 255.0).round() as u8,
                    (avg_green * 255.0).round() as u8,
                    (avg_blue * 255.0).round() as u8,
                ));

            }
        }
        result
    }

    #[staticmethod]
    // Do polling, generator pattern
    pub fn par_render(render_scenes: Vec<RenderScene>, width: i32, height: i32, folder_path: &str) {

        if !is_valid_path(folder_path){
            panic!("Invalid folder path");
        }

        let pb = ProgressBar::new(render_scenes.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {wide_bar} {percent}%").unwrap()
        .progress_chars("█▉▊▋▌▍▎▏  "));

        (0 .. render_scenes.len()).into_par_iter().for_each(|i|{
            let img_vec = render_scenes[i].singular_par_render();
            let file_name = format!("{}/images/rs_{}.jpeg",folder_path, i);
            let img = Image::new(file_name, img_vec, width as u32, height as u32);
            img.convert_to_image();
            pb.inc(1);
        });

    }

    #[staticmethod]
    pub fn par_render_image(render_scene: RenderScene, width: i32, height: i32, file_name: &str){
        let img_vec = render_scene.singular_par_render();
        if is_valid_path(file_name){
            let img = Image::new(file_name.to_string(), img_vec, width as u32, height as u32);
            img.convert_to_image();
        }
    }
}

fn is_valid_path(path_str: &str) -> bool {
    let path = Path::new(path_str);
    let parent = path.parent(); // Get the parent directory
    if let Some(parent_dir) = parent {
        parent_dir.exists() && parent_dir.is_dir()
    } else {
        false  // No parent directory found
    }
}