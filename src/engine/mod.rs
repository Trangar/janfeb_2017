mod engine_graphics;
mod keyboard_state;
mod text_graphics;
mod draw_helper;
mod game_state;
mod entity;
mod error;
mod time;

pub use self::engine_graphics::EngineGraphics;
pub use self::keyboard_state::KeyboardState;
pub use self::text_graphics::TextGraphics;
pub use self::draw_helper::DrawHelper;
pub use self::game_state::GameState;
pub use self::error::Result;
pub use self::entity::*;

use glium::glutin::{Event, ElementState};
use std::collections::HashMap;
use std::hash::Hash;
use glium::Surface;

pub use glium::glutin::VirtualKeyCode;
pub use std::rc::Rc;

pub trait TGraphicIndex: PartialEq + Eq + Hash {}

pub type Color = (f32, f32, f32, f32);

pub struct Engine<'a, T: 'a + TGraphicIndex> {
    pub graphics: EngineGraphics<T>,
    pub keyboard: KeyboardState,
    pub running: bool,

    pub last_update_time: u64,
    pub entities: Vec<EntityWrapper<T>>,
    pub layered_entities: HashMap<CollisionLayer, Vec<&'a EntityWrapper<T>>>,
    pub rng: ::rand::ThreadRng,
}

impl<'a, T: TGraphicIndex> Engine<'a, T> {
    pub fn new(width: f32, height: f32) -> Result<Engine<'a, T>> {
        let engine = Engine {
            graphics: EngineGraphics::<T>::new(width, height)?,
            keyboard: KeyboardState::default(),
            running: true,
            last_update_time: self::time::get(),
            entities: Vec::new(),
            layered_entities: HashMap::new(),
            rng: ::rand::thread_rng(),
        };
        Ok(engine)
    }

    pub fn register_entity<TEntity: EntityTrait<T> + 'static>(&mut self, entity: TEntity) {
        let wrapper = EntityWrapper::new(Box::new(entity), self);
        self.entities.push(wrapper);
    }

    pub fn draw(&mut self) -> Result<()> {
        self.keyboard.frame_start();
        self.graphics.frame = Some(self.graphics.display.draw());

        if let Some(ref mut frame) = self.graphics.frame {
            frame.clear_color(0.0, 0.0, 1.0, 1.0);
        }

        for entity in &self.entities {
            entity.entity.draw(&entity.state, &mut self.graphics)?;
        }

        if let Some(frame) = self.graphics.frame.take() {
            frame.finish()?;
        }
        self.graphics.text_graphics.frame_end();
        Ok(())
    }

    fn handle_event(&mut self, events: Vec<EntityEvent<T>>) {
        for result in events.into_iter() {
            match result {
                EntityEvent::SpawnEntity(entity) => {
                    let wrapper = EntityWrapper::new(entity, self);
                    self.entities.push(wrapper);
                }
                EntityEvent::ClearAllEntities => {
                    self.entities.clear();
                }
            }
        }
    }

    fn check_collisions(&mut self) {
        let events = {
            let mut events = Vec::new();
            let ref mut slice = self.entities[..];
            for i in 1..slice.len() {
                let (ref mut first, ref mut remaining) = slice.split_at_mut(i);
                let ref mut first = first.last_mut().unwrap();
                for ref mut second in remaining.iter_mut() {
                    if first.entity.intersects_with(&first.state, &second.entity, &second.state) {
                        let results = first.entity
                            .collided(&mut first.state, &second.entity, &mut second.state);
                        events.extend(results.into_iter());
                    }
                    if second.entity.intersects_with(&second.state, &first.entity, &first.state) {
                        let results = second.entity
                            .collided(&mut second.state, &first.entity, &mut first.state);
                        events.extend(results.into_iter());
                    }
                }
            }
            events
        };

        self.handle_event(events);
    }

    pub fn update_entities(&mut self) {
        let delta_time = self::time::since(&mut self.last_update_time) as f32;

        let mut events = Vec::new();

        for entity in &mut self.entities {
            let mut state = GameState {
                delta_time: delta_time,
                keyboard: &self.keyboard,
                screen_width: self.graphics.width,
                screen_height: self.graphics.height,
                rng: &mut self.rng,
            };
            let update_result = entity.entity.update(&mut state, &mut entity.state);
            events.extend(update_result.into_iter());
        }

        self.handle_event(events);

        self.check_collisions();

        self.entities.retain(|e| e.state.active);
    }

    pub fn run(&mut self) {
        let mut frame_count = 0;
        let mut last_frame_time = self::time::get();
        while self.running {
            self.draw().unwrap();
            self.update_events();
            self.update_entities();

            frame_count += 1;
            if self::time::has_elapsed(&mut last_frame_time, 500) {
                // println!("FPS: {} - entities: {}", frame_count * 2, self.entities.len());
                self.graphics
                    .display
                    .get_window()
                    .unwrap()
                    .set_title(&format!("FPS: {} - entities: {}",
                                        frame_count * 2,
                                        self.entities.len()));
                frame_count = 0;
            }
        }
    }

    pub fn update_events(&mut self) {
        for event in self.graphics.display.poll_events() {
            match event {
                Event::Closed |
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    self.running = false;
                    break;
                }
                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                    self.keyboard.set_keydown(code);
                }
                Event::KeyboardInput(ElementState::Released, _, Some(code)) => {
                    self.keyboard.clear_keydown(code);
                }
                _ => (),
            }
        }
    }
}
