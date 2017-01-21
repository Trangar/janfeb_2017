mod state;
mod entity_trait;
mod wrapper;

pub use self::state::EntityState;
pub use self::entity_trait::EntityTrait;
pub use self::wrapper::EntityWrapper;

use super::TGraphicIndex;

pub enum EntityEvent<T: TGraphicIndex> {
    ClearAllEntities,
    SpawnEntity(Box<EntityTrait<T>>),
}


#[derive(Default)]
pub struct Hitbox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
