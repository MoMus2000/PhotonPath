use pyo3::prelude::*;

use crate::vector::Vector;
use crate::color::Color;
use crate::light::Light;
use crate::scene::*;

pub fn _parse_vector(py_vec: PyObject, py: &Python) -> Vector{
    let object_ref: &PyAny = py_vec.extract(*py).unwrap();
    Vector{
        x: object_ref.getattr("x").unwrap().extract::<f64>().unwrap(),
        y: object_ref.getattr("y").unwrap().extract::<f64>().unwrap(),
        z: object_ref.getattr("z").unwrap().extract::<f64>().unwrap(),
    }
}

pub fn _parse_light(py_light: &PyObject, py: &Python) -> Light{
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

fn _parse_triangle(py_object: &PyObject, py: &Python) -> Triangle{
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

fn _parse_plane(py_object: &PyObject, py: &Python) -> Plane{
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


pub fn _parse_scene(py_object: &PyObject, py: &Python) -> Scene{
    let obtype =  py_object.as_ref(*py).get_type().name().unwrap();
    let mut scene = Scene{triangle: None, plane: None, sphere: None};

    if obtype == "Triangle"{
        scene.triangle = Some(_parse_triangle(py_object, py));
    }
    else if obtype == "Plane" {
        scene.plane = Some(_parse_plane(py_object, py));
    } 

    scene
}