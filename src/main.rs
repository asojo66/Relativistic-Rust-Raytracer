mod ray;
mod sphere;
mod vec3;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
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

    // Orthonormal basis
    let w = (origin - look_at).unit_vector();
    let up = Vec3::new(0.0, 1.0, 0.0);
    let u = up.cross(w).unit_vector();
    let v = w.cross(u);

    let horizontal = u * viewport_width;
    let vertical = v * viewport_height;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focal_length;

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
    // internal render resolution (kept constant unless you change these)
    let internal_width = 640_usize;
    let internal_height = 360_usize;

    // rendering buffer at internal resolution
    let mut render_buf = vec![0_u32; internal_width * internal_height];

    // window/backbuffer size and scaled buffer (updated when window size changes)
    let mut image_width = internal_width;
    let mut image_height = internal_height;
    let mut scaled_buf: Vec<u32> = vec![0_u32; image_width * image_height];

    // display scale (internal -> window). 1.0 = 1:1
    let mut display_scale: f64 = 1.0;

    let mut camera_position = Point3::new(0.0, 0.0, 0.0);
    let mut yaw: f64 = -90.0;
    let mut pitch: f64 = 0.0;
    let sensitivity: f64 = 0.15;
    let move_speed: f64 = 0.15;

    let mut world = build_world();
    let start = Instant::now();

    // front vector
    let mut front = Vec3::new(0.0, 0.0, -1.0);
    let mut look_at = camera_position + front;

    // initial mouse pos
    let mut window = Window::new(
        "Raytracer",
        image_width,
        image_height,
        WindowOptions { resize: true, ..WindowOptions::default() },
    )
    .expect("Unable to open window");

    window.set_target_fps(60);

    let mut prev_mouse = window.get_mouse_pos(MouseMode::Clamp);
    let mut left_down = false;
    let mut right_down = false;
    // FPS counter
    let mut frames: usize = 0;
    let mut fps: usize = 0;
    let mut fps_last = Instant::now();

    // initial render (into internal buffer)
    render_scene(&mut render_buf, internal_width, internal_height, camera_position, look_at, &world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start.elapsed().as_secs_f64();
        let object_x = elapsed.sin();
        let object_z = -1.5 + elapsed.cos() * 0.5;
        world[2].set_center(Point3::new(object_x, 0.0, object_z));

        // keyboard movement (will use camera basis)
        let forward = front;
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).unit_vector();
        let up = right.cross(forward);
        // speed multiplier (SHIFT doubles speed)
        let mut speed_mul = 1.0;
        if window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift) {
            speed_mul = 2.0;
        }

        if window.is_key_down(Key::W) {
            camera_position = camera_position + forward * (move_speed * speed_mul);
            look_at = look_at + forward * (move_speed * speed_mul);
        }
        if window.is_key_down(Key::S) {
            camera_position = camera_position - forward * (move_speed * speed_mul);
            look_at = look_at - forward * (move_speed * speed_mul);
        }
        if window.is_key_down(Key::A) {
            camera_position = camera_position - right * (move_speed * speed_mul);
            look_at = look_at - right * (move_speed * speed_mul);
        }
        if window.is_key_down(Key::D) {
            camera_position = camera_position + right * (move_speed * speed_mul);
            look_at = look_at + right * (move_speed * speed_mul);
        }
        if window.is_key_down(Key::Q) {
            camera_position = camera_position + up * (move_speed * speed_mul);
            look_at = look_at + up * (move_speed * speed_mul);
        }
        if window.is_key_down(Key::E) {
            camera_position = camera_position - up * (move_speed * speed_mul);
            look_at = look_at - up * (move_speed * speed_mul);
        }

        // mouse handling: rotate when left button held; scale when right button held
        let cur_mouse = window.get_mouse_pos(MouseMode::Clamp);
        let cur_left = window.get_mouse_down(MouseButton::Left);
        let cur_right = window.get_mouse_down(MouseButton::Right);

        // rotate with left button
        if cur_left && !left_down {
            prev_mouse = cur_mouse; // initialize
        }
        if cur_left && prev_mouse.is_some() && cur_mouse.is_some() {
            let (px, py) = prev_mouse.unwrap();
            let (cx, cy) = cur_mouse.unwrap();
            let dx = (cx - px) as f64;
            let dy = (cy - py) as f64;
            if dx != 0.0 || dy != 0.0 {
                yaw += dx * sensitivity;
                pitch += -dy * sensitivity;
                if pitch > 89.0 { pitch = 89.0; }
                if pitch < -89.0 { pitch = -89.0; }
                let yaw_r = yaw.to_radians();
                let pitch_r = pitch.to_radians();
                front = Vec3::new(yaw_r.cos() * pitch_r.cos(), pitch_r.sin(), yaw_r.sin() * pitch_r.cos()).unit_vector();
                look_at = camera_position + front;
            }
            prev_mouse = cur_mouse;
        }
        left_down = cur_left;

        // scale with right button vertical drag (relative to window)
        if cur_right && !right_down {
            prev_mouse = cur_mouse; // reuse prev_mouse
        }
        if cur_right && prev_mouse.is_some() && cur_mouse.is_some() {
            let (px, py) = prev_mouse.unwrap();
            let (cx, cy) = cur_mouse.unwrap();
            let dy = (cy - py) as f64;
            if dy.abs() >= 1.0 {
                // adjust scale multiplicatively
                display_scale *= 1.0 - dy * 0.005;
                if display_scale < 0.1 { display_scale = 0.1; }
                if display_scale > 4.0 { display_scale = 4.0; }
                prev_mouse = cur_mouse;
            }
        }
        right_down = cur_right;

        // handle window resize: update scaled_buf if required
        let (win_w, win_h) = window.get_size();
        if win_w != image_width || win_h != image_height {
            image_width = win_w;
            image_height = win_h;
            scaled_buf.resize(image_width * image_height, 0);
        }

        // render to internal buffer
        render_scene(&mut render_buf, internal_width, internal_height, camera_position, look_at, &world);

        // scale render_buf -> scaled_buf: compute fit scale so image fills window
        // while preserving aspect ratio, then apply user `display_scale` multiplier.
        let fit_scale_w = (image_width as f64) / (internal_width as f64);
        let fit_scale_h = (image_height as f64) / (internal_height as f64);
        let fit_scale = fit_scale_w.min(fit_scale_h);
        // effective scale = fit_scale * user multiplier (default 1.0 = fit)
        let effective_scale = (fit_scale * display_scale).clamp(0.1, 4.0);
        let scaled_w_f = (internal_width as f64) * effective_scale;
        let scaled_h_f = (internal_height as f64) * effective_scale;
        let scaled_w = scaled_w_f.round() as isize;
        let scaled_h = scaled_h_f.round() as isize;
        let offset_x = ((image_width as isize) - scaled_w) / 2;
        let offset_y = ((image_height as isize) - scaled_h) / 2;

        // fill with black
        for px in scaled_buf.iter_mut() {
            *px = 0;
        }

        for y in 0..scaled_h {
            for x in 0..scaled_w {
                let dst_x = x + offset_x;
                let dst_y = y + offset_y;
                if dst_x < 0 || dst_y < 0 || dst_x >= image_width as isize || dst_y >= image_height as isize {
                    continue;
                }
                // nearest neighbor sample from internal buffer using effective_scale
                let src_x_f = (x as f64) / effective_scale;
                let src_y_f = (y as f64) / effective_scale;
                let src_x = src_x_f.floor() as isize;
                let src_y = src_y_f.floor() as isize;
                if src_x < 0 || src_y < 0 || src_x >= internal_width as isize || src_y >= internal_height as isize {
                    continue;
                }
                let src_idx = (src_y as usize) * internal_width + (src_x as usize);
                let dst_idx = (dst_y as usize) * image_width + (dst_x as usize);
                scaled_buf[dst_idx] = render_buf[src_idx];
            }
        }

        // update FPS counter (per second)
        frames += 1;
        if fps_last.elapsed().as_secs_f64() >= 1.0 {
            fps = frames;
            frames = 0;
            fps_last = Instant::now();
        }

        // draw FPS overlay in top-left corner
        {
            // tiny 3x5 font for digits
            const DIGITS: [[u8; 5]; 10] = [
                [0b111, 0b101, 0b101, 0b101, 0b111], //0
                [0b010, 0b110, 0b010, 0b010, 0b111], //1
                [0b111, 0b001, 0b111, 0b100, 0b111], //2
                [0b111, 0b001, 0b111, 0b001, 0b111], //3
                [0b101, 0b101, 0b111, 0b001, 0b001], //4
                [0b111, 0b100, 0b111, 0b001, 0b111], //5
                [0b111, 0b100, 0b111, 0b101, 0b111], //6
                [0b111, 0b001, 0b010, 0b010, 0b010], //7
                [0b111, 0b101, 0b111, 0b101, 0b111], //8
                [0b111, 0b101, 0b111, 0b001, 0b111], //9
            ];

            let pad: isize = 6;
            let font_w: isize = 3;
            let font_h: isize = 5;
            let scale_font: isize = 2; // scale up for readability

            let txt = format!("{}", fps);
            let char_gap: isize = 1;
            let text_w = txt.len() as isize * (font_w * scale_font + char_gap);
            let text_h = font_h * scale_font;

            // background rectangle
            let bg_w = text_w + 8;
            let bg_h = text_h + 4;
            for by in 0..bg_h {
                for bx in 0..bg_w {
                    let x = pad + bx;
                    let y = pad + by;
                    if x < 0 || y < 0 || x >= image_width as isize || y >= image_height as isize {
                        continue;
                    }
                    scaled_buf[y as usize * image_width + x as usize] = 0x00202020; // dark grey
                }
            }

            // draw digits in white
            for (i, ch) in txt.chars().enumerate() {
                if let Some(d) = ch.to_digit(10) {
                    let d = d as usize;
                    let base_x = pad + 4 + (i as isize) * (font_w * scale_font + char_gap);
                    let base_y = pad + 2;
                    for row in 0..font_h {
                        let bits = DIGITS[d][row as usize];
                        for col in 0..font_w {
                            if ((bits >> (font_w as u8 - 1 - col as u8)) & 1) == 1 {
                                // draw scaled pixel block
                                for sy in 0..scale_font {
                                    for sx in 0..scale_font {
                                        let px = base_x + col * scale_font + sx;
                                        let py = base_y + row * scale_font + sy;
                                        if px < 0 || py < 0 || px >= image_width as isize || py >= image_height as isize {
                                            continue;
                                        }
                                        scaled_buf[py as usize * image_width + px as usize] = 0x00FFFFFF;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // copy scaled buffer to window
        window
            .update_with_buffer(&scaled_buf, image_width, image_height)
            .expect("Failed to update buffer");
    }
}
