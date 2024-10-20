use pyo3::prelude::*;
use pyo3::types::PyType;

pub mod builtin_nodes;

#[pyclass(subclass)]
#[derive(Clone, Default, Debug)]
pub struct Node {
    #[pyo3(set, get)]
    mro: Vec<Py<PyType>>,
}

#[pymethods]
impl Node {
    #[new]
    pub fn new() -> Self {
        Self { mro: Vec::new() }
    }
    pub const fn _ready(&self) {}
    pub const fn _process(&self, _delta: f64) {}
    pub const fn _on_key_input(&self) {}
    pub fn _ready_recursive(slf: &PyCell<Self>, py: Python) -> PyResult<()> {
        for pytype in &slf.borrow().mro {
            pytype.call_method1(py, "_ready", (slf,))?;
        }
        Ok(())
    }
    pub fn _process_recursive(slf: &PyCell<Self>, py: Python, delta: f64) -> PyResult<()> {
        for pytype in &slf.borrow().mro {
            pytype.call_method1(py, "_process", (slf, delta))?;
        }
        Ok(())
    }
    pub fn _on_key_input_recursive(slf: &PyCell<Self>, py: Python) -> PyResult<()> {
        for pytype in &slf.borrow().mro {
            pytype.call_method1(py, "_on_key_input", (slf,))?;
        }
        Ok(())
    }
}
