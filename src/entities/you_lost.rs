use get_initial_state;
use GraphicsEnum;
use engine::*;

pub struct YouLost {
}

pub const WIDTH: f32 = 128f32;
pub const HEIGHT: f32 = 64f32;

impl YouLost {
    pub fn new() -> Result<YouLost> {
        Ok(YouLost {
        })
    }
}

impl EntityTrait<GraphicsEnum> for YouLost {
    fn identifying_string(&self) -> String { "You lost!".to_owned() }

    fn draw(&self, _: &EntityState, graphics: &mut EngineGraphics<GraphicsEnum>) -> Result<()> {
        let x = graphics.width / 2f32;
        let y = graphics.height / 2f32;
        graphics.draw(GraphicsEnum::YouLost, x, y, 0.0f32, 1.0f32)
    }

    fn update(&mut self,
              game_state: &mut GameState,
              state: &mut EntityState)
              -> Vec<EntityEvent<GraphicsEnum>> {
        if game_state.keyboard.is_pressed_this_frame(VirtualKeyCode::Space) {
            state.active = false;
            
            let mut response = get_initial_state().into_iter().map(|s| EntityEvent::SpawnEntity(s)).collect::<Vec<_>>();
            response.insert(0, EntityEvent::ClearAllEntities);
            response
        } else {
            Vec::new()
        }
    }
}