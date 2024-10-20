#![allow(dead_code)]
use pyo3::{prelude::*, types::PyType, AsPyPointer, PyTypeInfo};

// Gets the python type of an object
pub fn get_pytype<'a, T: AsPyPointer>(obj: &'a T, py: Python<'a>) -> &'a PyType {
    unsafe {
        let t = *obj.as_ptr();
        PyType::from_type_ptr(py, t.ob_type)
    }
}

// Checks if an object is a child of another class
// e.g: if is_child_class::<Node>(py_object) { do_thing() }
pub fn is_subclass<T: PyTypeInfo>(py: Python, class_object: &PyObject) -> bool {
	get_pytype(class_object, py).is_subclass_of::<T>().unwrap()
}

pub trait SubclassCheck {
	fn get_pytype<'a>(&self, py: Python<'a>) -> &'a PyType;
	fn is_subclass<A: PyTypeInfo>(&self, py: Python) -> PyResult<bool>;
}

impl<T: AsPyPointer> SubclassCheck for T {
	fn get_pytype<'a>(&self, py: Python<'a>) -> &'a PyType {
	    unsafe {
	        let t = *self.as_ptr();
	        PyType::from_type_ptr(py, t.ob_type)
	    }
	}
	fn is_subclass<A: PyTypeInfo>(&self, py: Python) -> PyResult<bool> {
		self.get_pytype(py).is_subclass_of::<A>()
	}
}
