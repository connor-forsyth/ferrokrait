use super::all::{Clamp, Lerp, Max, Min};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vec2 {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
}

#[pymethods]
impl Vec2 {
    #[classattr]
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    #[classattr]
    pub const UP: Self = Self { x: 0.0, y: 1.0 };
    #[classattr]
    pub const DOWN: Self = Self { x: 0.0, y: -1.0 };
    #[classattr]
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    #[classattr]
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    #[new]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn normalised(&self) -> Self {
        if self == &Vec2::new(0.0, 0.0) {
            return *self;
        }
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
        }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn rotated(&self, rotation: f64) -> Self {
        let theta = rotation.to_radians();
        Self {
            x: self.x * theta.cos() - self.y * theta.sin(),
            y: self.x * theta.sin() + self.y * theta.cos(),
        }
    }
    #[pyo3(signature=(ndigits=1.0))]
    pub fn __round__(&self, ndigits: f64) -> Self {
        Self {
            x: (self.x * 10_f64.powf(ndigits)).round() / 10_f64.powf(ndigits),
            y: (self.y * 10_f64.powf(ndigits)).round() / 10_f64.powf(ndigits),
        }
    }
    pub fn __add__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            return Ok(*self + rhs);
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __iadd__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            *self += rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __sub__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            return Ok(*self - rhs);
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __isub__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            *self -= rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __mul__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            return Ok(*self * rhs);
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            return Ok(*self * rhs);
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __imul__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            *self *= rhs;
            return Ok(());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            *self *= rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __truediv__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            return Ok(*self / rhs);
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            return Ok(*self / rhs);
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __itruediv__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            *self /= rhs;
            return Ok(());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            *self /= rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __floordiv__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            return Ok((*self / rhs).floor());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            return Ok((*self / rhs).floor());
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
    }
    pub fn __ifloordiv__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec2>(py) {
            *self /= rhs;
            *self = self.floor();
            return Ok(());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            *self /= rhs;
            *self = self.floor();
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec2 can be added to Vec2"))
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
    #[pyo3(name = "lerp")]
    pub fn lerp_py(&self, rhs: &Self, lerp: f64) -> Self {
        self.lerp(rhs, lerp)
    }
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
    pub fn __repr__(&self) -> String {
        format!("Vec2({0}, {1})", self.x, self.y)
    }
}

impl Max for Vec2 {
    fn max(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }
}

impl Min for Vec2 {
    fn min(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl std::ops::DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Lerp for &Vec2 {
    type Output = Vec2;
    fn lerp(self, rhs: Self, lerp: f64) -> Self::Output {
        Vec2 {
            x: self.x.lerp(rhs.x, lerp),
            y: self.y.lerp(rhs.y, lerp),
        }
    }
}
