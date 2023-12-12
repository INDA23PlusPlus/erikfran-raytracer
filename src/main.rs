use std::{fs, path::Path};
use crate::math::*;
use crate::ray::*;
use crate::primitives::*;

mod math;
mod ray;
mod primitives;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const VIEWPORT_DISTANCE: f32 = 1.0;

fn main() {
    let mut current = 0;
    let mut current_path = format!("images/image{}.ppm", current);
    while Path::new(&current_path).exists() {
        current += 1;
        current_path = format!("images/image{}.ppm", current);
    }

    let mut image = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);

    let camera_position = Vec3::from(0.0, 0.0, 0.0);
    let sphere = Sphere::from(Vec3::from(0.0, 0.5, 4.0), 0.5);
    let plane = Plane::from(Vec3::from(0.5, 0.0, 0.0), Vec3::from(-1.0, 0.0, 0.0).normalized());
    let light_pos = Vec3::from(-6.0, -6.0, 2.0);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let viewport_x = (x as f32 - WIDTH as f32 / 2.0) / WIDTH as f32;
            let viewport_y = (y as f32 - HEIGHT as f32 / 2.0) / HEIGHT as f32;

            let ray = Ray::from(
                camera_position.clone(),
                Vec3::from(viewport_x, viewport_y, VIEWPORT_DISTANCE).normalized(),
            );

            let mut hit = None;

            if let Some(hit_record) = plane.hit(&ray, 0.0, std::f32::MAX) {
                hit = Some(hit_record);
            }
            if let Some(hit_record) = sphere.hit(&ray, 0.0, std::f32::MAX) {
                hit = Some(hit_record);
            }

            let mut color = Vec3::from(0.5, 0.7, 1.0);
            if let Some(hit_record) = hit {
                let light_direction = light_pos - (hit_record.point + hit_record.normal * 0.1);
                if let Some(light_hit) = sphere.hit(&Ray::from(hit_record.point, light_direction.normalized()), 0.0, light_direction.length()) {
                    color = Vec3::from(0.0, 0.0, 0.0);
                }
                else {
                    color = Vec3::from(1.0, 1.0, 1.0) * (hit_record.normal.cross(&light_direction).length() * 0.5 + 0.5);
                }
            }

            let r = (color.x * 255.0) as u32;
            let g = (color.y * 255.0) as u32;
            let b = (color.z * 255.0) as u32;

            image += &format!("{} {} {} ", r, g, b);
        }

        image += "\n";
    }

    fs::write(&current_path, image).unwrap();
}
