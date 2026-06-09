use crate::camera::Camera;
use crate::geometry::Objects;
use crate::ray::Ray;
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

    pub fn point(&self) -> Vector3 {
        self.point
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }
}

pub fn render(width: usize, height: usize, camera: &Camera, world: &Vec<Objects>, dtime: f32) -> Vec<u32>{

    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            
            let ray = camera.get_ray(x, y, width, height);
            let mut hit = Hit::new();

            for object in world {

                let hit_current = object.intersect(&ray);

                if hit_current.t() < hit.t() && hit_current.t() > 0.0 {
                    hit = hit_current;
                }
                
            }

            if hit.t() < f32::INFINITY {
                
                        let light_int = (hit.normal().dot(&(camera.position()-hit.point()).normalize())+0.5).clamp(0.0, 1.0);

                        let intensity = (light_int * 255.0) as u32;
                        buffer[y * width + x] = intensity << 16;
            
            } else {
                    buffer[y * width + x] = 0x88a9f2
            }

        }
    }

    buffer
}