use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[allow(dead_code)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    // pub fn center(&self) -> Point3 {
    //     self.center
    // }

    pub fn set_center(&mut self, center: Point3) {
        self.center = center;
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let half_b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord { p, normal, t: root })
    }
}

pub fn hit_world(world: &[Sphere], r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut hit_record: Option<HitRecord> = None;

    for object in world {
        if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
            closest_so_far = temp_rec.t;
            hit_record = Some(temp_rec);
        }
    }

    hit_record
}
