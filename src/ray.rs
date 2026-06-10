use crate::vector::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    speed: f32,
    o_time: f32,
}
impl Ray {
    pub fn new(origin: Vector3, direction: Vector3, speed:f32, o_time: f32) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
            speed,
            o_time,
        }
    }

    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * self.speed * (t-self.o_time)
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn o_time(&self) -> f32 {
        self.o_time
    }
}