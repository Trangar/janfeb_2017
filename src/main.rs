#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate image;
extern crate time;
extern crate rand;

mod engine;
mod entities;

use engine::{Engine, EntityTrait};

#[derive(PartialEq, Eq, Hash)]
pub enum GraphicsEnum {
    Bullet,
    Spaceship,
    YouLost,
}

fn get_initial_state() -> Vec<Box<EntityTrait<GraphicsEnum>>> {
    let spawner = entities::BulletSpawner::new();
    let player = entities::Player::new();
    vec![
        Box::new(spawner),
        Box::new(player)
    ]
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
    engine.graphics
        .load_graphic(GraphicsEnum::YouLost,
                      include_bytes!("../assets/you_lost.png"),
                      entities::you_lost::WIDTH,
                      entities::you_lost::HEIGHT)
        .unwrap();

    for entity in get_initial_state().into_iter(){
        engine.register_entity(entity);
    }

    engine.run();
}
