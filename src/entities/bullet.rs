use GraphicsEnum;
use engine::*;

pub const WIDTH: f32 = 16.0;
pub const HEIGHT: f32 = 16.0;

pub struct Bullet {
    start_x: f32,
    start_y: f32,
    id: u64,
}

impl Bullet {
    pub fn new(x: f32, y: f32, id: u64) -> Bullet {
        Bullet {
            start_x: x,
            start_y: y,
            id: id,
        }
    }
}

impl EntityTrait<GraphicsEnum> for Bullet {
    fn identifying_string(&self) -> String {
        format!("Bullet {}", self.id)
    }
    fn get_initial_state(&self, _: &Engine<GraphicsEnum>) -> EntityState {
        let hitbox = Hitbox {
            left: 6f32,
            top: 6f32,
            right: 6f32,
            bottom: 6f32,
        };
        EntityState {
            x: self.start_x,
            y: self.start_y,
            hitbox: hitbox,
            ..EntityState::default()
        }
    }

    fn update(&mut self,
              game_state: &mut GameState,
              state: &mut EntityState)
              -> Vec<EntityEvent<GraphicsEnum>> {
        state.x -= 0.1f32 * game_state.delta_time;

        if state.x + state.hitbox.right < 0f32 {
            state.active = false;
        }
        Vec::new()
    }

    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics<GraphicsEnum>) -> Result<()> {
        graphics.draw(GraphicsEnum::Bullet, state.x, state.y, 0f32, 1f32)
    }
}
