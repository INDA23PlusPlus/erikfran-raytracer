const SAMPLES_PER_PIXEL: u32 = 4000000000u;
const MAX_DEPTH: u32 = 50u;
const GLOBAL_ILUMINATION_COLOR: vec3f = vec3f(0.0, 0.0, 0.0);
const VIEWPORT_DISTANCE: f32 = 1.0;
const CAMERA_POSITION: vec3f = vec3f(0.0, 0.0, 0.0);
const WIDTH: u32 = 2048u;
const HEIGHT: u32 = WIDTH;
const SKY_COLOR: vec3f = vec3f(0.0, 0.0, 0.0);
const OBJECT_COUNT: u32 = 4u;

struct World {
    spheres: array<Sphere, OBJECT_COUNT>,
}

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    id: u32,
    material: Material,
}

struct Material {
    color: vec3<f32>,
    is_light: bool,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct Image {
    pixels: array<vec3f>,
}

var<private> world: World;

@group(0) @binding(0) var<storage, read_write> output: Image;

@compute
@workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>
    ) {
    world = World(
        array(
            Sphere(
                vec3<f32>(0.0, 0.0, 4.0),
                0.7,
                0u,
                Material(
                    vec3<f32>(0.5, 0.5, 0.5),
                    false,
                ),
            ),
            Sphere(
                vec3<f32>(-2.0, 0.5, 4.0),
                0.7,
                1u,
                Material(
                    vec3<f32>(10.0, 10.0, 10.0),
                    true,
                )
            ),
            Sphere(
                vec3<f32>(0.0, 1.7, 4.0),
                0.7,
                2u,
                Material(
                    vec3<f32>(10.0, 10.0, 10.0),
                    true,
                ),
            ),
            Sphere(
                vec3<f32>(100.75, 0.0, 4.0),
                100.0,
                3u,
                Material(
                    vec3<f32>(0.5, 1.0, 0.3),
                    false,
                ),
            ),
        ),
    );

    let viewport_x = (f32(local_id.x) - f32(WIDTH) / 2.0) / f32(WIDTH);
    let viewport_y = (f32(local_id.y) - f32(HEIGHT) / 2.0) / f32(HEIGHT);

    var acc = vec3<f32>(0.0, 0.0, 0.0);

    let global_index = global_id.x + global_id.y * WIDTH;

    for (var i = 0u; i < SAMPLES_PER_PIXEL; i++) {
        for (var i = 0u; i < SAMPLES_PER_PIXEL; i++) {
            let ray = Ray(
                CAMERA_POSITION,
                vec3f(
                    viewport_x - (0.5 + rand()) / f32(WIDTH),
                    viewport_y - (0.5 + rand()) / f32(HEIGHT),
                    VIEWPORT_DISTANCE,
                )
            );
            
            acc += ray_caste(ray);
        }
    }

    output.pixels[global_index] = acc / f32(SAMPLES_PER_PIXEL);
}

fn ray_caste(ray : Ray) -> vec3f {
    var current_color = vec3f(0.0, 0.0, 0.0);
    var current_ray = ray;

    for (var depth = 0u; depth < MAX_DEPTH; depth++) {
        var has_hit: bool = false;
        var hit_record: HitRecord;
        var closest_so_far: f32 = 0.0;
        let t_min: f32 = 0.01;

        for (var i: u32 = 0u; i < OBJECT_COUNT; i++) {
            let object = world.spheres[i];

            let oc = current_ray.origin - object.center;
            let a = dot(current_ray.direction, current_ray.direction);
            let b = dot(oc, current_ray.direction);
            let c = dot(oc, oc) - pow(object.radius, 2.0);
            let discriminant = pow(b, 2.0) - a * c;

            if discriminant > 0.0 {
                var t = (-b - sqrt(discriminant)) / a;

                if t < closest_so_far && t > t_min {
                    let point = ray_at(current_ray, t);
                    let normal = (point - object.center) / object.radius;

                    hit_record = HitRecord(point, normal, t, object.id);
                    closest_so_far = t;
                    has_hit = true;
                    break;
                }
            }
        }

        if has_hit {
            let material = world.spheres[hit_record.object_id].material;

            if material.is_light {
                current_color = material.color;
            } else {
                let scattered = diffuse_scatter(current_ray, hit_record, material);
                current_color = material.color * current_color;
            }
        }
        else {
            if depth == 0u {
                current_color = SKY_COLOR;
            }
            else {
                current_color = GLOBAL_ILUMINATION_COLOR;
            }
        }
    }

    return current_color;
}

struct HitRecord {
    point: vec3f,
    normal: vec3f,
    t: f32,
    object_id: u32,
}

fn diffuse_scatter(ray: Ray, hit_record: HitRecord, material: Material) -> Ray {
    return Ray(
        hit_record.point,
        hit_record.normal + random_unit_vector(),
    );
}

fn random_unit_vector() -> vec3f {
    return normalize(vec3f(
        rand() * 2.0 - 1.0,
        rand() * 2.0 - 1.0,
        rand() * 2.0 - 1.0,
    ));
}

var<private> seed: f32 = 0.0;

fn rand() -> f32 {
    seed += 1.0;
    return fract(sin(seed)*1.0);
}

fn ray_at(ray: Ray, t: f32) -> vec3f {
    return ray.origin + ray.direction * t;
}