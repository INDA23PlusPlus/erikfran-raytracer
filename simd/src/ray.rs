use crate::{math::*, object::Object};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
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
    pub object_id: usize,
}