use crate::math::Vec3;
use crate::ray::*;
use crate::material::*;
use crate::mesh::*;

pub struct Object {
    pub position: Vec3,
    pub material: Material,
    pub mesh: Mesh,
}

impl Object {
    pub fn from(position: Vec3, material: Material, mesh: Mesh) -> Self {
        Self { position, material, mesh }
    }
}