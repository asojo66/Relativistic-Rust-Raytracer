use crate::vector::Vector3;
use crate::ray::Ray;

pub struct Camera {
    position: Vector3,
    f_length: f32,
    direction: Vector3,
    u : Vector3,
    v : Vector3,
    angle: f32,
    fov: f32,
}

impl Camera {

    pub fn new(position: Vector3, f_length: f32, direction: Vector3, angle: f32, fov: f32) -> Self {

        let u_vec = direction.cross(&Vector3::new(0.0, 1.0, 0.0)).rotate(direction, angle).normalize();
        let v_vec = u_vec.cross(&direction).rotate(direction, angle).normalize();

        Camera {
            position,
            f_length,
            direction: direction.normalize(),
            u: u_vec,
            v: v_vec,
            angle,
            fov,
        }
    }

    pub fn get_ray(&self, ix:usize, iy:usize, width: usize, height: usize) -> Ray {

        let focal_point = self.position - self.direction * self.f_length;

        let h_size = 2.0 * self.f_length * (self.fov.to_radians() / 2.0).tan();
        let w_size = h_size * (width as f32 / height as f32);

        let left_upper_corner = self.v * (h_size / 2.0)-self.u * (w_size / 2.0);

        let in_plane = left_upper_corner + ix as f32*h_size / width as f32 * self.u - iy as f32*h_size / height as f32 * self.v;

        Ray::new(
            focal_point,
            (in_plane - focal_point).normalize(),
            0.0,
        )
            
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn set_position(&mut self, new_position: Vector3) {
        self.position = new_position;
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn u(&self) -> Vector3 {
        self.u
    }

    pub fn v(&self) -> Vector3 {
        self.v
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    
}