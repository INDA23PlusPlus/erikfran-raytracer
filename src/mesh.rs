use crate::object::*;
//use crate::primitives::*;
use crate::ray::*;
use crate::math::*;

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
                return Some(HitRecord { point, normal, t: temp, object_id });
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
        Self { d: point.dot(&normal), normal }
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
            return Some(HitRecord { point, normal: self.normal, t, object_id });
        }

        None
    }
}

#[derive(Clone)]
pub struct Mesh{
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub normals: Vec<Vec3>,
}

impl MeshTrait for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, object_id: usize) -> Option<HitRecord> {
        let mut hit_record = None;

        for i in (0..self.indices.len()).step_by(3) {
            let i0 = self.indices[i];
            let i1 = self.indices[i + 1];
            let i2 = self.indices[i + 2];

            let v0 = self.vertices[i0];
            let v1 = self.vertices[i1];
            let v2 = self.vertices[i2];

            let e1 = v1 - v0;
            let e2 = v2 - v0;
            let p = ray.direction.cross(&e2);
            let det = e1.dot(&p);

            if det > -0.000001 && det < 0.000001 {
                continue;
            }

            let inv_det = 1.0 / det;
            let t = ray.origin - v0;
            let u = t.dot(&p) * inv_det;

            if u < 0.0 || u > 1.0 {
                continue;
            }

            let q = t.cross(&e1);
            let v = ray.direction.dot(&q) * inv_det;

            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let temp = e2.dot(&q) * inv_det;

            if temp < t_max && temp > t_min {
                hit_record = Some(HitRecord {
                    point: ray.at(temp),
                    normal: (e1.cross(&e2)).normalized(),
                    t: temp,
                    object_id,
                });
            }
        }

        hit_record
    }
    
}