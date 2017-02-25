use std::f32::consts::PI;
use entities::Bullet;
use GraphicsEnum;
use engine::*;

pub struct Enemy1 {
    start_x: f32,
    start_y: f32,
    health: f32,
    diagonal_direction: f32,
    bullet_target_time: f32,
    bullet_recharge_counter: f32,
}

const ONETHIRD: f32 = 1f32 / 3f32;
const TWOTHIRD: f32 = ONETHIRD * 2f32;

impl Enemy1 {
    pub fn new(x: f32, y: f32, factor: f32) -> Enemy1 {
        let bullet_target_time = 5000f32 / (factor / 2f32);
        Enemy1 {
            start_x: x,
            start_y: y,
            health: factor / 5f32,
            diagonal_direction: 0f32,
            bullet_target_time: bullet_target_time,
            bullet_recharge_counter: bullet_target_time / 2f32,
        }
    }
}

impl EntityTrait<GraphicsEnum> for Enemy1 {
    fn identifying_string(&self) -> String {
        "Enemy type 1".to_owned()
    }
    fn collision_layers(&self) -> Option<CollisionLayer> {
        Some(CollisionLayer::Enemy)
    }
    fn collides_with_layers(&self) -> Vec<CollisionLayer> {
        vec![CollisionLayer::Player]
    }
    fn get_initial_state(&mut self, engine: &Engine<GraphicsEnum>) -> EntityState {
        let hitbox = Hitbox {
            left: 25f32,
            top: 40f32,
            right: 25f32,
            bottom: 40f32,
        };
        self.diagonal_direction = if self.start_y > engine.graphics.height / 2f32 { -1f32 } else { 1f32 };
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
        

        state.x -= 0.3f32 * game_state.delta_time;

        if state.x > game_state.screen_width * ONETHIRD && state.x < game_state.screen_width * TWOTHIRD {
            state.y += self.diagonal_direction * 0.1f32 * game_state.delta_time;
        }

        let mut result = Vec::new();

        self.bullet_recharge_counter += game_state.delta_time;
        if self.bullet_recharge_counter > self.bullet_target_time {
            let bullet = Bullet::new(state.x, state.y, false);
            self.bullet_recharge_counter -= self.bullet_target_time;
            result.push(EntityEvent::SpawnEntity(Box::new(bullet)));
        }
        result
    }
    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics<GraphicsEnum>) -> Result<()> {
        graphics.draw(GraphicsEnum::Player, state.x, state.y, PI / 2f32 * 3f32, 1f32)
    }
    fn collided(&mut self,
                self_state: &mut EntityState,
                _: &Box<EntityTrait<GraphicsEnum>>,
                _: &mut EntityState)
                -> Vec<EntityEvent<GraphicsEnum>> {
        if self.health <= 1f32 {
            self_state.active = false;
        } else {
            self.health -= 1f32;
        }
        
        Vec::new()
    }
}