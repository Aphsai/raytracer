extern crate minifb;
mod geometry;
mod material;
mod light;

use minifb::{ Key, WindowOptions, Window };
use crate::geometry::{ Vec3, dot };
use crate::material::{ Material };
use crate::light::{ Light };
use std::f64;

const WIDTH : usize = 1280;
const HEIGHT : usize = 720;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn ray_intersect(&self, origin: Vec3, direction: Vec3, distance: &mut f64) -> bool {

        let oc = self.center - origin;
        let b = dot(oc, direction);
        let d = dot(oc, oc) - b * b;
        if d > self.radius * self.radius {
            return false;
        }
        let t = (self.radius * self.radius - d).sqrt();

        if d - t > 0.0 {
            *distance = d - t;
            return true;
        }

        if d + t > 0.0 {
            *distance = d + t;
            return true;
        }

        return false;
    }
}

fn cast_ray(spheres: &Vec<Sphere>, lights: &Vec<Light>, origin: Vec3, direction: Vec3) -> Vec3 {
    let mut color = Vec3 { x: 0.0, y: 50.0, z: 50.0 };
    'sphere_loop: for sphere in spheres {
        let mut distance = 0.0;
        let mut diffuse_light_intensity = 0.0;
        if sphere.ray_intersect(origin, direction, &mut distance) {
                for light in lights {

                    let point = origin + direction * distance;
                    let mut normal = point - sphere.center;
                    let mut light_direction = light.position - point;
                    normal.normalize();
                    light_direction.normalize();

                    diffuse_light_intensity += light.intensity *  dot(light_direction, normal).max(0.0);
                }
                color = sphere.material.diffuse_color * diffuse_light_intensity;
                break 'sphere_loop;

            }
        }
    return color;
}

fn main() {
    let fov : f64 = 90.0 / 180.0 * f64::consts::PI;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions { borderless: true, title: false, resize: false, scale: minifb::Scale::X1 } ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    let mut spheres: Vec<Sphere> = Vec::new();
    let mut lights: Vec<Light> = Vec::new();

    let s1 = Sphere { 
        center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        material: Material { diffuse_color: Vec3 { x: 100.0, y: 0.0, z: 1.0 } },
        radius: 0.4,
    };
    let s2 = Sphere { 
        center: Vec3 { x: 0.5, y: 0.0, z: 0.0 },
        material: Material { diffuse_color: Vec3 { x: 100.0, y: 20.0, z: 30.0 } },
        radius: 0.4,
    };
    let s3 = Sphere { 
        center: Vec3 { x: -0.5, y: 0.0, z: 0.0 },
        material: Material { diffuse_color: Vec3 { x: 100.0, y: 20.0, z: 30.0 } },
        radius: 0.4,
    };
    let s4 = Sphere { 
        center: Vec3 { x: 0.0, y: 0.5, z: 0.0 },
        material: Material { diffuse_color: Vec3 { x: 100.0, y: 20.0, z: 30.0 } },
        radius: 0.4,
    };

    let l1 = Light {
        position: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
        intensity: 1.5
    };
    
    spheres.push(s1);
    spheres.push(s2);
    spheres.push(s3);
    spheres.push(s4);

    lights.push(l1);

    let camera = Vec3 { x: 0.0, y: 0.0, z: -1.0 };

    for x in 0..HEIGHT {
        for y in 0..WIDTH {

            let i = (2.0 * (y as f64 + 0.5) / (WIDTH as f64) - 1.0) * (fov / 2.0).tan() * camera.z.abs() * (WIDTH as f64) / (HEIGHT as f64);
            let j = -(2.0 * (x as f64 + 0.5) / (HEIGHT as f64) - 1.0) * (fov / 2.0).tan() * camera.z.abs();

            let mut dir = Vec3 { x: i, y: j, z: 1.0 };
            dir.normalize();

            let color = cast_ray(&spheres, &lights, camera, dir);

            buffer[x * WIDTH + y] = (color.x as u32) << 16 | (color.y as u32) << 8 | (color.z as u32);

        }
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Processing
        // Update window
        window.update_with_buffer(&buffer).unwrap();

    }

}
