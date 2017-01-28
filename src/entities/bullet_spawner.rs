use super::Bullet;
use GraphicsEnum;
use engine::*;

const BULLET_SPAWN_INTERVAL: f32 = 100f32;

pub struct BulletSpawner {
    pub time_counter: f32,
    pub next_bullet_id: u64,
}

impl BulletSpawner {
    pub fn new() -> BulletSpawner {
        BulletSpawner {
            time_counter: BULLET_SPAWN_INTERVAL,
            next_bullet_id: 1,
        }
    }
}

impl EntityTrait<GraphicsEnum> for BulletSpawner {
    fn identifying_string(&self) -> String {
        "Bullet spawner".to_owned()
    }
    fn update(&mut self,
              game_state: &mut GameState,
              _: &mut EntityState)
              -> Vec<EntityEvent<GraphicsEnum>> {
        self.time_counter += game_state.delta_time;
        let mut result = Vec::new();
        while self.time_counter > BULLET_SPAWN_INTERVAL {
            self.time_counter -= BULLET_SPAWN_INTERVAL;
            let height = (game_state.screen_height) * game_state.rand_f32();
            let bullet = Bullet::new(game_state.screen_width, height, self.next_bullet_id);
            result.push(EntityEvent::SpawnEntity(Box::new(bullet)));

            self.next_bullet_id += 1;
        }
        result
    }
}
