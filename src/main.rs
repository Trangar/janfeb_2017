#[macro_use] extern crate glium;
extern crate image;
extern crate time;
extern crate rand;

mod engine;

use glium::glutin::{ElementState, Event, VirtualKeyCode};
use engine::{DrawHelper, Engine};
use rand::Rng;

const HORIZONTAL_SPEED: f32 = 3f32;
const VERTICAL_SPEED: f32 = 4f32;
const ENEMY_BULLET_SPEED: f32 = 0.5f32;

const PLAYER_WIDTH: f32 = 128.0;
const PLAYER_HEIGHT: f32 = 64.0;
const BULLET_WIDTH: f32 = 16.0;
const BULLET_HEIGHT: f32 = 16.0;

const MAX_BULLETS: usize = 5000;
const BULLET_SPAWN_INTERVAL: u64 = 100;
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

fn main() {
    let mut engine: Engine = Engine::new(1200, 400).unwrap();
    let spaceship_wrapper = DrawHelper::new(&engine, PLAYER_WIDTH, PLAYER_HEIGHT, &include_bytes!("../assets/spaceship.png")[..]).unwrap();
    let bullet_wrapper = DrawHelper::new(&engine, BULLET_WIDTH, BULLET_HEIGHT, &include_bytes!("../assets/bullet.png")[..]).unwrap();

    let mut running = true;

    let mut down_down = false;
    let mut up_down = false;
    let mut right_down = false;
    let mut left_down = false;

    let mut x = 0f32;
    let mut y = 0f32;
    let mut last_spawn_time = get_time();
    let mut enemies: Vec<(f32, f32)> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut frame_count = 0;
    let mut last_frame_time = get_time();

    while running {
        let mut frame = engine.begin_draw();
        
        for enemy in &mut enemies {
            bullet_wrapper.draw_at(&mut engine, &mut frame, enemy.0, enemy.1, 0.0, 1.0).unwrap();
            enemy.0 -= ENEMY_BULLET_SPEED;
        }
        enemies.retain(|e| e.0 > -BULLET_WIDTH);

        spaceship_wrapper.draw_at(&mut engine, &mut frame, x, y, 0.0, 0.5).unwrap();
        frame.finish().unwrap();

        for event in engine.display.poll_events() {
            match event {
                Event::Closed |
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    running = false; break;
                },
                Event::KeyboardInput(state, _, Some(code)) => {
                    match code {
                        VirtualKeyCode::W => up_down = if let ElementState::Pressed = state { true } else { false },
                        VirtualKeyCode::D => right_down = if let ElementState::Pressed = state { true } else { false },
                        VirtualKeyCode::S => down_down = if let ElementState::Pressed = state { true } else { false },
                        VirtualKeyCode::A => left_down = if let ElementState::Pressed = state { true } else { false },
                        _ => {}
                    }
                },
                _ => (),
            }
        }

        if up_down { y -= VERTICAL_SPEED; }
        if down_down { y += VERTICAL_SPEED; }
        if left_down { x -= HORIZONTAL_SPEED; }
        if right_down { x += HORIZONTAL_SPEED; }

        clamp_to(&mut y, spaceship_wrapper.height / 2f32, engine.height as f32 - spaceship_wrapper.height / 2f32);
        clamp_to(&mut x, spaceship_wrapper.width / 2f32, engine.width as f32 - spaceship_wrapper.width / 2f32);
        
        while time_elapsed_since(&mut last_spawn_time, BULLET_SPAWN_INTERVAL) && enemies.len() < MAX_BULLETS {
            let height = (engine.height as f32 + BULLET_HEIGHT) * rng.next_f32() - BULLET_HEIGHT;
            //enemies.push((engine.width as f32, height));
        }

        frame_count += 1;
        if time_elapsed_since(&mut last_frame_time, 500) {
            engine.display.get_window().unwrap().set_title(&format!("FPS: {} - entities: {}", frame_count * 2, enemies.len() + 1));
            frame_count = 0;
        }
    }
}

fn clamp_to(value: &mut f32, min: f32, max: f32) {
    if *value < min { *value = min; }
    if *value > max { *value = max; }
}