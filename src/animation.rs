use crate::vector::Vector3;

pub struct Straight {
    v: Vector3,
}
impl Straight {
    pub fn new(v: Vector3) -> Self {Straight { v }}
    pub fn v(&self) -> Vector3 {self.v}
}

pub struct Orbit {
    center: Vector3,
    axis: Vector3,
    w: f32
}

pub enum Animation {
    Idle,
    Straight(Straight),
    Orbit(Orbit)
}