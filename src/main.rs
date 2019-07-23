extern crate minifb;
extern crate threadpool;
extern crate num_cpus;
extern crate crossbeam;

mod geometry;
mod material;
mod light;

use minifb::{ Key, WindowOptions, Window };
use threadpool::ThreadPool;
use crate::geometry::{ Vec3, dot };
use crate::material::{ Material };
use crate::light::{ Light };
use std::f64;
use std::sync::{ Arc, Mutex };

const WIDTH : usize = 1280;
const HEIGHT : usize = 720;
const MAX_DEPTH : u16 = 4;
const MAX_RENDER_DISTANCE : f64 = 1000.0;

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
        if d > self.radius * self.radius { return false; }
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

#[derive(Clone, Copy)]
pub struct Hit {
    pub distance: f64,
    pub normal: Vec3,
    pub point: Vec3,
    pub material: Material,
}

impl Hit {
    fn new() -> Hit {
        Hit {
            distance: MAX_RENDER_DISTANCE + 1e-3, 
            normal: Vec3::new(),
            point: Vec3::new(),
            material: material::RED_RUBBER,
        }
    }
}

fn intersect_scene(spheres: &Vec<Sphere>, origin: Vec3, direction: Vec3, hit: &mut Hit) -> bool {
    for sphere in spheres {
        let mut current_distance = 0.0;
        if sphere.ray_intersect(origin, direction, &mut current_distance) && current_distance < hit.distance {
            hit.distance =  current_distance;
            hit.material = sphere.material;
            hit.point = origin + direction * current_distance;
            hit.normal = hit.point - sphere.center;
        }
    }
    hit.normal.normalize();
    return hit.distance < MAX_RENDER_DISTANCE;
}

fn cast_ray(spheres: &Vec<Sphere>, lights: &Vec<Light>, origin: Vec3, direction: Vec3, depth: u16) -> Vec3 {
    let mut color = Vec3 { x: 0.0, y: 0.2, z: 0.2 };

    let mut hit = Hit::new();

    if intersect_scene(&spheres, origin, direction, &mut hit) && depth < MAX_DEPTH {

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        let mut reflect_color = Vec3::new();
    
        for light in lights {

            let light_distance = (light.position - hit.point).squared_length();
            let mut light_direction = light.position - hit.point;

            light_direction.normalize();

            let shadow_origin = if dot(light_direction, hit.normal) < 0.0 { hit.point - hit.normal * 1e-3 } else { hit.point + hit.normal * 1e-3 };
            let mut shadow_hit = Hit::new();

            if intersect_scene(&spheres, shadow_origin, light_direction, &mut shadow_hit) && (shadow_hit.point - shadow_origin).squared_length() < light_distance { continue; }

            let reflect_direction = direction.reflect(&(hit.normal));
            let reflect_origin = if dot(reflect_direction, hit.normal) < 0.0 { hit.point - hit.normal * 1e-3 } else { hit.point + hit.normal * 1e-3 };

            reflect_color = cast_ray(&spheres, &lights, reflect_origin, reflect_direction, depth + 1);

            diffuse_light_intensity += light.intensity *  dot(light_direction, hit.normal).max(0.0);
            specular_light_intensity += dot(-(-light_direction.reflect(&hit.normal)), direction).max(0.0).powf(hit.material.specular_exponent) * light.intensity;

        }

        color = hit.material.diffuse_color * diffuse_light_intensity * hit.material.albedo.x + specular_light_intensity * hit.material.albedo.y * Vec3::unit() + reflect_color * hit.material.albedo.z;
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

    for x in 0..HEIGHT {
        for y in 0..WIDTH {
            let spheres_t = Arc::clone(&spheres);
            let lights_t = Arc::clone(&lights);
            let buffer_t = Arc::clone(&buffer);
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

    while window.is_open() && !window.is_key_down(Key::Escape) {    
        window.update_with_buffer(&buffer.lock().unwrap().to_vec()).unwrap();
    }

}
