use crate::camera::Camera;
use crate::geometry::World;
use crate::vector::Vector3;

pub struct Hit{
    hit_q: bool,
    point: Vector3,
    t: f32,
    normal: Vector3,
}
impl Hit {
    pub fn new() -> Self {
        Hit {
            hit_q: false,
            point: Vector3::new(0.0, 0.0, 0.0),
            t: f32::INFINITY,
            normal: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_hit(&mut self, point: Vector3, t: f32, normal: Vector3) {

        self.hit_q = true;

        self.point = point;
        self.t = t;
        self.normal = normal;

    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }
}

fn shade_color(color: u32, intensity: u32) -> u32 {
    let red = ((color >> 16) & 0xff) * intensity / 255;
    let green = ((color >> 8) & 0xff) * intensity / 255;
    let blue = (color & 0xff) * intensity / 255;

    (red << 16) | (green << 8) | blue
}

pub fn render(width: usize, height: usize, camera: &Camera, world: &World, t: f32, ray_speed: f32) -> Vec<u32>{

    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            
            let ray = camera.get_ray(x, y, width, height, ray_speed, t);
            let mut hit = Hit::new();
            let mut color = 0x88a9f2;

            for (_, object) in world {
                
                let color_current = object.color();
                let hit_current = object.intersect(&ray);

                if hit_current.t() < hit.t() && hit_current.t() > 0.0 {
                    hit = hit_current;
                    color = color_current;
                }
                
            }

            if hit.t() < f32::INFINITY {
                // Dirección de luz desde arriba y atrás
                let light_dir = Vector3::new(1.0, 2.0, 1.0).normalize();
                
                // Iluminación difusa (Ley de Lambert)
                let diffuse = hit.normal().dot(&light_dir).max(0.0);
                
                // Luz ambiental para suavizar sombras
                let ambient = 0.2;
                let light_int = (diffuse + ambient).min(1.0);

                let intensity = (light_int * 255.0) as u32;
                buffer[y * width + x] = shade_color(color, intensity);
            
            } else {
                    buffer[y * width + x] = color
            }

        }
    }

    buffer
}