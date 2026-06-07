mod camera;
mod rrrwindow;
mod vector;
mod render;
mod geometry;
mod ray;

use minifb::{Key};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: usize = 60;


fn main() {

    // Initialize buffer storage
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Create the window instance
    let mut window = rrrwindow::initialize_window(WIDTH, HEIGHT, FPS);

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Update the buffer with pixel data (for demonstration, we fill it with a gradient)

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let r = (x as f32 / WIDTH as f32 * 255.0) as u32;
                let g = (y as f32 / HEIGHT as f32 * 255.0) as u32;
                let b = 128; // Fixed blue value
                buffer[y * WIDTH + x] = (r << 16) | (g << 8) | b; // Combine RGB into a single u32
            }
        }

        // 5. Actualizar la ventana con el búfer modificado
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
