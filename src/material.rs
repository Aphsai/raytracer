use crate::geometry::{ Vec3 };

#[derive(Clone, Copy)]
pub struct Albedo {
    pub diffusion_coeff : f64,
    pub specularity_coeff : f64,
    pub reflection_coeff : f64,
    pub refraction_coeff : f64,
}

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse_color: Vec3,
    pub albedo: Albedo,
    pub specular_exponent: f64,
    pub refractive_index: f64,
}

pub const RED_RUBBER : Material = Material {
    diffuse_color: Vec3 { x: 0.3, y: 0.1, z: 0.1 },
    albedo: Albedo { diffusion_coeff: 0.9, specularity_coeff: 0.1, reflection_coeff: 0.0, refraction_coeff: 0.0 },
    specular_exponent: 10.0,
    refractive_index : 1.0,
};

pub const IVORY : Material = Material {
    diffuse_color: Vec3 { x: 0.4, y: 0.4, z: 0.3 },
    albedo: Albedo { diffusion_coeff: 0.6, specularity_coeff: 0.3, reflection_coeff: 0.1, refraction_coeff: 0.0 },
    specular_exponent: 50.0,
    refractive_index : 1.0,
};

pub const MIRROR : Material = Material {
    diffuse_color: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
    albedo: Albedo { diffusion_coeff: 0.0, specularity_coeff: 10.0, reflection_coeff: 0.8, refraction_coeff: 0.0 },
    specular_exponent : 1425.0,
    refractive_index : 1.0,
};

pub const GLASS : Material = Material {
    diffuse_color : Vec3 { x: 0.6, y: 0.7, z: 0.8 },
    albedo: Albedo { diffusion_coeff: 0.0, specularity_coeff: 0.5, reflection_coeff: 0.1, refraction_coeff: 0.8 },
    specular_exponent : 10.0,
    refractive_index : 1.05,
};
