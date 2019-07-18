use crate::geometry::{ Vec3 };

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse_color: Vec3,
    pub albedo: Vec3,
    pub specular_exponent: f64
}

pub const RED_RUBBER : Material = Material {
    diffuse_color: Vec3 { x: 0.3, y: 0.1, z: 0.1 },
    albedo: Vec3 { x: 0.9, y: 0.1, z: 0.0 },
    specular_exponent: 10.0
};

pub const IVORY : Material = Material {
    diffuse_color: Vec3 { x: 0.4, y: 0.4, z: 0.3 },
    albedo: Vec3 { x: 0.6, y: 0.3, z: 0.1 },
    specular_exponent: 50.0
};

pub const MIRROR : Material = Material {
    diffuse_color: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
    albedo: Vec3 { x: 0.0, y: 10.0, z: 0.8 },
    specular_exponent : 1425.0
};
