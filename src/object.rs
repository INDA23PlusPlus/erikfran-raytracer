use bytemuck::Pod;

use crate::math::Vec3;
use crate::ray::*;
use crate::material::*;
use crate::mesh::*;

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn zero() -> Self {
        Self {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::one(),
        }
    }

    pub fn from(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Vec3::zero(),
            scale: Vec3::one(),
        }
    }
}

#[repr(C)]
pub struct Object {
    pub transform: Transform,
    pub material: Box<dyn Material>,
    mesh: Box<dyn MeshTrait>,
    pub id: usize,
}

impl Object {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.mesh.hit(ray, t_min, t_max, self.id)
    }

    pub fn sphere(center: Vec3, radius: f32, material: Box<dyn Material>, id: usize) -> Self {
        Self {
            transform: Transform::from_position(center),
            material,
            mesh: Box::new(Sphere { center, radius }),
            id,
        }
    }

    pub fn plane(point: Vec3, normal: Vec3, material: Box<dyn Material>, id: usize) -> Self {
        Self {
            transform: Transform::from_position(point),
            material,
            mesh: Box::new(Plane::new(point, normal)),
            id,
        }
    }

    pub fn from_mesh(position: Vec3, mesh: Mesh, material: Box<dyn Material>, id: usize) -> Self {
        Self {
            transform: Transform::from_position(position),
            material,
            mesh: Box::new(mesh),
            id,
        }
    }

    pub fn point_light(position: Vec3, radius: f32, range: f32, color: Vec3, id: usize) -> Self {
        Self {
            transform: Transform::from_position(position),
            material: Box::new(PointLightMaterial { color }),
            mesh: Box::new(Sphere { center: position, radius }),
            id,
        }
    }
}
