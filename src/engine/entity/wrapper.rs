use engine::{Engine, TGraphicIndex};
use super::{EntityState, EntityTrait};

// TODO: Cache default drawing helper and draw that automatically when it has one
pub struct EntityWrapper<T: TGraphicIndex> {
    pub entity: Box<EntityTrait<T>>,
    pub state: EntityState,
    pub name: String,
}

impl<T: TGraphicIndex> EntityWrapper<T> {
    pub fn new(entity: Box<EntityTrait<T>>, engine: &Engine<T>) -> EntityWrapper<T> {
        let mut entity = entity;
        EntityWrapper {
            state: entity.get_initial_state(engine),
            name: entity.identifying_string(),
            entity: entity,
        }
    }
}
