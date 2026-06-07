use crate::vector::Vector3;

struct Camera {
    position: Vector3,
    direction: Vector3,
    u : Vector3,
    v : Vector3,
    angle: f32,
    fov: f32,
}

impl Camera {

    pub fn new(position: Vector3, direction: Vector3, angle: f32, fov: f32) -> Self {

        let u_vec = direction.cross(&Vector3::new(0.0, 1.0, 0.0)).rotate(direction, angle).normalize();
        let v_vec = u_vec.cross(&direction).rotate(direction, angle).normalize();

        Camera {
            position,
            direction: direction.normalize(),
            u: u_vec,
            v: v_vec,
            angle,
            fov,
        }
    }

    
}