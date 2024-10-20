use pyo3::prelude::*;

use super::all::*;
use crate::builtin_types::all::*;

#[pyclass(subclass, extends = Node)]
#[derive(Default, Debug, Clone)]
pub struct Node2D {
    position: Vec2,
    rotation_rad: f64,
}

#[pymethods]
impl Node2D {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Node::new()).add_subclass(Self {
            position: Vec2::ZERO,
            rotation_rad: 0.0,
        })
    }
    pub fn test(slf: PyRef<Self>) {
        let _t = slf.as_ref();
    }
}
