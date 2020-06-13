use super::{Enemy1, Enemy2}; //, Enemy3};
use engine::*;
use rand::{prelude::StdRng, Rng, SeedableRng};
use GraphicsEnum;

pub struct EnemySpawner {
    pub time_counter: f32,
    pub target_time_counter: f32,
    pub enemy_amount_to_spawn: u32,
    pub rng: StdRng,
}

impl EnemySpawner {
    pub fn new() -> EnemySpawner {
        EnemySpawner {
            time_counter: 0f32,
            target_time_counter: 3000f32,
            enemy_amount_to_spawn: 3,
            rng: StdRng::seed_from_u64(0),
        }
    }

    fn generate_time_target(&mut self) {
        let factor = 5000f32 - ((self.enemy_amount_to_spawn - 3) * 10) as f32;
        self.target_time_counter = factor + self.rng.gen::<f32>() * factor;
    }
}

impl EntityTrait<GraphicsEnum> for EnemySpawner {
    fn identifying_string(&self) -> String {
        "Enemy spawner".to_owned()
    }
    fn update(
        &mut self,
        game_state: &mut GameState,
        _: &mut EntityState,
    ) -> Vec<EntityEvent<GraphicsEnum>> {
        self.time_counter += game_state.delta_time;
        let mut result = Vec::new();
        while self.time_counter > self.target_time_counter {
            self.time_counter -= self.target_time_counter;
            let height = (game_state.screen_height) * self.rng.gen::<f32>();
            let mut x = game_state.screen_width;
            let enemy_type = self.rng.gen::<u32>() % 3 + 1;
            for _ in 0..self.enemy_amount_to_spawn {
                match enemy_type {
                    3 => {
                        let enemy = Enemy1::new(x, height, self.enemy_amount_to_spawn as f32);
                        result.push(EntityEvent::SpawnEntity(Box::new(enemy)));
                    }
                    2 => {
                        let enemy = Enemy2::new(x, height, self.enemy_amount_to_spawn as f32);
                        result.push(EntityEvent::SpawnEntity(Box::new(enemy)));
                    }
                    1 => {
                        let enemy = Enemy1::new(x, height, self.enemy_amount_to_spawn as f32);
                        result.push(EntityEvent::SpawnEntity(Box::new(enemy)));
                    }
                    _ => unreachable!(),
                };
                x += 100f32;
            }
            self.enemy_amount_to_spawn += 1;
            self.generate_time_target();
        }
        result
    }
}
