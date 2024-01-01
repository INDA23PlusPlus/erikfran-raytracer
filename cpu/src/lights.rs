use crate::math::*;
use crate::material::*;
use crate::object::*;
use crate::mesh::*;

pub struct PointLight;

impl PointLight {
    pub fn new(position: Vec3, radius: f32, range: f32, color: Color) -> Sphere {
        Object::sphere(
            position, 
            radius, 
            Box::new(PointLightMaterial { color })
        )
    }
}