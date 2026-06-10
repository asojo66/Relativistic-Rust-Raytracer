mod camera;
mod rrrwindow;
mod vector;
mod render;
mod ray;
mod geometry;

use crate::render::render;
use crate::camera::Camera;
use crate::geometry::{Objects, Sphere};
use crate::vector::Vector3;
use minifb::{Key};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FPS: usize = 60;

fn main() {

    // Initialize buffer storage
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Create the window instance
    let mut window = rrrwindow::initialize_window(WIDTH, HEIGHT, FPS);

    let world = vec![
        Objects::Sphere(Sphere::new(
            Vector3::new(5.0, 0.0, 0.0),
            1.0,
        )),
        Objects::Sphere(Sphere::new(
            Vector3::new(5.0, 0.0, -11.0),
            10.0,
        ))
    ];

    let mut cam = Camera::new(
                vector::Vector3::new(0.0, 0.0, 0.0),
                1.0,
                vector::Vector3::new(1.0, 0.0, 0.0),
                0.0,
                90.0,
    );

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Update the buffer with pixel data (for demonstration, we fill it with a gradient)

        buffer = render(
            WIDTH, 
            HEIGHT, 
            &cam,
            &world,
            0.0
        );

        // 5. Actualizar la ventana con el búfer modificado
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        
        let mut camspeed = 0.1;

        if window.is_key_down(Key::LeftCtrl) {
            camspeed = 0.3;
        }

        if window.is_key_down(Key::W) {
            cam.set_position(
                cam.position() + cam.direction() * camspeed
            )
        }
        if window.is_key_down(Key::S) {
            cam.set_position(
                cam.position() - cam.direction() * camspeed
            )
        }
        if window.is_key_down(Key::D) {
            cam.set_position(
                cam.position() + cam.u() * camspeed
            )
        }

        if window.is_key_down(Key::A) {
            cam.set_position(
                cam.position() - cam.u() * camspeed
            )
        }

        if window.is_key_down(Key::Space) {
            cam.set_position(
                cam.position() + cam.v() * camspeed
            )
        }

        if window.is_key_down(Key::LeftShift) {
            cam.set_position(
                cam.position() - cam.v() * camspeed
            )
        }

        if window.is_key_down(Key::Up) {
            cam.rotate(0.0, 0.05);
        }

        if window.is_key_down(Key::Down) {
            cam.rotate(0.0, -0.05);
        }

        if window.is_key_down(Key::Left) {
            cam.rotate(0.05, 0.0);
        }

        if window.is_key_down(Key::Right) {
            cam.rotate(-0.05, 0.0);
        }


    }
}
