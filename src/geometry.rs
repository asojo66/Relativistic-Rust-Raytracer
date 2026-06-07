use crate::vector::Vector3;
use crate::ray::Ray;
use crate::render::Hit;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}
impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn intersect(&self, ray: &Ray) -> Hit {

        let oc = ray.origin() - self.center;
        let a = ray.direction().norm2();
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - 4.0 * a * c;
        
        let mut return_hit = Hit::new();

        if discriminant < 0.0 {

            return_hit

        } else  {

            let d  = discriminant.sqrt() / (2.0 * a);
            let e  = -b / (2.0 * a);
            let t = (e + d).min(e - d);

            if t > 0.0 {

                return_hit.set_hit(ray.at(t), t, (ray.at(t) - self.center).normalize());
                return_hit

            } else {
                return_hit
            }
            
        }
    }

}

pub enum Objects {
    Sphere(Sphere),
}

impl Objects {
    pub fn intersect(&self, ray: &Ray) -> Hit {
        match self {
            Objects::Sphere(s) => {
                s.intersect(ray)
            }
        }
    }
}