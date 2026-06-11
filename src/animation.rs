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
impl Orbit {
    pub fn new(center: Vector3, axis: Vector3, w: f32) -> Self {
         Orbit { center, axis, w }
    }

    pub fn center(&self) -> Vector3 {
        self.center
    }

    pub fn axis(&self) -> Vector3 {
        self.axis
    }
    pub fn w(&self) -> f32 {
        self.w
    }

}

pub enum Animation {
    Idle,
    Straight(Straight),
    Orbit(Orbit)
}