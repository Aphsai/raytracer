use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
    }
}

