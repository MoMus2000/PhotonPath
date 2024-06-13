use pyo3::prelude::*;

mod color;
mod light;
mod scene;
mod vector;
mod object_parser;
mod ray;
mod raytrace;
mod sphere;
mod camera;
mod render_scene;
mod rt_image;


/// A Python module implemented in Rust.
#[pymodule]
fn raytracer_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<vector::Vector>()?;
    m.add_class::<scene::Triangle>()?;
    m.add_class::<color::Color>()?;
    m.add_class::<ray::Ray>()?;
    m.add_class::<scene::Plane>()?;
    m.add_class::<raytrace::Raytrace>()?;
    m.add_class::<light::Light>()?;
    m.add_class::<scene::Scene>()?;
    m.add_class::<sphere::Sphere>()?;
    m.add_class::<camera::Camera>()?;
    m.add_class::<render_scene::RenderScene>()?;

    let image_sub_module = rt_image::add_image_submodule(_py);
    m.add_submodule(image_sub_module)?;

    // Set to call like from raytracer_rs.image import VLR, and so on ...
    _py.import("sys")?
        .getattr("modules")?
        .set_item("raytracer_rs.image", image_sub_module)?;

    Ok(())
}