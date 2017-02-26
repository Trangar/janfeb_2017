use super::KeyboardState;
use rand::{ThreadRng, Rng};

pub struct GameState<'a> {
    pub delta_time: f32,
    pub keyboard: &'a KeyboardState,
    pub screen_width: f32,
    pub screen_height: f32,
    pub rng: &'a mut ThreadRng,
}

impl<'a> GameState<'a> {
    #[allow(dead_code)]
    pub fn rand_f32(&mut self) -> f32 {
        self.rng.next_f32()
    }
}
