use engine::*;

// const PLAYER_FIRE_POINTS: [[f32;2];2] = [[-10.0, -25.0],[-10.0,25.0]];
const PLAYER_WIDTH: f32 = 128.0;
const PLAYER_HEIGHT: f32 = 64.0;
const HORIZONTAL_SPEED: f32 = 0.15f32;
const VERTICAL_SPEED: f32 = 0.2f32;

pub struct Player {
    pub drawable: DrawHelper,
    pub last_bullet_time: f32,
    pub last_bullet_position_index: u8,
}

impl Player {
    pub fn new(engine: &Engine) -> Result<Player> {
        let drawable = DrawHelper::new(&engine.graphics,
                                       PLAYER_WIDTH,
                                       PLAYER_HEIGHT,
                                       &include_bytes!("../../assets/spaceship.png")[..])?;

        Ok(Player {
            drawable: drawable,
            last_bullet_time: 0f32,
            last_bullet_position_index: 0,
        })
    }
}

impl EntityTrait for Player {
    fn get_initial_state(&self, engine: &Engine) -> EntityState {
        let hitbox = Hitbox {
            left: 58f32,
            top: 29f32,
            right: 40f32,
            bottom: 29f32,
        };
        EntityState {
            x: hitbox.left,
            y: engine.graphics.height / 2f32,
            hitbox: hitbox,
            ..EntityState::default()
        }
    }

    fn update(&mut self, game_state: &mut GameState, state: &mut EntityState) -> Vec<UpdateResult> {
        let mut x = 0f32;
        let mut y = 0f32;

        if game_state.keyboard.is_keydown(VirtualKeyCode::A) {
            x -= 1f32;
        }
        if game_state.keyboard.is_keydown(VirtualKeyCode::D) {
            x += 1f32;
        }
        if game_state.keyboard.is_keydown(VirtualKeyCode::S) {
            y += 1f32;
        }
        if game_state.keyboard.is_keydown(VirtualKeyCode::W) {
            y -= 1f32;
        }

        state.x += x * HORIZONTAL_SPEED * game_state.delta_time;
        state.y += y * VERTICAL_SPEED * game_state.delta_time;

        Vec::new()
    }

    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics) -> Result<()> {
        self.drawable.draw_at(graphics, state.x, state.y, 0f32, 1f32)
    }
}
