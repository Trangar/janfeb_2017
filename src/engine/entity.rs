use rand::ThreadRng;
use super::{Result, EngineGraphics, KeyboardState};

// TODO: Implement something like this:
#[derive(PartialEq)]
pub enum CollisionResult {
}

#[derive(Debug)]
pub enum UpdateResult {
    SpawnEntity(Box<EntityTrait>),
}

pub trait EntityTrait: ::std::fmt::Debug {
    fn draw(&self, _state: &EntityState, _graphics: &mut EngineGraphics) -> Result<()> {
        Ok(())
    }
    fn get_initial_state(&self) -> EntityState {
        EntityState::default()
    }
    fn update(&mut self, _state: &mut EntityUpdateState) -> Vec<UpdateResult> {
        Vec::new()
    }
    fn collided(&self,
                _self_state: &EntityState,
                _other: Box<EntityTrait>,
                _other_state: &EntityState)
                -> Vec<CollisionResult> {
        Vec::new()
    }

    fn intersects_with(&self,
                       self_state: &EntityState,
                       _other: Box<EntityTrait>,
                       other_state: &EntityState)
                       -> bool {
        // check if our left hitbox is larger than the other's right hitbox
        if self_state.x - self_state.hitbox.left > other_state.x + other_state.hitbox.right {
            return false;
        }
        // check if our right hitbox is smaller than the other's left hitbox
        if self_state.x + self_state.hitbox.right < other_state.x - other_state.hitbox.left {
            return false;
        }
        // check if our top hitbox is larger than the other's bottom hitbox
        if self_state.y - self_state.hitbox.top > other_state.y + other_state.hitbox.bottom {
            return false;
        }
        // check if our bottom hitbox is smaler than the other's top hitbox
        if self_state.y + self_state.hitbox.bottom < other_state.y - other_state.hitbox.top {
            return false;
        }

        // if the statements above are false, we have a collision
        true
    }
}

pub struct EntityUpdateState<'a> {
    pub state: &'a mut EntityState,
    pub delta_time: f32,
    pub keyboard_state: &'a KeyboardState,
    pub screen_width: f32,
    pub screen_height: f32,
    pub rng: &'a mut ThreadRng,
}

pub struct EntityWrapper {
    pub entity: Box<EntityTrait>,
    pub state: EntityState,
}

pub struct EntityState {
    pub active: bool,
    pub hitbox: Hitbox,
    pub x: f32,
    pub y: f32,
}

impl Default for EntityState {
    fn default() -> EntityState {
        EntityState {
            active: true,
            hitbox: Default::default(),
            x: 0f32,
            y: 0f32,
        }
    }
}

impl EntityWrapper {
    pub fn new(entity: Box<EntityTrait>) -> EntityWrapper {
        EntityWrapper {
            state: entity.get_initial_state(),
            entity: entity,
        }
    }
}

// pub struct Entity<'a> {
// pub draw_helper: &'a DrawHelper<'a>,
// pub x: f32,
// pub y: f32,
// pub rotation: f32,
// pub scale: f32,
// pub hitbox: Rect,
// }
//
// impl<'a> Entity<'a> {
// pub fn new_player(draw_helper: &'a DrawHelper<'a>, y: f32) -> Entity<'a> {
// let hitbox = Rect {
// left: 58f32,
// top: 29f32,
// right: 40f32,
// bottom: 29f32
// };
// Entity {
// draw_helper: draw_helper,
// x: hitbox.left,
// y: y,
// rotation: 0.0,
// scale: 1.0,
// hitbox: hitbox,
// }
// }
// pub fn new_bullet(draw_helper: &'a DrawHelper<'a>, x: f32, y: f32) -> Entity<'a> {
// let hitbox = Rect {
// left: 6f32,
// top: 6f32,
// right: 6f32,
// bottom: 6f32
// };
// Entity {
// draw_helper: draw_helper,
// x: x,
// y: y,
// rotation: 0.0,
// scale: 1.0,
// hitbox: hitbox,
// }
// }
//
// pub fn draw(&self, engine: &Engine, frame: &mut Frame) -> Result<()> {
// self.draw_helper.draw_at(engine, frame, self.x, self.y, self.rotation, self.scale)
// }
//
// pub fn intersects_with(&self, other: &Entity) -> bool {
// check if our left hitbox is larger than the other's right hitbox
// if self.x - self.hitbox.left > other.x + other.hitbox.right { return false; }
// check if our right hitbox is smaller than the other's left hitbox
// if self.x + self.hitbox.right < other.x - other.hitbox.left { return false; }
// check if our top hitbox is larger than the other's bottom hitbox
// if self.y - self.hitbox.top > other.y + other.hitbox.bottom { return false; }
// check if our bottom hitbox is smaler than the other's top hitbox
// if self.y + self.hitbox.bottom < other.y - other.hitbox.top { return false; }
//
// if the statements above are false, we have a collision
// true
// }
// }
//

#[derive(Default)]
pub struct Hitbox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
