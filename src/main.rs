extern crate minifb;
extern crate threadpool;
extern crate num_cpus;
extern crate crossbeam;

mod geometry;
mod material;
mod light;

use minifb::{ Key, WindowOptions, Window };
use threadpool::ThreadPool;
use crossbeam::atomic::AtomicCell;
use crate::geometry::{ Vec3, dot };
use crate::material::{ Material };
use crate::light::{ Light };
use std::f64;
use std::sync::{ Arc, Mutex };
//use std::sync::atomic::{ AtomicU32 , Ordering };
use std::sync::mpsc;

const WIDTH : usize = 1280;
const HEIGHT : usize = 720;
const MAX_DEPTH : u16 = 4;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Vec3,
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

        if b - t > 0.0 {
            *distance = b - t;
            return true;
        }

        if b + t > 0.0 {
            *distance = b + t;
            return true;
        }

        return false;
    }
}

fn cast_ray(spheres: &Vec<Sphere>, lights: &Vec<Light>, origin: Vec3, direction: Vec3, depth: u16) -> Vec3 {
    let mut color = Vec3 { x: 0.0, y: 0.2, z: 0.2 };
    if depth > MAX_DEPTH {
        return color;
    }
    for sphere in spheres {
        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        let mut reflect_color = Vec3::new();
        let mut distance = 0.0;

        if sphere.ray_intersect(origin, direction, &mut distance) {
            for light in lights {
                let point = origin + direction * distance;
                let mut normal = point - sphere.center;
                let mut light_direction = light.position - point;

                normal.normalize();
                light_direction.normalize();

                let reflect_direction = direction.reflect(&normal);
                let reflect_origin = if dot(reflect_direction, normal) < 0.0 { point - normal * 1e-3 } else { point + normal * 1e-3 };

                reflect_color = cast_ray(&spheres, &lights, reflect_origin, reflect_direction, depth + 1);

                diffuse_light_intensity += light.intensity *  dot(light_direction, normal).max(0.0);
                specular_light_intensity += dot(-(-light_direction.reflect(&normal)), direction).max(0.0).powf(sphere.material.specular_exponent) * light.intensity;

            }
            color = sphere.material.diffuse_color * diffuse_light_intensity * sphere.material.albedo.x + specular_light_intensity * sphere.material.albedo.y * Vec3::unit() + reflect_color * sphere.material.albedo.z;
        }
    }
    return color;
}

fn main() {
    let fov = 90.0 / 180.0 * f64::consts::PI;
    let camera = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
    let pool = ThreadPool::new(num_cpus::get());

    let buffer = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions { borderless: true, title: false, resize: false, scale: minifb::Scale::X1 } ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let s1 = Sphere { 
        center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        material: material::RED_RUBBER,
        radius: 0.4,
    };
    let s2 = Sphere { 
        center: Vec3 { x: -0.1, y: 0.1, z: 0.0 },
        material: material::GLASS,
        radius: 0.2,
    };
    let s3 = Sphere { 
        center: Vec3 { x: -0.5, y: -0.3, z: 0.0 },
        material: material::IVORY,
        radius: 0.1,
    };
    let s4 = Sphere { 
        center: Vec3 { x: 0.5, y: 0.5, z: 0.0 },
        material: material::MIRROR,
        radius: 0.3,
    };

    let l1 = Light {
        position: Vec3 { x: 1.0, y: 1.0, z: -10.0 },
        intensity: 1.3
    };
    let l2 = Light {
        position: Vec3 { x: -2.0, y: -1.0, z: -5.0 },
        intensity: 1.1
    };
    let l3 = Light {
        position: Vec3 { x: 0.0, y: -2.0, z: -10.0 },
        intensity: 1.2
    };
    
    let spheres = Arc::new(vec![s1, s2, s3, s4]);
    let lights = Arc::new(vec![l1, l2, l3]);

    //let (tx, rx) = mpsc::channel();

    for x in 0..HEIGHT {
        for y in 0..WIDTH {
            let spheres_t = Arc::clone(&spheres);
            let lights_t = Arc::clone(&lights);
            //let tx_t = tx.clone();
            let buffer_t = buffer.clone();
            pool.execute(move || {

                let i = (2.0 * (y as f64 + 0.5) / (WIDTH as f64) - 1.0) * (fov / 2.0).tan() * camera.z.abs() * (WIDTH as f64) / (HEIGHT as f64);
                let j = -(2.0 * (x as f64 + 0.5) / (HEIGHT as f64) - 1.0) * (fov / 2.0).tan() * camera.z.abs();

                let mut dir = Vec3 { x: i, y: j, z: 1.0 };
                dir.normalize();

                let mut color = cast_ray(&spheres_t, &lights_t, camera, dir, 0);
                color.x = color.x.min(1.0) * 255.0;
                color.y = color.y.min(1.0) * 255.0;
                color.z = color.z.min(1.0) * 255.0;

                let mut buffer_t = buffer_t.lock().unwrap();

                buffer_t[x * WIDTH + y] = (color.x as u32) << 16 | (color.y as u32) << 8 | (color.z as u32);

            });
        }
    }
    pool.join();
    //for received in rx {
    //}

    let compiled_buffer = buffer.lock().unwrap().to_vec();
    while window.is_open() && !window.is_key_down(Key::Escape) {    
        window.update_with_buffer(&compiled_buffer).unwrap();
    }

}
