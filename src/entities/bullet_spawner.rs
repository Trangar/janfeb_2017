use engine::*;
use super::Bullet;

pub const BULLET_WIDTH: f32 = 16.0;
pub const BULLET_HEIGHT: f32 = 16.0;
const BULLET_SPAWN_INTERVAL: f32 = 100f32;

pub struct BulletSpawner {
    pub bullet_wrapper: Rc<DrawHelper>,
    pub time_counter: f32,
    pub next_bullet_id: u64,
}

impl BulletSpawner {
    pub fn new(graphics: &EngineGraphics) -> BulletSpawner {
        let bullet_wrapper = DrawHelper::new(graphics,
                                             BULLET_WIDTH,
                                             BULLET_HEIGHT,
                                             &include_bytes!("../../assets/bullet.png")[..])
            .unwrap();
        BulletSpawner {
            bullet_wrapper: Rc::new(bullet_wrapper),
            time_counter: BULLET_SPAWN_INTERVAL,
            next_bullet_id: 1,
        }
    }
}

impl EntityTrait for BulletSpawner {
    fn identifying_string(&self) -> String { "Bullet spawner".to_owned() }
    fn update(&mut self, game_state: &mut GameState, _: &mut EntityState) -> Vec<EntityEvent> {
        self.time_counter += game_state.delta_time;
        let mut result = Vec::new();
        while self.time_counter > BULLET_SPAWN_INTERVAL {
            self.time_counter -= BULLET_SPAWN_INTERVAL;
            let height = (game_state.screen_height) * game_state.rand_f32();
            let bullet = Bullet::new(self.bullet_wrapper.clone(), game_state.screen_width, height, self.next_bullet_id);
            result.push(EntityEvent::SpawnEntity(Box::new(bullet)));

            self.next_bullet_id += 1;
        }
        result
    }
}
