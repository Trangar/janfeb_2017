use super::Hitbox;

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
