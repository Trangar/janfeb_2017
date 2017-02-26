mod state;
mod entity_trait;
mod wrapper;

pub use self::state::EntityState;
pub use self::entity_trait::EntityTrait;
pub use self::wrapper::EntityWrapper;

use super::TGraphicIndex;

#[derive(PartialEq, Eq, Hash, Debug)]
#[allow(dead_code)]
pub enum CollisionLayer {
    Player,
    Enemy,
}

pub enum EntityEvent<T: TGraphicIndex> {
    #[allow(dead_code)]
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
