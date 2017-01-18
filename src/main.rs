#[macro_use]
extern crate glium;
extern crate image;
extern crate time;
extern crate rand;

mod engine;
mod entities;

use engine::Engine;

fn main() {
    let mut engine = Engine::new(1200f32, 400f32).unwrap();

    let spawner = entities::BulletSpawner::new(&engine.graphics);
    let player = entities::Player::new(&engine).unwrap();

    engine.register_entity(spawner);
    engine.register_entity(player);

    engine.run();
}
