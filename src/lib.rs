use nalgebra::{Vector3, Vector4};
use nalgebra_glm;
use rand::prelude::ThreadRng;
use rand::{distributions::Distribution, thread_rng, Rng};
use statrs::distribution::Normal;
use wasm_bindgen::JsValue;
use web_sys;

use wasm_bindgen::prelude::wasm_bindgen;

type Real = f64;

const NUM_SNOWFLAKES: i32 = 10000;
const BOUNDS_MIN: Vector3<Real> = Vector3::new(-100.0, -100.0, -100.0);
const BOUNDS_MAX: Vector3<Real> = Vector3::new(100.0, 100.0, 100.0);
const GRAVITY: Vector3<Real> = Vector3::new(0.0, -10.0, 0.0);
const WIND: Vector3<Real> = Vector3::new(3.0, 0.0, 0.0);
const MAX_SPEED: Real = 30.0;

/*
 * From https://rustwasm.github.io/docs/book/game-of-life/debugging.html
 */
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

fn wrap(val: Real, min: Real, max: Real) -> Real {
    if val < min {
        max
    } else if val > max {
        min
    } else {
        val
    }
}

fn wrap_pos(val: Vector3<Real>, min: Vector3<Real>, max: Vector3<Real>) -> Vector3<Real> {
    Vector3::new(
        wrap(val.x, min.x, max.x),
        wrap(val.y, min.y, max.y),
        wrap(val.z, min.z, max.z),
    )
}

#[wasm_bindgen]
// #[derive(Clone, Copy, Debug, PartialEq)]
pub struct Snowflake {
    pos: Vector3<Real>,
    vel: Vector3<Real>,
}

#[wasm_bindgen]
// #[derive(Clone, Debug)]
pub struct AppState {
    snowflakes: Vec<Snowflake>,
    rng: ThreadRng,
}

#[wasm_bindgen]
impl AppState {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let snowflakes: Vec<_> = (0..NUM_SNOWFLAKES)
            .map(|_| {
                let x = rng.gen_range(BOUNDS_MIN.x..BOUNDS_MAX.x);
                let y = rng.gen_range(BOUNDS_MIN.y..BOUNDS_MAX.y);
                let z = rng.gen_range(BOUNDS_MIN.z..BOUNDS_MAX.z);
                let pos = Vector3::new(x, y, z);
                let vel = Vector3::new(0.0, 0.0, 0.0);

                Snowflake { pos, vel }
            })
            .collect();
        Self { rng, snowflakes }
    }

    pub fn tick(&mut self, duration: Real) {
        for snowflake in self.snowflakes.iter_mut() {
            let noise_dist = Normal::new(0.0, 20.0).unwrap();
            let noise = Vector3::new(
                noise_dist.sample(&mut self.rng),
                noise_dist.sample(&mut self.rng),
                noise_dist.sample(&mut self.rng),
            );

            snowflake.vel += duration * (GRAVITY + WIND + noise);

            if snowflake.vel.norm() > MAX_SPEED {
                snowflake.vel = snowflake.vel.normalize() * MAX_SPEED;
            }

            snowflake.pos += duration * snowflake.vel;
            snowflake.pos = wrap_pos(snowflake.pos, BOUNDS_MIN, BOUNDS_MAX);
        }
    }

    pub fn draw(&self, ctx: web_sys::CanvasRenderingContext2d, width: f64, height: f64) {
        let view_matrix = nalgebra_glm::look_at(
            &Vector3::new(0.0, 0.0, -100.0),
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::y_axis(),
        );
        let near_plane = 0.1;
        let far_plane = 100.0;
        let fov = 90.0 / 180.0 * std::f64::consts::PI;
        let projection_matrix =
            nalgebra_glm::perspective(width / height, fov, near_plane, far_plane);

        let combined_matrix = projection_matrix * view_matrix;

        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill_rect(0.0, 0.0, width, height);

        ctx.set_fill_style(&JsValue::from_str("#CCCCDD"));
        for snowflake in self.snowflakes.iter() {
            let pos = Vector4::new(snowflake.pos.x, snowflake.pos.y, snowflake.pos.z, 1.0);
            let homogeneous_coords = combined_matrix * pos;

            if homogeneous_coords.z < near_plane {
                continue;
            }

            let screen_coords = homogeneous_coords / homogeneous_coords.w;

            let x = screen_coords.x * width / 2.0 + width / 2.0;
            let y = screen_coords.y * width / 2.0 + height / 2.0;
            let r = 5.0 * (1.0 - homogeneous_coords.z / far_plane).min(1.0).max(0.0);

            ctx.begin_path();
            ctx.arc(x, height - y, r, 0.0, std::f64::consts::TAU)
                .unwrap();
            ctx.fill();
        }
    }
}
