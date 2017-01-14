#[macro_use] extern crate glium;
extern crate image;
extern crate time;
extern crate rand;

mod engine;

use std::rc::Rc;
use engine::{DrawHelper, Engine, EntityTrait, EngineGraphics, EntityState, Result, UpdateResult, EntityUpdateState};
use rand::Rng;
use std::fmt::{Debug, Formatter, Result as fmtResult};

const BULLET_WIDTH: f32 = 16.0;
const BULLET_HEIGHT: f32 = 16.0;

struct BulletSpawner {
    pub bullet_wrapper: Rc<DrawHelper>,
    pub time_counter: f32
}

impl BulletSpawner {
    pub fn new(graphics: &EngineGraphics) -> BulletSpawner {
        let bullet_wrapper = DrawHelper::new(graphics, BULLET_WIDTH, BULLET_HEIGHT, &include_bytes!("../assets/bullet.png")[..]).unwrap();
        BulletSpawner {
            bullet_wrapper: Rc::new(bullet_wrapper),
            time_counter: 0f32
        }
    }
}

const BULLET_SPAWN_INTERVAL: f32 = 100f32;
impl Debug for BulletSpawner {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "Bullet spawner [{}/{}]", self.time_counter, BULLET_SPAWN_INTERVAL)
    }
}

impl EntityTrait for BulletSpawner {
    fn update(&mut self, state: &mut EntityUpdateState) -> Vec<UpdateResult>{
        self.time_counter += state.delta_time;
        let mut result = Vec::new();
        while self.time_counter > BULLET_SPAWN_INTERVAL {
            self.time_counter -= BULLET_SPAWN_INTERVAL;
            let height = (state.screen_height) * state.rng.next_f32();
            let bullet = Bullet::new(self.bullet_wrapper.clone(), state.screen_width, height);
            result.push(UpdateResult::SpawnEntity(Box::new(bullet)));
        }
        result
    }
}

struct Bullet {
    drawable: Rc<DrawHelper>,
    start_x: f32,
    start_y: f32,
}

impl Bullet {
    pub fn new(drawable: Rc<DrawHelper>, x: f32, y: f32) -> Bullet {
        Bullet {
            drawable: drawable,
            start_x: x,
            start_y: y,
        }
    }
}

impl Debug for Bullet {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "Bullet [{}/{}]", self.start_x, self.start_y)
    }
}

impl EntityTrait for Bullet {
    fn get_initial_state(&self) -> EntityState {
        EntityState {
            x: self.start_x,
            y: self.start_y,
            .. EntityState::default()
        }
    }

    fn update(&mut self, state: &mut EntityUpdateState) -> Vec<UpdateResult> {
        state.state.x -= 0.1f32 * state.delta_time;

        if state.state.x + state.state.hitbox.right < 0f32 {
            state.state.active = false;
        }
        Vec::new()
    }

    fn draw(&self, state: &EntityState, graphics: &mut EngineGraphics) -> Result<()> {
        self.drawable.draw_at(graphics, state.x, state.y, 0f32, 1f32)
    }
}


fn main() {
    let mut engine: Engine = Engine::new(1200f32, 400f32).unwrap();
    //engine.register_entity(Player::new());
    let spawner = BulletSpawner::new(&engine.graphics);
    engine.register_entity(spawner);

    engine.run();

    /*
const HORIZONTAL_SPEED: f32 = 0.15f32;
const VERTICAL_SPEED: f32 = 0.2f32;
const ENEMY_BULLET_SPEED: f32 = 0.3f32;
const BULLET_SPAWN_INTERVAL: u64 = 100;

const PLAYER_WIDTH: f32 = 128.0;
const PLAYER_HEIGHT: f32 = 64.0;
const BULLET_WIDTH: f32 = 16.0;
const BULLET_HEIGHT: f32 = 16.0;

const PLAYER_FIRE_INTERVAL: u64 = 100;
const PLAYER_FIRE_POINTS: [[f32;2];2] = [[-10.0, -25.0],[-10.0,25.0]];
const PLAYER_BULLET_SPEED: f32 = 0.5f32;

const MAX_BULLETS: usize = 5000;
const NS_TO_MS: u64 = 1_000_000;


    let spaceship_wrapper = DrawHelper::new(&engine, PLAYER_WIDTH, PLAYER_HEIGHT, &include_bytes!("../assets/spaceship.png")[..]).unwrap();
    let bullet_wrapper = DrawHelper::new(&engine, BULLET_WIDTH, BULLET_HEIGHT, &include_bytes!("../assets/bullet.png")[..]).unwrap();
    
    let mut player = Entity::new_player(&spaceship_wrapper, (engine.height / 2f32) );
    let mut player_bullets: Vec<Entity> = Vec::new();
    let mut enemy_bullets: Vec<Entity> = Vec::new();

    let mut last_player_fire_time = get_time();
    let mut last_player_fire_position = 0;

    let mut last_enemy_bullet_spawn_time = get_time();
    
    let mut frame_count = 0;
    let mut last_frame_time = get_time();

    let mut rng = rand::thread_rng();
    
    while engine.running {
        let mut frame = engine.begin_draw();
        
        for bullet in &player_bullets {
            bullet.draw(&engine, &mut frame).unwrap();
        }
        for enemy_bullet in &enemy_bullets {
            enemy_bullet.draw(&engine, &mut frame).unwrap();
        }
        player.draw(&engine, &mut frame).unwrap();
        
        frame.finish().unwrap();

        engine.update();
        let delta_time = get_time_since(&mut last_frame_time) as f32;
        
        let mut diff = (0f32, 0f32);
        if engine.keyboard.is_keydown(VirtualKeyCode::Left) || engine.keyboard.is_keydown(VirtualKeyCode::A) { diff.0 -= HORIZONTAL_SPEED; }
        if engine.keyboard.is_keydown(VirtualKeyCode::Right) || engine.keyboard.is_keydown(VirtualKeyCode::D) { diff.0 += HORIZONTAL_SPEED; }
        if engine.keyboard.is_keydown(VirtualKeyCode::Up) || engine.keyboard.is_keydown(VirtualKeyCode::W) { diff.1 -= VERTICAL_SPEED; }
        if engine.keyboard.is_keydown(VirtualKeyCode::Down) || engine.keyboard.is_keydown(VirtualKeyCode::S) { diff.1 += VERTICAL_SPEED; }

        player.x += diff.0 * delta_time;
        player.y += diff.1 * delta_time;

        clamp_to(&mut player.y, player.hitbox.top, engine.height  - player.hitbox.bottom);
        clamp_to(&mut player.x, player.hitbox.left, engine.width  - player.hitbox.right);
        
        while time_elapsed_since(&mut last_player_fire_time, PLAYER_FIRE_INTERVAL) {
            let position = PLAYER_FIRE_POINTS[last_player_fire_position];
            last_player_fire_position = (last_player_fire_position + 1) % PLAYER_FIRE_POINTS.len();

            player_bullets.push(Entity::new_bullet(&bullet_wrapper, player.x + position[0], player.y + position[1]));
        }
        
        while time_elapsed_since(&mut last_enemy_bullet_spawn_time, BULLET_SPAWN_INTERVAL) && enemy_bullets.len() < MAX_BULLETS {
            let height = (engine.height  + BULLET_HEIGHT) * rng.next_f32() - BULLET_HEIGHT;
            enemy_bullets.push(Entity::new_bullet(&bullet_wrapper, engine.width , height));
        }

        for bullet in &mut player_bullets {
            bullet.x += PLAYER_BULLET_SPEED * delta_time;

            let mut has_intersections = false;
            for ref mut enemy_bullet in enemy_bullets.iter_mut().filter(|b| b.intersects_with(bullet)) {
                has_intersections = true;
                enemy_bullet.x = -enemy_bullet.hitbox.right;
            }
            if has_intersections {
                bullet.x = engine.width  + bullet.hitbox.left;
            }
        }
        
        for enemy_bullet in &mut enemy_bullets {
            enemy_bullet.x -= ENEMY_BULLET_SPEED * delta_time;
        }

        player_bullets.retain(|b| b.x - b.hitbox.left <= engine.width);
        enemy_bullets.retain(|b| b.x + b.hitbox.right >= 0f32);

        frame_count += 1;
        if time_elapsed_since(&mut last_frame_time, 500) {
            engine.display.get_window().unwrap().set_title(&format!("FPS: {} - entities: {}", frame_count * 2, enemy_bullets.len() + player_bullets.len() + 1));
            frame_count = 0;
        }
    }
    */
}

// fn clamp_to(value: &mut f32, min: f32, max: f32) {
//     if *value < min { *value = min; }
//     if *value > max { *value = max; }
// }