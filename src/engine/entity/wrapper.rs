use super::{EntityState, EntityTrait};
use engine::{Engine, TGraphicIndex};

// TODO: Cache default drawing helper and draw that automatically when it has one
pub struct EntityWrapper<T: TGraphicIndex> {
    pub entity: Box<EntityTrait<T>>,
    pub state: EntityState,
    pub name: String,
}

impl<T: TGraphicIndex> EntityWrapper<T> {
    pub fn new(entity: Box<EntityTrait<T>>, engine: &Engine<T>) -> EntityWrapper<T> {
        EntityWrapper {
            state: entity.get_initial_state(engine),
            name: entity.identifying_string(),
            entity: entity,
        }
    }
}
