use std::time::Instant;

use kiss3d::{light::Light, scene::SceneNode, window::Window};
use nalgebra::Translation3;
use nalgebra::Vector3;
use rand::{distributions::Distribution, thread_rng, Rng};
use statrs::distribution::Normal;

const NUM_SNOWFLAKES: i32 = 1000;
const RADIUS: f32 = 0.1;
const BOUNDS_MIN: Vector3<f32> = Vector3::new(-100.0, -100.0, -100.0);
const BOUNDS_MAX: Vector3<f32> = Vector3::new(100.0, 100.0, 100.0);
const GRAVITY: Vector3<f32> = Vector3::new(0.0, -10.0, 0.0);
const WIND: Vector3<f32> = Vector3::new(3.0, 0.0, 0.0);
const MAX_SPEED: f32 = 30.0;

fn wrap(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        max
    } else if val > max {
        min
    } else {
        val
    }
}

fn wrap_trans(val: Translation3<f32>, min: Vector3<f32>, max: Vector3<f32>) -> Translation3<f32> {
    Vector3::new(
        wrap(val.x, min.x, max.x),
        wrap(val.y, min.y, max.y),
        wrap(val.z, min.z, max.z),
    )
    .into()
}

struct Snowflake {
    node: SceneNode,
    vel: Vector3<f32>,
}

fn main() {
    let mut window = Window::new("Snow animation");
    window.set_light(Light::StickToCamera);
    // window.set_background_color(0.7, 0.7, 0.9);

    let mut rng = thread_rng();

    let mut snowflakes: Vec<_> = (0..NUM_SNOWFLAKES)
        .map(|_| {
            let mut node = window.add_sphere(RADIUS);
            let x = rng.gen_range(BOUNDS_MIN.x..BOUNDS_MAX.x);
            let y = rng.gen_range(BOUNDS_MIN.y..BOUNDS_MAX.y);
            let z = rng.gen_range(BOUNDS_MIN.z..BOUNDS_MAX.z);
            node.set_local_translation(Vector3::new(x, y, z).into());
            node.set_color(1.0, 1.0, 1.0);
            node.set_lines_color(None);

            let vel = Vector3::new(0.0, 0.0, 0.0);

            Snowflake { node, vel }
        })
        .collect();

    let mut prev_time = Instant::now();

    while window.render() {
        let this_time = Instant::now();
        let duration_ms = (this_time - prev_time).as_millis();
        prev_time = this_time;

        let duration = duration_ms as f32 / 1000.0;

        for snowflake in snowflakes.iter_mut() {
            let noise_dist = Normal::new(0.0, 20.0).unwrap();
            let noise = Vector3::new(
                noise_dist.sample(&mut rng) as f32,
                noise_dist.sample(&mut rng) as f32,
                noise_dist.sample(&mut rng) as f32,
            );

            snowflake.vel += duration * (GRAVITY + WIND + noise);

            if snowflake.vel.norm() > MAX_SPEED {
                snowflake.vel = snowflake.vel.normalize() * MAX_SPEED;
            }

            snowflake
                .node
                .append_translation(&(duration * snowflake.vel).into());

            let current_trans = snowflake.node.data().local_translation();
            let wrapped_trans = wrap_trans(current_trans, BOUNDS_MIN, BOUNDS_MAX);
            snowflake.node.set_local_translation(wrapped_trans);
        }
    }
}
