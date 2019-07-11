use std::ops;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[inline]
    pub fn squared_length(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    #[inline]
    pub fn length(&self) -> f64 {
        return self.squared_length().sqrt();
    }
    #[inline]
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {}, {})", self.x, self.y, self.z);
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, _rhs: Vec3) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x * _rhs.x, y: self.y * _rhs.y, z: self.z * _rhs.z }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 { x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self * _rhs.x, y: self * _rhs.y, z: self * _rhs.z }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x / _rhs.x, y: self.y / _rhs.y, z: self.z / _rhs.z }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 { x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs }
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y +  v1.z * v2.z
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: -v1.x * v2.z + v1.z * v2.x,
        z: v1.x * v2.y - v1.y * v2.x
    }
}
