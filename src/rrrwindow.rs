
use minifb::{Window, WindowOptions};

pub fn initialize_window(width: usize, height: usize, fps: usize) -> Window {
    // Create the window instance
    let mut window = Window::new(
        "Relativistic Rust Raytracer",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| { // Handle window creation error
        panic!("{}", e);
    });

    // Set the FPS limit to 60 FPS
    window.set_target_fps(fps);

    window
    
}

