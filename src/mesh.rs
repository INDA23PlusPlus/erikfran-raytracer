use crate::primitives::*;
use crate::ray::*;
use crate::math::*;

pub struct Mesh{
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub normals: Vec<Vec3>,
}

impl Mesh {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = HitRecord::new();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

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
                hit_anything = true;
                closest_so_far = temp;
                hit_record.t = temp;
                hit_record.point = ray.at(temp);
                hit_record.normal = (e1.cross(&e2)).normalized();
            }
        }

        if hit_anything {
            return Some(hit_record);
        }

        None
    }
    
}