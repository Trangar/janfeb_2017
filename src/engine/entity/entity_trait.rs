use engine::{Engine, EngineGraphics, GameState, TGraphicIndex, Result};
use super::{CollisionLayer, EntityEvent, EntityState};

pub trait EntityTrait<T: TGraphicIndex> {
    fn identifying_string(&self) -> String;
    fn default_graphic(&self) -> Option<T> { None }
    fn collision_layers(&self) -> Option<CollisionLayer> { None }
    fn collides_with_layers(&self) -> Vec<CollisionLayer> { Vec::new() }
    fn draw(&self, _state: &EntityState, _graphics: &mut EngineGraphics<T>) -> Result<()> {
        Ok(())
    }
    fn get_initial_state(&self, _engine: &Engine<T>) -> EntityState {
        EntityState::default()
    }
    fn update(&mut self,
              _game_state: &mut GameState,
              _entity_state: &mut EntityState)
              -> Vec<EntityEvent<T>> {
        Vec::new()
    }
    fn collided(&mut self,
                _self_state: &mut EntityState,
                _other: &Box<EntityTrait<T>>,
                _other_state: &mut EntityState)
                -> Vec<EntityEvent<T>> {
        Vec::new()
    }

    fn intersects_with(&self,
                       self_state: &EntityState,
                       _other: &Box<EntityTrait<T>>,
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
