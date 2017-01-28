#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate image;
extern crate time;
extern crate rand;

mod engine;
mod entities;

use engine::Engine;

#[derive(PartialEq, Eq, Hash)]
pub enum GraphicsEnum {
    Bullet,
    Spaceship,
}

impl engine::TGraphicIndex for GraphicsEnum {}

fn main() {
    let mut engine = Engine::<GraphicsEnum>::new(1200f32, 400f32).unwrap();
    engine.graphics
        .load_graphic(GraphicsEnum::Bullet,
                      include_bytes!("../assets/bullet.png"),
                      entities::bullet::WIDTH,
                      entities::bullet::HEIGHT)
        .unwrap();
    engine.graphics
        .load_graphic(GraphicsEnum::Spaceship,
                      include_bytes!("../assets/spaceship.png"),
                      entities::player::WIDTH,
                      entities::player::HEIGHT)
        .unwrap();

    let spawner = entities::BulletSpawner::new();
    let player = entities::Player::new();

    engine.register_entity(spawner);
    engine.register_entity(player);

    engine.run();
}
