use engine::*;

pub struct YouLost {
    pub drawable: DrawHelper,
}

const WIDTH: f32 = 128f32;
const HEIGHT: f32 = 64f32;

impl YouLost {
    pub fn new() -> Result<YouLost> {
        Ok(YouLost {
            drawable: DrawHelper::new(get_engine().unwrap().graphics, WIDTH, HEIGHT)?
        })
    }
}

impl EntityTrait for YouLost {
    fn identifying_string(&self) -> String { "You lost!".to_owned() }

    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics) -> Result<()> {
        self.drawable.draw_at(graphics, (graphics.width - WIDTH) / 2f32, (graphics.height - HEIGHT) / 2f32, 0f32, 1f32)
    }
}