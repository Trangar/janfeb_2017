use GraphicsEnum;
use engine::*;

pub const WIDTH: f32 = 16.0;
pub const HEIGHT: f32 = 16.0;

pub struct Bullet {
    start_x: f32,
    start_y: f32,
    is_player_bullet: bool,
}

impl Bullet {
    pub fn new(x: f32, y: f32, is_player_bullet: bool) -> Bullet {
        Bullet {
            start_x: x,
            start_y: y,
            is_player_bullet: is_player_bullet,
        }
    }
}

impl EntityTrait<GraphicsEnum> for Bullet {
    fn identifying_string(&self) -> String {
        format!("Bullet")
    }
    fn collision_layers(&self) -> Option<CollisionLayer> {
        Some(
            if self.is_player_bullet { CollisionLayer::Player }
            else { CollisionLayer::Enemy }
        )
    }
    fn collides_with_layers(&self) -> Vec<CollisionLayer> {
        if self.is_player_bullet { vec![CollisionLayer::Enemy] }
        else { vec![CollisionLayer::Player] }
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
        if self.is_player_bullet {
            state.x += 0.5f32 * game_state.delta_time;
            if state.x - state.hitbox.left > game_state.screen_width {
                state.active = false;
            }
        } else {
            state.x -= 0.1f32 * game_state.delta_time;

            if state.x + state.hitbox.right < 0f32 {
                state.active = false;
            }
        }
        Vec::new()
    }

    fn collided(&mut self, state: &mut EntityState, _: &Box<EntityTrait<GraphicsEnum>>, _: &mut EntityState) -> Vec<EntityEvent<GraphicsEnum>> {
        state.active = false;
        Vec::new()
    }

    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics<GraphicsEnum>) -> Result<()> {
        graphics.draw(GraphicsEnum::Bullet, state.x, state.y, 0f32, 1f32)
    }
}
