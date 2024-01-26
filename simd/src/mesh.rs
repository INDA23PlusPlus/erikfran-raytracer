use crate::object::*;
//use crate::primitives::*;
use crate::math::*;
use crate::ray::*;

pub trait MeshTrait {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, object_id: usize) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl MeshTrait for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, object_id: usize) -> Option<HitRecord> {
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
                return Some(HitRecord {
                    point,
                    normal,
                    t: temp,
                    object_id,
                });
            }
        }

        None
    }
}

#[derive(Clone, Copy)]
pub struct Plane {
    d: f32,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self {
            d: point.dot(&normal),
            normal,
        }
    }
}

impl MeshTrait for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, object_id: usize) -> Option<HitRecord> {
        let denominator = ray.direction.dot(&self.normal);

        if denominator < 0.000001 {
            return None;
        }

        let t = (self.d - ray.origin.dot(&self.normal)) / denominator;

        if t < t_max && t > t_min {
            let point = ray.at(t);
            return Some(HitRecord {
                point,
                normal: self.normal,
                t,
                object_id,
            });
        }

        None
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub normals: Vec<Vec3>,
}

impl MeshTrait for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, object_id: usize) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;

        for triangle in &self.triangles {
            if let Some(hit) = triangle.hit(ray, t_min, t_max, object_id) {
                if let Some(closest) = closest_hit {
                    if hit.t < closest.t {
                        closest_hit = Some(hit);
                    }
                } else {
                    closest_hit = Some(hit);
                }
            }
        }

        closest_hit
    }
}
