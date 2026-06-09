use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm2(&self) -> f32 {
        self.dot(self)
    }

    pub fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let n = self.norm();
        if n == 0.0 {
            *self
        } else {
            *self * (1.0 / n)
        }
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn rotate(&self, axis: Vector3, angle: f32) -> Vector3 {
        
        let k = axis.normalize();

        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        *self * cos_theta + k.cross(self) * sin_theta + k * (k.dot(self) * (1.0 - cos_theta))
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

// Implement operator overloading for Vector3
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vec: Vector3) -> Vector3 {
        vec * self
    }
}

