use super::{EntityState, EntityTrait};
use {Engine, TGraphicIndex};

// TODO: Cache default drawing helper and draw that automatically when it has one
pub struct EntityWrapper<T: TGraphicIndex> {
    pub entity: Box<dyn EntityTrait<T>>,
    pub state: EntityState,
    pub name: String,
}

impl<T: TGraphicIndex> EntityWrapper<T> {
    pub fn new(entity: Box<dyn EntityTrait<T>>, engine: &Engine<T>) -> EntityWrapper<T> {
        let mut entity = entity;
        EntityWrapper {
            state: entity.get_initial_state(engine),
            name: entity.identifying_string(),
            entity,
        }
    }
}
