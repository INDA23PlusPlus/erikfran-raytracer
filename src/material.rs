use std::fmt::Display;

use crate::math::Vec3;
use crate::ray::*;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert to Vec3 with values between 0.0 and 1.0
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::from(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0)
    }
}

impl From<Vec3> for Color {
    fn from(vec: Vec3) -> Self {
        Self {
            r: (vec.x * 255.0) as u8,
            g: (vec.y * 255.0) as u8,
            b: (vec.z * 255.0) as u8,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            r: (self.r as f32 * other) as u8,
            g: (self.g as f32 * other) as u8,
            b: (self.b as f32 * other) as u8,
        }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Option<Ray>, Vec3);
}

pub struct PointLightMaterial {
    pub color: Vec3,
}

impl Material for PointLightMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Option<Ray>, Vec3) {
        (None, self.color)
    }
}

pub struct Diffuse {
    pub color: Vec3,
}

impl Diffuse {
    pub fn boxed(color: Vec3) -> Box<dyn Material> {
        Box::new(Self { color })
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Option<Ray>, Vec3) {
        let ray = Ray {
            origin: hit_record.point,
            direction: hit_record.normal + Vec3::random_unit_vector(),
        };

        (Some(ray), self.color)
    }
}

// Metal
// Mirror
// Mesh