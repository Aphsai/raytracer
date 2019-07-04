extern crate minifb;
mod geometry;

use minifb::{ Key, WindowOptions, Window };
use crate::geometry::{ Vec3, dot, cross };

const WIDTH : usize = 1280;
const HEIGHT : usize = 720;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn ray_intersect(&self, origin: Vec3, direction: Vec3) -> bool {
        let oc = origin - self.center;
        let b = dot(oc, direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let h = b * b - c;
        if h < 0.0 {
            return false
        }

        true
    }
}

fn determine_color(sphere: Sphere, origin: Vec3, direction: Vec3) -> Vec3 {
    if sphere.ray_intersect(origin, direction) {
        return Vec3 { x: 100.0, y: 0.0, z: 0.0 }
    }
    Vec3 { x: 0.0, y: 100.0, z: 0.0 }
}

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let sphere = Sphere { 
        center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        radius: 0.5,
    };

    let camera = Vec3 { x: 0.0, y: 0.0, z: -1.0 };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Processing
        for x in 0..HEIGHT {
            for y in 0..WIDTH {
               let color = determine_color(
                   sphere, 
                   camera, 
                   Vec3 { x: (x as f64/ HEIGHT as f64), y: (y as f64/ WIDTH as f64), z: 1.0 },
               );
               buffer[x * HEIGHT + y] = (color.x as u32) << 16 | (color.y as u32) << 8 | (color.z as u32);
            }
        }

            
        // Update window
        window.update_with_buffer(&buffer).unwrap();

    }

}
