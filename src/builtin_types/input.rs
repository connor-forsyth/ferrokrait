use crate::builtin_types::vec2::Vec2;
use device_query::{DeviceQuery, DeviceState, Keycode};
use pyo3::prelude::*;
use std::cell::LazyCell;

static mut QUERY_HANDLER: LazyCell<DeviceState> = LazyCell::new(DeviceState::new);

#[pyclass]
pub struct Input;

#[pymethods]
impl Input {
    #[staticmethod]
    pub unsafe fn get_key(key: Keycode) -> i8 {
        if key == Keycode::Any {
            return !QUERY_HANDLER.get_keys().is_empty() as i8;
        }
        QUERY_HANDLER.get_keys().contains(&key) as i8
    }
    #[staticmethod]
    pub unsafe fn all_keys() -> Vec<Keycode> {
        QUERY_HANDLER.get_keys()
    }
    #[staticmethod]
    pub unsafe fn mouse_pos() -> Vec2 {
        let t = QUERY_HANDLER.get_mouse().coords;
        Vec2 {
            x: t.0 as f64,
            y: t.1 as f64,
        }
    }
    #[staticmethod]
    pub unsafe fn relative_mouse_pos(origin: Vec2) -> Vec2 {
        Self::mouse_pos() - origin
    }
    #[staticmethod]
    pub unsafe fn get_vector(up: Keycode, down: Keycode, left: Keycode, right: Keycode) -> Vec2 {
        Vec2 {
            x: (Input::get_key(right) as i32 - Input::get_key(left) as i32) as f64,
            y: (Input::get_key(up) as i32 - Input::get_key(down) as i32) as f64,
        }
        .normalised()
    }
    #[staticmethod]
    pub unsafe fn get_action(action: Action) -> bool {
        let t: Vec<Keycode> = QUERY_HANDLER.get_keys();
        for item in action.0.iter() {
            if t.binary_search(item).is_err() {
                return false;
            }
        }
        true
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Action(Vec<Keycode>);

#[pymethods]
impl Action {
    #[new]
    pub fn new(keys: Vec<Keycode>) -> Self {
        Self(keys)
    }
}
