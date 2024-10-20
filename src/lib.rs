#![feature(once_cell)]
#![feature(const_for)]
#![feature(const_try)]
use pyo3::prelude::*;

mod builtin_types;
mod nodetree;
mod utils;

use builtin_types::all::*;
use nodetree::{get_tree, node::builtin_nodes::all::*, NodeTree};

#[pymodule]
fn ferrokrait(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NodeTree>()?;
    m.add_class::<Node>()?;
    m.add_class::<Vec2>()?;
    m.add_class::<Vec3>()?;
    m.add_class::<Vec4>()?;
    m.add_class::<Input>()?;
    m.add_class::<device_query::Keycode>()?;
    m.add_class::<Color>()?;
    m.add_function(wrap_pyfunction!(lerp_py, m)?)?;
    m.add_function(wrap_pyfunction!(clamp_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_tree, m)?)?;
    Ok(())
}
