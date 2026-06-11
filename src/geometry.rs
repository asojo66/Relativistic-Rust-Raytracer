use crate::vector::Vector3;
use crate::ray::Ray;
use crate::render::Hit;
use crate::animation::Animation;
use std::slice::Iter;

pub struct InfinitePlane {
    point: Vector3,
    normal: Vector3,
    anim: Animation
}
impl InfinitePlane {

    pub fn new(point: Vector3, normal: Vector3, anim: Animation) -> Self {
        
        InfinitePlane {point, normal, anim}

    }

    pub fn intersect(&self, ray: &Ray) -> Hit {

        let point = self.position(ray.o_time());

        let a = self.normal.dot(&(point - ray.origin()));
        let b = ray.direction().dot(&self.normal);
        
        let mut hit = Hit::new();
        if b == 0.0 {
            hit
        } else {
            
            let t = a/b/ray.speed();

            if t < 0.0 {

                hit

            } else {
                let o_time = ray.o_time();
                hit.set_hit(ray.at(t+o_time), t + o_time , self.normal);
                hit

            }

        }
    }

    pub fn position(&self, t: f32) -> Vector3 {

        match &self.anim {
            Animation::Idle => {self.point},
            Animation::Straight(s) => {self.point + s.v()*t},
            Animation::Orbit(_o) => {
                self.point 
            }
        }
        
    }

}

pub struct Sphere {
    center: Vector3,
    radius: f32,
    anim: Animation
}
impl Sphere {
    pub fn new(center: Vector3, radius: f32, anim: Animation) -> Self {
        Sphere { center, radius, anim }
    }

    pub fn center(&self, _t: f32) -> Vector3 {
        self.center
    }

    pub fn intersect(&self, ray: &Ray) -> Hit {

        let center = self.position(ray.o_time());

        let oc = ray.origin() - center;
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
            let t = (e + d).min(e - d)/ray.speed();

            if t > 0.0 {

                let c_time = t + ray.o_time();

                return_hit.set_hit(ray.at(c_time), c_time, (ray.at(c_time) - center).normalize());
                return_hit

            } else {
                return_hit
            }
            
        }
    }

    pub fn position(&self, t: f32) -> Vector3 {

        match &self.anim {
            Animation::Idle => {self.center},
            Animation::Straight(s) => {self.center + s.v()*t},
            Animation::Orbit(o) => {
                (self.center - o.center()).rotate(o.axis(), o.w()*t)+ o.center()
            }
        }
        
    }

}

pub enum Objects {
    Sphere(Sphere),
    InfinitePlane(InfinitePlane)
}

impl Objects {

    pub fn intersect(&self, ray: &Ray) -> Hit {
        match self {
            Objects::Sphere(s) =>  s.intersect(ray),
            Objects::InfinitePlane(p) => p.intersect(ray),
        }
    }

    pub fn position(&self, t:f32) -> Vector3 {
        match self {
            Objects::Sphere(s) => s.position(t),
            Objects::InfinitePlane(p) => p.position(t),
        }
    }

}

pub struct World {
    names: Vec<String>,
    objects: Vec<Objects>
}

pub struct WorldIter<'a> {
    names: Iter<'a, String>,
    objects: Iter<'a, Objects>,
}

impl<'a> Iterator for WorldIter<'a> {
    type Item = (&'a str, &'a Objects);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.names.next(), self.objects.next()) {
            (Some(name), Some(object)) => Some((name.as_str(), object)),
            _ => None,
        }
    }
}
impl World {
    pub fn new() -> Self {
        World {
            names: Vec::new(),
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, name: &str, object: Objects) -> Result<(), String> {
        if self.names.iter().any(|existing| existing == name) {
            return Err(format!("object name '{}' already exists", name));
        }

        self.names.push(name.to_string());
        self.objects.push(object);

        Ok(())
    }

    pub fn get_object(&self, name: &str) -> Option<&Objects> {
        self.names
            .iter()
            .position(|existing| existing == name)
            .map(|index| &self.objects[index])
    }

    pub fn get_object_mut(&mut self, name: &str) -> Option<&mut Objects> {
        self.names
            .iter()
            .position(|existing| existing == name)
            .map(move |index| &mut self.objects[index])
    }
}

impl<'a> IntoIterator for &'a World {
    type Item = (&'a str, &'a Objects);
    type IntoIter = WorldIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        WorldIter {
            names: self.names.iter(),
            objects: self.objects.iter(),
        }
    }
}