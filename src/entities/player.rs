use GraphicsEnum;
use engine::*;
use super::{Bullet,YouLost};

const PLAYER_FIRE_POINTS: [[f32;2];2] = [[-10.0, -25.0],[-10.0,25.0]];
const PLAYER_FIRE_INTERVAL: f32 = 200f32;

pub const WIDTH: f32 = 128.0;
pub const HEIGHT: f32 = 64.0;
const HORIZONTAL_SPEED: f32 = 0.15f32;
const VERTICAL_SPEED: f32 = 0.2f32;

pub struct Player {
    pub last_bullet_time: f32,
    pub last_bullet_position_index: u8,
    pub health: u64,
    pub max_health: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            last_bullet_time: 0f32,
            last_bullet_position_index: 0,
            health: 10,
            max_health: 10,
        }
    }
}

const COLOR_WHITE: Color = (1.0, 1.0, 1.0, 1.0);
const COLOR_GREEN: Color = (0.0, 1.0, 0.0, 1.0);
const COLOR_RED: Color = (1.0, 0.0, 0.0, 1.0);

impl EntityTrait<GraphicsEnum> for Player {
    fn identifying_string(&self) -> String {
        "Player".to_owned()
    }
    fn collision_layers(&self) -> Option<CollisionLayer> {
        Some(CollisionLayer::Player)
    }
    fn collides_with_layers(&self) -> Vec<CollisionLayer> {
        vec![CollisionLayer::Enemy]
    }
    fn get_initial_state(&self, engine: &Engine<GraphicsEnum>) -> EntityState {
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

    fn update(&mut self,
              game_state: &mut GameState,
              state: &mut EntityState)
              -> Vec<EntityEvent<GraphicsEnum>> {
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

        clamp_to(&mut state.x,
                 state.hitbox.left,
                 game_state.screen_width - state.hitbox.right);
        clamp_to(&mut state.y,
                 state.hitbox.top,
                 game_state.screen_height - state.hitbox.bottom);

        let mut result = Vec::new();
        if self.last_bullet_time < game_state.delta_time {
            self.last_bullet_time = PLAYER_FIRE_INTERVAL;
            let position = PLAYER_FIRE_POINTS[self.last_bullet_position_index as usize];
            self.last_bullet_position_index = (self.last_bullet_position_index + 1) % 2;

            let bullet = Bullet::new(state.x + position[0], state.y + position[1], true);
            result.push(EntityEvent::SpawnEntity(Box::new(bullet)));
        } else {
            self.last_bullet_time -= game_state.delta_time;
        }

        result
    }

    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics<GraphicsEnum>) -> Result<()> {
        graphics.draw(GraphicsEnum::Spaceship, state.x, state.y, 0f32, 1f32)?;

        let health_factor = (self.health as f32) / (self.max_health as f32);
        let healthbar_offset: (f32, f32) = (-state.hitbox.left * 0.75, -(state.hitbox.top + 10f32));
        let healthbar_size: (f32, f32) = ((state.hitbox.left + state.hitbox.right) * 0.5, 5f32);
        graphics.draw_rectangle(state.x + healthbar_offset.0,
                            state.y + healthbar_offset.1,
                            healthbar_size.0 * health_factor,
                            healthbar_size.1,
                            COLOR_GREEN)?;
        graphics.draw_rectangle(state.x + healthbar_offset.0 + healthbar_size.0 * health_factor,
                            state.y + healthbar_offset.1,
                            healthbar_size.0 - healthbar_size.0 * health_factor,
                            healthbar_size.1,
                            COLOR_RED)?;

        graphics.draw_text_at("Hello".to_owned(),
                          state.x + healthbar_offset.0,
                          state.y + healthbar_offset.1 - 15f32,
                          COLOR_WHITE)?;
        Ok(())
    }

    fn collided(&mut self,
                self_state: &mut EntityState,
                _other: &Box<EntityTrait<GraphicsEnum>>,
                other_state: &mut EntityState)
                -> Vec<EntityEvent<GraphicsEnum>> {
        self.health -= 1;
        other_state.active = false;

        if self.health == 0 {
            self_state.active = false;
            let you_lost = Box::new(YouLost::new().unwrap());
            vec![
                EntityEvent::SpawnEntity(you_lost)
            ]
        } else {
            Vec::new()
        }
    }
}

fn clamp_to(value: &mut f32, min: f32, max: f32) {
    if *value < min {
        *value = min;
    }
    if *value > max {
        *value = max;
    }
}
