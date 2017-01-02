#[macro_use] extern crate glium;
extern crate image;
extern crate time;
extern crate rand;

mod engine;

use glium::glutin::{ElementState, Event, VirtualKeyCode};
use engine::{Engine, Rect};
use rand::Rng;

const HORIZONTAL_SPEED: f32 = 3f32;
const VERTICAL_SPEED: f32 = 4f32;
const ENEMY_BULLET_SPEED: f32 = 2f32;

const PLAYER_WIDTH: f32 = 128.0;
const PLAYER_HEIGHT: f32 = 64.0;
const BULLET_WIDTH: f32 = 16.0;
const BULLET_HEIGHT: f32 = 16.0;

const MAX_BULLETS: usize = 500;
const BULLET_SPAWN_INTERVAL: u64 = 1;

fn time_elapsed_since(time: &mut u64, interval_in_ms: u64) -> bool {
    const NS_TO_MS: u64 = 1_000_000;
    let compare_time = time::precise_time_ns() / NS_TO_MS;
    if *time + interval_in_ms < compare_time {
        *time = compare_time;
        true
    } else {
        false
    }
}

fn main() {
    let mut engine: Engine = Engine::new(1200, 400).unwrap();

    let spaceship_texture = engine.load_texture(include_bytes!("../assets/spaceship.png")).unwrap();
    let enemy_texture = engine.load_texture(include_bytes!("../assets/bullet.png")).unwrap();
    let mut running = true;

    let mut down_down = false;
    let mut up_down = false;
    let mut right_down = false;
    let mut left_down = false;

    let mut x = 0f32;
    let mut y = 0f32;
    let mut last_spawn_time = 0;
    let mut enemies: Vec<(f32, f32)> = Vec::new();
    let mut rng = rand::thread_rng();

    while running {
        engine.display.get_window().unwrap().set_title(&format!("Enemies: {}", enemies.len()));
        let mut frame = engine.begin_draw();
        
        for enemy in &mut enemies {
            engine.draw_texture(&mut frame, Rect::new(enemy.0, enemy.1, BULLET_WIDTH, BULLET_HEIGHT), &enemy_texture).unwrap();
            enemy.0 -= ENEMY_BULLET_SPEED;
        }
        enemies.retain(|e| e.0 > -BULLET_WIDTH);

        engine.draw_texture(&mut frame, Rect::new(x, y, PLAYER_WIDTH, PLAYER_HEIGHT), &spaceship_texture).unwrap();
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

        if y < 0.0 { y = 0.0; }
        if y > engine.height as f32 - PLAYER_HEIGHT { y = engine.height as f32 - PLAYER_HEIGHT; }

        if left_down { x -= HORIZONTAL_SPEED; }
        if right_down { x += HORIZONTAL_SPEED; }

        if x < 0.0 { x = 0.0; }
        if x > engine.width as f32 - PLAYER_WIDTH { x = engine.width as f32 - PLAYER_WIDTH; }

        if time_elapsed_since(&mut last_spawn_time, BULLET_SPAWN_INTERVAL) && enemies.len() < MAX_BULLETS {
            let height = engine.height as f32 * rng.next_f32();
            enemies.push((engine.width as f32, height));
        }
    }
}
