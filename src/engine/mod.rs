mod error;
mod draw_helper;
mod entity;
mod keyboard_state;
mod time;

pub use self::keyboard_state::KeyboardState;
pub use self::draw_helper::DrawHelper;
pub use self::error::Result;
pub use self::entity::*;

use glium::{DisplayBuild, DrawParameters, IndexBuffer, Frame, VertexBuffer, Program, Surface};
use glium::glutin::{Event, ElementState, WindowBuilder};
use glium::backend::glutin_backend::GlutinFacade;
use glium::uniforms::UniformsStorage;
use glium::index::PrimitiveType;

pub use glium::glutin::VirtualKeyCode;
pub use std::rc::Rc;

pub struct Engine {
    pub graphics: EngineGraphics,
    pub keyboard: KeyboardState,
    pub running: bool,

    pub last_update_time: u64,
    pub entities: Vec<EntityWrapper>,
    pub rng: ::rand::ThreadRng,
}

impl Engine {
    pub fn new(width: f32, height: f32) -> Result<Engine> {
        let engine = Engine {
            graphics: EngineGraphics::new(width, height)?,
            keyboard: KeyboardState::default(),
            running: true,
            last_update_time: self::time::get(),
            entities: Vec::new(),
            rng: ::rand::thread_rng(),
        };
        Ok(engine)
    }

    pub fn register_entity<T: EntityTrait + 'static>(&mut self, entity: T) {
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
        Ok(())
    }

    fn handle_event(&mut self, events: Vec<EntityEvent>) {
        for result in events.into_iter() {
            match result {
                EntityEvent::SpawnEntity(entity) => {
                    let wrapper = EntityWrapper::new(entity, self);
                    self.entities.push(wrapper);
                },
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
                        let results = first.entity.collided(&mut first.state, &second.entity, &mut second.state, &self.graphics);
                        events.extend(results.into_iter());
                    }
                    if second.entity.intersects_with(&second.state, &first.entity, &first.state) {
                        let results = second.entity.collided(&mut second.state, &first.entity, &mut first.state, &self.graphics);
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

#[derive(Copy, Clone)]
pub struct Vertex {
    pub dimension_affinity: [f32; 2],
}

implement_vertex!(Vertex, dimension_affinity);

pub struct EngineGraphics {
    pub display: GlutinFacade,
    pub textured_program: Program,
    pub color_program: Program,
    pub frame: Option<Frame>,
    pub width: f32,
    pub height: f32,

    rectangle_vertex_buffer: VertexBuffer<Vertex>,
    rectangle_index_buffer: IndexBuffer<u8>,
}

impl EngineGraphics {
    pub fn new(width: f32, height: f32) -> Result<EngineGraphics> {
        let display = WindowBuilder::new().with_dimensions(width as u32, height as u32)
            .with_min_dimensions(width as u32, height as u32)
            .with_max_dimensions(width as u32, height as u32)
            .with_vsync()
            .build_glium()?;
        let textured_program =
            Program::from_source(&display,
                                 include_str!("../../assets/textured_shader.vert"),
                                 include_str!("../../assets/textured_shader.frag"),
                                 None)?;
        let color_program = Program::from_source(&display,
                                                 include_str!("../../assets/color_shader.vert"),
                                                 include_str!("../../assets/color_shader.frag"),
                                                 None)?;

        println!("{:?}", display.get_opengl_version());

        let rectangle_vertex_buffer = VertexBuffer::new(&display,
                                            &[Vertex { dimension_affinity: [0f32, 0f32] },
                                              Vertex { dimension_affinity: [1f32, 0f32] },
                                              Vertex { dimension_affinity: [0f32, 1f32] },
                                              Vertex { dimension_affinity: [1f32, 1f32] }])?;

        let rectangle_index_buffer = IndexBuffer::<u8>::new(&display,
                                                PrimitiveType::TriangleStrip,
                                                &[0, 1, 2, 3])?;

        Ok(EngineGraphics {
            display: display,
            textured_program: textured_program,
            color_program: color_program,
            frame: None,
            width: width,
            height: height,
            rectangle_vertex_buffer: rectangle_vertex_buffer,
            rectangle_index_buffer: rectangle_index_buffer
        })
    }
    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: (f32, f32, f32, f32)) -> Result<()> {
        if let Some(ref mut frame) = self.frame {
            let uniform = UniformsStorage::new("offset", [x, y]);
            let uniform = uniform.add("dimensions", [width, height]);
            let uniform = uniform.add("color", color);
            let uniform = uniform.add("screen_size", [self.width as f32, self.height as f32]);
            frame.draw(&self.rectangle_vertex_buffer,
                    &self.rectangle_index_buffer,
                    &self.color_program,
                    &uniform,
                    &DrawParameters::default())?;
        }
        Ok(())
    }
}
