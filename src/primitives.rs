use crate::ray::*;
use crate::math::*;


pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Vec3::zero(),
            radius: 0.0,
        }
    }

    pub fn from(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {
                let point = ray.at(temp);
                let normal = (point - self.center) / self.radius;
                let hit_record = HitRecord { point, normal, t: temp };
                return Some(hit_record);
            }

            temp = (-b + discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {
                let point = ray.at(temp);
                let normal = (point - self.center) / self.radius;
                let hit_record = HitRecord { point, normal, t: temp };
                return Some(hit_record);
            }
        }

        None
    }
}

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            point: Vec3::zero(),
            normal: Vec3::zero(),
        }
    }

    pub fn from(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denominator = self.normal.dot(&ray.direction);

        if denominator.abs() > 0.0001 {
            let t = (self.point - ray.origin).dot(&self.normal) / denominator;

            if t < t_max && t > t_min {
                let point = ray.at(t);
                let hit_record = HitRecord { point, normal: self.normal, t };
                return Some(hit_record);
            }
        }

        None
    }
}