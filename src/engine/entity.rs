use glium::Frame;

use super::{DrawHelper, Engine, Result};

// TODO: Implement something like this:
// #[derive(PartialEq)]
// pub enum CollisionResult {
//     DestroySelf,
//     DestroyOther,
//     UpdateOtherPosition { x: f32, y: f32 },
// }

// pub enum UpdateResult {
//     DestroySelf,
//     SpawnEntity(Box<EntityTrait>),
//     UpdateHitbox(Rect),
// }

// pub trait EntityTrait {
//     fn draw(&self, engine: &Engine, frame: &mut Frame) -> Result<()>;
//     fn force_position(&mut self, x: f32, y: f32);
//     fn as_any(&self) -> &Any;

//     fn update(&mut self, delta_time: u64) -> Vec<UpdateResult> {
//         Vec::new()
//     }
//     fn get_hitbox(&self) -> Rect {
//         Rect { left: 0f32, top: 0f32, right: 0f32, bottom: 0f32 }
//     }
//     fn collided(&self, other: &EntityTrait) -> Vec<CollisionResult> {
//         Vec::new()
//     }
// }

pub struct Entity<'a> {
    pub draw_helper: &'a DrawHelper<'a>,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
    pub hitbox: Rect,
}

impl<'a> Entity<'a> {
    pub fn new_player(draw_helper: &'a DrawHelper<'a>, y: f32) -> Entity<'a> {
        let hitbox = Rect {
            left: 58f32,
            top: 29f32,
            right: 40f32,
            bottom: 29f32
        };
        Entity {
            draw_helper: draw_helper,
            x: hitbox.left,
            y: y,
            rotation: 0.0,
            scale: 1.0,
            hitbox: hitbox,
        }
    }
    pub fn new_bullet(draw_helper: &'a DrawHelper<'a>, x: f32, y: f32) -> Entity<'a> {
        let hitbox = Rect {
            left: 6f32,
            top: 6f32,
            right: 6f32,
            bottom: 6f32
        };
        Entity {
            draw_helper: draw_helper,
            x: x,
            y: y,
            rotation: 0.0,
            scale: 1.0,
            hitbox: hitbox,
        }
    }

    pub fn draw(&self, engine: &Engine, frame: &mut Frame) -> Result<()> {
        self.draw_helper.draw_at(engine, frame, self.x, self.y, self.rotation, self.scale)
    }

    pub fn intersects_with(&self, other: &Entity) -> bool {
        // check if our left hitbox is larger than the other's right hitbox
        if self.x - self.hitbox.left > other.x + other.hitbox.right { return false; }
        // check if our right hitbox is smaller than the other's left hitbox
        if self.x + self.hitbox.right < other.x - other.hitbox.left { return false; }
        // check if our top hitbox is larger than the other's bottom hitbox
        if self.y - self.hitbox.top > other.y + other.hitbox.bottom { return false; }
        // check if our bottom hitbox is smaler than the other's top hitbox
        if self.y + self.hitbox.bottom < other.y - other.hitbox.top { return false; }

        // if the statements above are false, we have a collision
        true
    }
}

pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}