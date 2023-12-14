use crate::math::Vec3;
use crate::ray::*;

pub struct Material {
    pub color: Vec3,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Ray, Vec3) {
        let reflected = ray.reflect(hit_record.normal);
        let scattered = Ray::from(hit_record.point + hit_record.normal * 0.01, reflected, self.color);
        (scattered, self.color)
    }
}