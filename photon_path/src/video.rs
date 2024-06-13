use pyo3::prelude::*;

#[pyclass]
pub enum FPS{
    F10 = 10,
    F30 = 30,
    F60 = 60,
    F120 = 120
}

pub fn add_video_submodule(py: Python) -> &PyModule{
    let submodule = PyModule::new(py, "Video").expect("Expected to create image submodule");
    submodule.add_class::<FPS>().expect("Expected to add FPS class");
    submodule
}
