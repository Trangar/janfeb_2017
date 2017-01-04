use glium::Frame;

use super::{DrawHelper, Engine, Result};

pub struct Entity<'a> {
    pub draw_helper: &'a DrawHelper<'a>,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
    pub hitbox: Rect,
}

impl<'a> Entity<'a> {
    pub fn new_player(draw_helper: &'a DrawHelper<'a>) -> Entity<'a> {
        let hitbox = Rect {
            left: 58f32,
            top: 29f32,
            right: 40f32,
            bottom: 29f32
        };
        Entity {
            draw_helper: draw_helper,
            x: 10000f32, //hitbox.left,
            y: 10000f32, //hitbox.top,
            rotation: 0.0,
            scale: 1.0,
            hitbox: hitbox,
        }
    }

    pub fn draw(&self, engine: &Engine, frame: &mut Frame) -> Result<()> {
        self.draw_helper.draw_at(engine, frame, self.x, self.y, self.rotation, self.scale)
    }
}

pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}