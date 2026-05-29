mod ray;
mod sphere;
mod vec3;

use minifb::{Key, Window, WindowOptions};
use ray::Ray;
use sphere::{hit_world, Sphere};
use std::time::Instant;
use vec3::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &[Sphere]) -> Color {
    if let Some(rec) = hit_world(world, r, 0.001, f64::INFINITY) {
        let n = rec.normal.unit_vector();
        return Color::new(
            0.5 * (n.x() + 1.0),
            0.5 * (n.y() + 1.0),
            0.5 * (n.z() + 1.0),
        );
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn color_to_pixel(pixel_color: Color) -> u32 {
    let ir = (255.999 * pixel_color.x()).clamp(0.0, 255.0) as u32;
    let ig = (255.999 * pixel_color.y()).clamp(0.0, 255.0) as u32;
    let ib = (255.999 * pixel_color.z()).clamp(0.0, 255.0) as u32;
    (ir << 16) | (ig << 8) | ib
}

fn build_world() -> Vec<Sphere> {
    vec![
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(-1.0, 0.0, -1.5), 0.5),
        Sphere::new(Point3::new(1.0, 0.0, -1.5), 0.5),
    ]
}

fn render_scene(
    buffer: &mut [u32],
    image_width: usize,
    image_height: usize,
    origin: Point3,
    look_at: Point3,
    world: &[Sphere],
) {
    let aspect_ratio = image_width as f64 / image_height as f64;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let direction = (look_at - origin).unit_vector();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 + direction * focal_length;

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = (image_height - 1 - j) as f64 / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            buffer[j * image_width + i] = color_to_pixel(ray_color(&r, world));
        }
    }
}

fn main() {
    let image_width = 640;
    let image_height = 360;
    let mut buffer = vec![0_u32; image_width * image_height];

    let mut camera_position = Point3::new(0.0, 0.0, 0.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let move_speed = 0.15;

    let mut world = build_world();
    let start = Instant::now();
    render_scene(&mut buffer, image_width, image_height, camera_position, look_at, &world);

    let mut window = Window::new(
        "Raytracer",
        image_width,
        image_height,
        WindowOptions::default(),
    )
    .expect("Unable to open window");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start.elapsed().as_secs_f64();
        let object_x = elapsed.sin();
        let object_z = -1.5 + elapsed.cos() * 0.5;
        world[2].set_center(Point3::new(object_x, 0.0, object_z));

        let mut moved = false;
        if window.is_key_down(Key::W) {
            camera_position = camera_position + Vec3::new(0.0, 0.0, -move_speed);
            moved = true;
        }
        if window.is_key_down(Key::S) {
            camera_position = camera_position + Vec3::new(0.0, 0.0, move_speed);
            moved = true;
        }
        if window.is_key_down(Key::A) {
            camera_position = camera_position + Vec3::new(-move_speed, 0.0, 0.0);
            moved = true;
        }
        if window.is_key_down(Key::D) {
            camera_position = camera_position + Vec3::new(move_speed, 0.0, 0.0);
            moved = true;
        }
        if window.is_key_down(Key::Q) {
            camera_position = camera_position + Vec3::new(0.0, move_speed, 0.0);
            moved = true;
        }
        if window.is_key_down(Key::E) {
            camera_position = camera_position + Vec3::new(0.0, -move_speed, 0.0);
            moved = true;
        }

        if moved {
            render_scene(&mut buffer, image_width, image_height, camera_position, look_at, &world);
        } else {
            // Still re-render when the object moves
            render_scene(&mut buffer, image_width, image_height, camera_position, look_at, &world);
        }

        window
            .update_with_buffer(&buffer, image_width, image_height)
            .expect("Failed to update buffer");
    }
}
