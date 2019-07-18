use crate::geometry::{ Vec3 };

#[derive(Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}
