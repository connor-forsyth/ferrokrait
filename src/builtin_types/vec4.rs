use super::all::{Clamp, Lerp, Max, Min};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Vec4 {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
    #[pyo3(get, set)]
    pub z: f64,
    #[pyo3(get, set)]
    pub w: f64,
}

#[pymethods]
impl Vec4 {
    #[classattr]
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    #[classattr]
    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };
    #[classattr]
    pub const DOWN: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
        w: 0.0,
    };
    #[classattr]
    pub const LEFT: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    #[classattr]
    pub const RIGHT: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    #[classattr]
    pub const Z_FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 0.0,
    };
    #[classattr]
    pub const Z_BACK: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
        w: 0.0,
    };
    #[classattr]
    pub const W_FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    #[classattr]
    pub const W_BACK: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: -1.0,
    };

    #[new]
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
    pub fn normalised(&self) -> Self {
        if self == &Vec4::new(0.0, 0.0, 0.0, 0.0) {
            return *self;
        }
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    #[pyo3(signature=(ndigits=1.0))]
    pub fn __round__(&self, ndigits: f64) -> Self {
        Self {
            x: (self.x * 10_f64.powf(ndigits)).round() / 10_f64.powf(ndigits),
            y: (self.y * 10_f64.powf(ndigits)).round() / 10_f64.powf(ndigits),
            z: (self.z * 10_f64.powf(ndigits)).round() / 10_f64.powf(ndigits),
            w: (self.w * 10_f64.powf(ndigits)).round() / 10_f64.powf(ndigits),
        }
    }
    pub fn __add__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            return Ok(*self + rhs);
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __iadd__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            *self += rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __sub__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            return Ok(*self - rhs);
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __isub__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            *self -= rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __mul__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            return Ok(*self * rhs);
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            return Ok(*self * rhs);
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __imul__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            *self *= rhs;
            return Ok(());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            *self *= rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __truediv__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            return Ok(*self / rhs);
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            return Ok(*self / rhs);
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __itruediv__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            *self /= rhs;
            return Ok(());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            *self /= rhs;
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec3 can be added to Vec4"))
    }
    pub fn __floordiv__(&self, rhs: PyObject, py: Python) -> PyResult<Self> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            return Ok((*self / rhs).floor());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            return Ok((*self / rhs).floor());
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
    }
    pub fn __ifloordiv__(&mut self, rhs: PyObject, py: Python) -> PyResult<()> {
        if let Ok(rhs) = rhs.extract::<Vec4>(py) {
            *self /= rhs;
            *self = self.floor();
            return Ok(());
        } else if let Ok(rhs) = rhs.extract::<f64>(py) {
            *self /= rhs;
            *self = self.floor();
            return Ok(());
        }
        Err(PyValueError::new_err("Only type Vec4 can be added to Vec4"))
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
            z: self.z.floor(),
            w: self.w.floor(),
        }
    }
    pub fn __repr__(&self) -> String {
        format!("Vec4({0}, {1}, {2}, {3})", self.x, self.y, self.z, self.w)
    }
}

impl Max for Vec4 {
    fn max(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
            w: self.w.max(rhs.w),
        }
    }
}

impl Min for Vec4 {
    fn min(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
            w: self.w.min(rhs.w),
        }
    }
}

impl std::ops::Add for Vec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}
impl std::ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
impl std::ops::Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl std::ops::Mul for Vec4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl std::ops::Mul<f64> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl std::ops::Div for Vec4 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}

impl std::ops::DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl std::ops::Div<f64> for Vec4 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Lerp for &Vec4 {
    type Output = Vec4;
    fn lerp(self, rhs: Self, lerp: f64) -> Self::Output {
        Vec4 {
            x: self.x.lerp(rhs.x, lerp),
            y: self.y.lerp(rhs.y, lerp),
            z: self.z.lerp(rhs.z, lerp),
            w: self.w.lerp(rhs.w, lerp),
        }
    }
}
