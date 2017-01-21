use engine::*;
//use super::YouLost;

// const PLAYER_FIRE_POINTS: [[f32;2];2] = [[-10.0, -25.0],[-10.0,25.0]];
const PLAYER_WIDTH: f32 = 128.0;
const PLAYER_HEIGHT: f32 = 64.0;
const HORIZONTAL_SPEED: f32 = 0.15f32;
const VERTICAL_SPEED: f32 = 0.2f32;

pub struct Player {
    pub drawable: DrawHelper,
    pub last_bullet_time: f32,
    pub last_bullet_position_index: u8,
    pub health: u64,
    pub max_health: u64,
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
            health: 10,
            max_health: 10,
        })
    }
}

impl EntityTrait for Player {
    fn identifying_string(&self) -> String { "Player".to_owned() }
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

    fn update(&mut self, game_state: &mut GameState, state: &mut EntityState) -> Vec<EntityEvent> {
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

        clamp_to(&mut state.x, state.hitbox.left, game_state.screen_width - state.hitbox.right);
        clamp_to(&mut state.y, state.hitbox.top, game_state.screen_height - state.hitbox.bottom);

        Vec::new()
    }

    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics) -> Result<()> {
        self.drawable.draw_at(graphics, state.x, state.y, 0f32, 1f32)?;

        let health_factor = (self.health as f32) / (self.max_health as f32);
        let HEALTHBAR_OFFSET: (f32, f32) = (-state.hitbox.left, state.hitbox.bottom + 10f32);
        let HEALTHBAR_SIZE: (f32, f32) = (state.hitbox.left + state.hitbox.right, 5f32);
        const COLOR_GREEN: (f32, f32, f32, f32) = (0.0, 1.0, 0.0, 1.0);
        const COLOR_RED: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
        graphics.draw_rectangle(
            state.x + HEALTHBAR_OFFSET.0,
            state.y + HEALTHBAR_OFFSET.1,
            HEALTHBAR_SIZE.0 * health_factor, 
            HEALTHBAR_SIZE.1,
            COLOR_GREEN
        )?;
        graphics.draw_rectangle(
            state.x + HEALTHBAR_OFFSET.0 + HEALTHBAR_SIZE.0 * health_factor,
            state.y + HEALTHBAR_OFFSET.1,
            HEALTHBAR_SIZE.0 - HEALTHBAR_SIZE.0 * health_factor,
            HEALTHBAR_SIZE.1,
            COLOR_RED
        )?;
        Ok(())
    }

    fn collided(&mut self,
                self_state: &mut EntityState,
                _other: &Box<EntityTrait>,
                other_state: &mut EntityState,
                _graphics: &EngineGraphics)
                -> Vec<EntityEvent> {
        self.health -= 1;
        other_state.active = false;
        
        if self.health == 0 {
            self_state.active = false;
            //let you_lost = Box::new(YouLost::new().unwrap());
            //vec![
            //    EntityEvent::SpawnEntity(you_lost)
            //]
        } //else {
            Vec::new()
        //}
    }
}

fn clamp_to(value: &mut f32, min: f32, max: f32) {
    if *value < min { *value = min; }
    if *value > max { *value = max; }
}