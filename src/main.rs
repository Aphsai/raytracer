extern crate minifb;
mod geometry;

use minifb::{ Key, WindowOptions, Window };
use crate::geometry::Vec3;

const WIDTH : usize = 1280;
const HEIGHT : usize = 720;


struct Sphere {
    
}

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let point_a = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    let point_b = Vec3 { x: 1.0, y: 0.0, z: 1.0 };
    let point_c = &point_a + &point_b + &point_b;
    println!("point a : {:?}", point_a);
    println!("point b : {:?}", point_b);
    println!("point c : {:?}", point_c);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Processing
        for pixel in &mut buffer {
            *pixel = 20 << 16;
        }

        // Update window
        window.update_with_buffer(&buffer).unwrap();

    }

}
