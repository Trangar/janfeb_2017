#![allow(clippy::borrowed_box)]

#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate image;
extern crate rand;
extern crate time as __time;
extern crate winit;

mod draw_helper;
mod engine;
mod engine_graphics;
mod entity;
mod error;
mod game_state;
mod keyboard_state;
mod text_graphics;
mod time;

pub use self::draw_helper::DrawHelper;
pub use self::engine::Engine;
pub use self::engine_graphics::EngineGraphics;
pub use self::entity::*;
pub use self::error::Result;
pub use self::game_state::GameState;
pub use self::keyboard_state::KeyboardState;
pub use self::text_graphics::TextGraphics;

use glium::glutin::{ElementState, Event};
use std::hash::Hash;

pub use glium::glutin::VirtualKeyCode;
pub type Color = (f32, f32, f32, f32);
pub trait TGraphicIndex: PartialEq + Eq + Hash {}
