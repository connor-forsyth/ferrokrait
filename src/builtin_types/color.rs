use super::all::{Clamp, Lerp, Max, Min, Vec4};
use pyo3::prelude::*;

#[pyclass]
pub struct Color {
    #[pyo3(get, set)]
    pub r: u8,
    #[pyo3(get, set)]
    pub g: u8,
    #[pyo3(get, set)]
    pub b: u8,
    #[pyo3(get, set)]
    pub a: u8,
}

impl Min for Color {
    fn min(&self, rhs: &Self) -> Self {
        Self {
            r: self.r.min(rhs.r),
            g: self.g.min(rhs.g),
            b: self.b.min(rhs.b),
            a: self.a.min(rhs.a),
        }
    }
}

impl Max for Color {
    fn max(&self, rhs: &Self) -> Self {
        Self {
            r: self.r.max(rhs.r),
            g: self.g.max(rhs.g),
            b: self.b.max(rhs.b),
            a: self.a.max(rhs.a),
        }
    }
}

impl Lerp<Self, u8> for &Color {
    type Output = Color;
    fn lerp(self, rhs: Self, lerp: u8) -> Self::Output {
        Color {
            r: self.r.lerp(rhs.r, lerp),
            g: self.g.lerp(rhs.g, lerp),
            b: self.b.lerp(rhs.b, lerp),
            a: self.a.lerp(rhs.a, lerp),
        }
    }
}

#[pymethods]
impl Color {
    #[classattr]
    pub const TRANSPARENT: Self = Self::new(255, 255, 255, 0);
    #[classattr]
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    #[classattr]
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    #[classattr]
    pub const RED: Self = Self::new(255, 0, 0, 255);
    #[classattr]
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    #[classattr]
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    #[classattr]
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);
    #[classattr]
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    #[classattr]
    pub const CYAN: Self = Self::new(0, 255, 255, 255);

    #[new]
    pub const fn new(r: u8, b: u8, g: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const fn inverted(&self) -> Self {
        Self {
            r: self.a - self.r,
            g: self.a - self.g,
            b: self.a - self.b,
            a: self.a,
        }
    }
    pub const fn brighten(&self, delta: u8) -> Self {
        Self {
            r: self.r + delta,
            g: self.g + delta,
            b: self.b + delta,
            a: self.a,
        }
    }
    pub const fn dim(&self, delta: u8) -> Self {
        Self {
            r: self.r - delta,
            g: self.g - delta,
            b: self.b - delta,
            a: self.a,
        }
    }
    #[pyo3(name = "clamp")]
    pub fn clamp_py(&self, min: &Self, max: &Self) -> Self {
        self.clamp(min, max)
    }
    #[pyo3(name = "min")]
    pub fn min_py(&self, rhs: &Self) -> Self {
        self.min(rhs)
    }
    #[pyo3(name = "max")]
    pub fn max_py(&self, rhs: &Self) -> Self {
        self.max(rhs)
    }
    #[staticmethod]
    pub fn from_vec4(vec: Vec4) -> Self {
        Self {
            r: vec.x as u8,
            g: vec.y as u8,
            b: vec.z as u8,
            a: vec.w as u8,
        }
    }
    pub fn to_vec4(&self) -> Vec4 {
        Vec4 {
            x: self.r as f64,
            y: self.g as f64,
            z: self.b as f64,
            w: self.a as f64,
        }
    }
}
