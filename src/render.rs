use crate::camera::Camera;
use crate::geometry::Objects;
use crate::ray::Ray;

pub fn render(width: usize, height: usize, camera: &Camera, world: &Vec<Objects>, dtime: f32) -> Vec<u32>{

    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            
            let ray = camera.get_ray(x, y, width, height);

            for object in world {
                if let Some(t) = object.intersect(&ray) {
                    buffer[y * width + x] = 0xFF0000; 
                } else {
                    buffer[y * width + x] = 0x000000
                }
            }

        }
    }

    buffer
}