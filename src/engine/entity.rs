use super::{Result, Engine, EngineGraphics, KeyboardState};
use rand::{ThreadRng, Rng};
use std::any::Any;

// TODO: Implement something like this:
#[derive(PartialEq)]
pub enum CollisionResult {
}

pub enum UpdateResult {
    SpawnEntity(Box<EntityTrait>),
}

pub trait EntityCastTrait {
    fn as_type<T: Any>(&self) -> Option<&Box<&T>>;
}
impl EntityCastTrait for Box<EntityTrait> {
    fn as_type<T: Any>(&self) -> Option<&Box<&T>> {
        let any = self as &Any;
        any.downcast_ref::<Box<&T>>()
    }
}

pub trait EntityTrait {
    fn draw(&self, _state: &EntityState, _graphics: &mut EngineGraphics) -> Result<()> {
        Ok(())
    }
    fn get_initial_state(&self, _engine: &Engine) -> EntityState {
        EntityState::default()
    }
    fn update(&mut self,
              _game_state: &mut GameState,
              _entity_state: &mut EntityState)
              -> Vec<UpdateResult> {
        Vec::new()
    }
    fn collided(&self,
                _self_state: &EntityState,
                _other: &Box<EntityTrait>,
                _other_state: &EntityState)
                -> Vec<CollisionResult> {
        Vec::new()
    }

    fn intersects_with(&self,
                       self_state: &EntityState,
                       _other: &Box<EntityTrait>,
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

pub struct GameState<'a> {
    pub delta_time: f32,
    pub keyboard: &'a KeyboardState,
    pub screen_width: f32,
    pub screen_height: f32,
    pub rng: &'a mut ThreadRng,
}

impl<'a> GameState<'a> {
    pub fn rand_f32(&mut self) -> f32 {
        self.rng.next_f32()
    }
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
    pub fn new(entity: Box<EntityTrait>, engine: &Engine) -> EntityWrapper {
        EntityWrapper {
            state: entity.get_initial_state(engine),
            entity: entity,
        }
    }
}

#[derive(Default)]
pub struct Hitbox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
