use crate::vector::Vector3;
use crate::ray::Ray;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}
impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        
        let oc = ray.origin() - self.center;
        let a = ray.direction().norm2();
        let b = 2.0 * oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - 4.0 * a * c;
        


        if discriminant < 0.0 {

            None

        } else  {

            let d  = discriminant.sqrt() / (2.0 * a);
            let e  = -b / (2.0 * a);
            let t = (e + d).min(e - d);

            if t > 0.0 {
                Some(t)
            } else {
                None
            }

        }
    }

}

pub enum Objects {
    Sphere(Sphere),
}

impl Objects {
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        match self {
            Objects::Sphere(s) => {
                s.intersect(ray)
            }
        }
    }
}