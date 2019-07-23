#![allow(dead_code)]
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
    pub fn normalize(&mut self) -> Vec3 {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        return *self;
    }

    #[inline]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * 2.0 * dot(*self, *normal)
    }

    pub fn refract(&self, normal: &Vec3, incident_refractive_index: f64, outgoing_refractive_index: f64) -> Vec3 {
        let cos_i = -((dot(*self, *normal).min(1.0)).max(-1.0));

        if cos_i < 0.0 { return self.refract(&(-*normal), outgoing_refractive_index, incident_refractive_index); }

        let ratio = incident_refractive_index / outgoing_refractive_index;
        let k = 1.0 - ratio * ratio * (1.0 - cos_i * cos_i);
        if k < 0.0 { return Vec3 { x: 1.0, y: 0.0, z: 0.0 }; } else { return *self * ratio + *normal * (ratio * cos_i - k.sqrt()); }
    }


    pub fn new() -> Vec3 { Vec3 { x: 0.0, y: 0.0, z: 0.0 } }
    pub fn unit() -> Vec3 { Vec3 { x: 1.0, y: 1.0, z: 1.0 } }
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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
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
