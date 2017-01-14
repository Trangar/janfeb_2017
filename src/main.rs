#[macro_use] extern crate glium;
extern crate image;
extern crate time;
extern crate rand;

mod engine;

#[derive(PartialEq, Eq, Hash)]
pub enum Graphic {
    Spaceship,
    Bullet
}

use glium::glutin::VirtualKeyCode;
use engine::{DrawHelper, Engine, Entity};
use rand::Rng;

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

fn get_time() -> u64 {
    time::precise_time_ns() / NS_TO_MS
}

fn time_elapsed_since(time: &mut u64, interval_in_ms: u64) -> bool {
    let compare_time = time::precise_time_ns() / NS_TO_MS;
    if *time + interval_in_ms < compare_time {
        *time += interval_in_ms;
        true
    } else {
        false
    }
}

fn get_time_since(time: &mut u64) -> u64 {
    let time_now = time::precise_time_ns() / NS_TO_MS;
    let diff = time_now - *time;
    *time = time_now;
    diff
}

fn main() {
    let mut engine: Engine = Engine::new(1200f32, 400f32).unwrap();
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
}

fn clamp_to(value: &mut f32, min: f32, max: f32) {
    if *value < min { *value = min; }
    if *value > max { *value = max; }
}