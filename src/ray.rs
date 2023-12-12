use crate::math::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Vec3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn from(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        self.direction - normal * self.direction.dot(&normal) * 2.0
    }
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
        }
    }

    pub fn from(point: Vec3, normal: Vec3, t: f32) -> Self {
        Self { point, normal, t }
    }
}