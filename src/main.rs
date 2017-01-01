#[macro_use] extern crate glium;
extern crate image;

mod engine;

use glium::glutin::{ElementState, Event, VirtualKeyCode};
use engine::{Engine, Rect};

const HORIZONTAL_SPEED: f32 = 3f32;
const VERTICAL_SPEED: f32 = 4f32;

fn main() {
    let mut engine: Engine = Engine::new(1200, 400).unwrap();

    let spaceship_texture = engine.load_texture(include_bytes!("../assets/spaceship.png")).unwrap();
    let mut running = true;

    let mut down_down = false;
    let mut up_down = false;
    let mut right_down = false;
    let mut left_down = false;

    let mut x = 0f32;
    let mut y = 0f32;

    while running {
        let mut frame = engine.begin_draw();
        
        engine.draw_texture(&mut frame, Rect::new(x, y, 128.0, 64.0), &spaceship_texture).unwrap();
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
        if y > engine.height as f32 - 64.0 { y = engine.height as f32 - 64.0; }

        if left_down { x -= HORIZONTAL_SPEED; }
        if right_down { x += HORIZONTAL_SPEED; }

        if x < 0.0 { x = 0.0; }
        if x > engine.width as f32 - 128.0 { x = engine.width as f32 - 128.0; }
    }
}
