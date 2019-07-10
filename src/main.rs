extern crate minifb;
mod geometry;

use minifb::{ Key, WindowOptions, Window };
use crate::geometry::{ Vec3, dot, cross };
use std::f64;

const WIDTH : usize = 1280;
const HEIGHT : usize = 720;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub color: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn ray_intersect(&self, origin: Vec3, direction: Vec3) -> bool {

        let oc = self.center - origin;
        let b = dot(oc, direction);
        let d = dot(oc, oc) - b * b;

        return d < self.radius * self.radius 
    }
}

fn determine_color(spheres: &Vec<Sphere>, origin: Vec3, direction: Vec3) -> Vec3 {
    let mut color = Vec3 { x: 0.0, y: 50.0, z: 50.0 };
    for sphere in spheres {
        if sphere.ray_intersect(origin, direction) {
           color = sphere.color;
           break;
        }
    }
    return color
}

fn main() {
    let fov : f64 = 90.0 / 180.0 * f64::consts::PI;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    let mut spheres: Vec<Sphere> = Vec::new();
    let s1 = Sphere { 
        center: Vec3 { x: 0.5, y: 0.5, z: 0.0 },
        color: Vec3 { x: 100.0, y: 0.0, z: 1.0 },
        radius: 0.5,
    };

    spheres.push(s1);

    let camera = Vec3 { x: 0.0, y: 0.0, z: -1.0 };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Processing
        for x in 0..HEIGHT {
            for y in 0..WIDTH {

                let i = (2.0 * (y as f64 + 0.5) / (WIDTH as f64) - 1.0) * (fov / 2.0).tan() * camera.z.abs() * (WIDTH as f64) / (HEIGHT as f64);
                let j = (2.0 * (x as f64 + 0.5) / (HEIGHT as f64) - 1.0) * (fov / 2.0).tan() * camera.z.abs();

                let mut dir = Vec3 { x: i, y: j, z: 1.0 };

                dir.make_unit_vector();
                let color = determine_color(&spheres, camera, dir);

                buffer[x * WIDTH + y] = (color.x as u32) << 16 | (color.y as u32) << 8 | (color.z as u32);

            }
        }

            
        // Update window
        window.update_with_buffer(&buffer).unwrap();

    }

}
