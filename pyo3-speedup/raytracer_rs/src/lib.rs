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
    Ok(())
}
