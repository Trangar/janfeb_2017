#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate image;
extern crate time as __time;
extern crate rand;

mod engine_graphics;
mod keyboard_state;
mod text_graphics;
mod draw_helper;
mod game_state;
mod engine;
mod entity;
mod error;
mod time;

pub use self::engine_graphics::EngineGraphics;
pub use self::keyboard_state::KeyboardState;
pub use self::text_graphics::TextGraphics;
pub use self::draw_helper::DrawHelper;
pub use self::game_state::GameState;
pub use self::engine::Engine;
pub use self::error::Result;
pub use self::entity::*;

use glium::glutin::{Event, ElementState};
use std::hash::Hash;

pub use glium::glutin::VirtualKeyCode;
pub type Color = (f32, f32, f32, f32);
pub trait TGraphicIndex: PartialEq + Eq + Hash {}
