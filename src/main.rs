use std::{fs, path::Path};

use crate::math::*;
use crate::ray::*;
// use crate::primitives::*;
use crate::material::*;
use crate::object::*;
use crate::mesh::*;

mod math;
mod ray;
// mod primitives;
mod material;
mod object;
mod mesh;
// mod lights;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = WIDTH;
const VIEWPORT_DISTANCE: f32 = 1.0;

const SKY_COLOR: Vec3 = /* Vec3 { x: 0.0, y: 0.0, z: 0.0 }; */ Vec3 { x: 0.5, y: 0.7, z: 1.0 };

const MAX_DEPTH: u32 = 10;
const NUM_SAMPLES: u32 = 5000;

fn main() {
    let mut current = 0;
    let mut current_path = format!("images/image{}.ppm", current);
    while Path::new(&current_path).exists() {
        current += 1;
        current_path = format!("images/image{}.ppm", current);
    }

    let mut image = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);

    let mut objects = Vec::new();
/*     objects.push(Object::plane(
        Vec3::from(1.0, 0.0, 0.0),
        Vec3::from(1.0, 0.0, 0.0),
        Box::new(PointLightMaterial { color: Vec3::from(1.0, 1.0, 1.0) } ),
        objects.len()
    )); */
    objects.push(Object::sphere(
        Vec3::from(0.0, 0.0, 4.0),
        0.7,
        Diffuse::boxed(Vec3::from(0.5, 0.5, 0.5).into()),
        objects.len()
    ));
    objects.push(Object::point_light(
        Vec3::from(-2.0, 0.5, 4.0),
        0.7,
        100.0,
        (Vec3::one() * 10.0).into(),
        objects.len()
    ));
/*     objects.push(Object::plane(
        Vec3::from(1.0, 0.0, 0.0),
        Vec3::from(-1.0, 0.0, 0.0),
        Diffuse::boxed(Vec3::from(0.5, 0.5, 0.5).into()),
        objects.len()
    )); */
    objects.push(Object::point_light(
        Vec3::from(0.0, 1.7, 4.0),
        0.7,
        100.0,
        (Vec3::one() * 10.0).into(),
        objects.len()
    ));
    objects.push(Object::sphere(
        Vec3::from(100.7, 0.0, 4.0),
        100.0,
        Diffuse::boxed(Vec3::from(0.5, 1.0, 0.3).into()),
        objects.len()
    ));

    let camera_position = Vec3::from(0.0, 0.0, 0.0);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let viewport_x = (x as f32 - WIDTH as f32 / 2.0) / WIDTH as f32;
            let viewport_y = (y as f32 - HEIGHT as f32 / 2.0) / HEIGHT as f32;

            let average_color = (0..NUM_SAMPLES).fold(Vec3::zero(), |acc, _| {
                    let ray = Ray {
                        origin: camera_position.clone(),
                        direction: Vec3 { 
                            x: viewport_x - (0.5 + rand::random::<f32>()) / WIDTH as f32, 
                            y: viewport_y - (0.5 + rand::random::<f32>()) / HEIGHT as f32, 
                            z: VIEWPORT_DISTANCE 
                        }.normalized(),
                    };
                    acc + ray_caste(&ray, &objects, MAX_DEPTH)
                }) / NUM_SAMPLES as f32;

            image += Color::from(average_color).to_string().as_str();
            image += " ";
        }

        image += "\n";
    }

    fs::write(&current_path, image).unwrap();
}

fn ray_caste(ray: &Ray, objects: &Vec<Object>, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let mut hit_record = None;
    let mut closest_so_far = f32::MAX;

    for object in objects {
        if let Some(temp_hit_record) = object.hit(ray, 0.01, closest_so_far) {
            closest_so_far = temp_hit_record.t;
            hit_record = Some(temp_hit_record);
        }
    }

    if let Some(hit_record) = hit_record {
        let (scattered, attenuation) = 
            objects[hit_record.object_id]
            .material
            .scatter(ray, &hit_record);

        if let Some(scattered) = scattered {
            return attenuation * ray_caste(&scattered, objects, depth - 1);
        }
        else {
            return attenuation;
        }
    }

    if depth == 0 {
        return SKY_COLOR;
    }

    Vec3::zero()//SKY_COLOR
}