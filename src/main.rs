extern crate minifb;

use minifb::{ Key, WindowOptions, Window };

const WIDTH : usize = 1280;
const HEIGHT : usize = 720;


struct Sphere {
    
}

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Processing
        for pixel in &mut buffer {
            *pixel = 255 << 16 | 255;
        }

        // Update window
        window.update_with_buffer(&buffer).unwrap();

    }

}
