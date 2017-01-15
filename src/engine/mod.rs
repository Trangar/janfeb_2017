mod error;
mod draw_helper;
mod entity;
mod keyboard_state;
mod time;

pub use self::draw_helper::DrawHelper;
pub use self::error::Result;
pub use self::entity::{EntityTrait, EntityWrapper, EntityState, CollisionResult, UpdateResult,
                       EntityUpdateState};
pub use self::keyboard_state::KeyboardState;

use glium::{DisplayBuild, DrawParameters, IndexBuffer, Frame, VertexBuffer, Program, Surface};
use glium::glutin::{Event, VirtualKeyCode, ElementState, WindowBuilder};
use glium::backend::glutin_backend::GlutinFacade;
use glium::uniforms::UniformsStorage;
use glium::index::PrimitiveType;

pub struct Engine {
    pub graphics: EngineGraphics,
    pub keyboard: KeyboardState,
    pub running: bool,

    pub last_update_time: u64,
    pub entities: Vec<EntityWrapper>,
    pub rng: ::rand::ThreadRng,
}

pub struct EngineGraphics {
    pub display: GlutinFacade,
    pub textured_program: Program,
    pub color_program: Program,
    pub frame: Option<Frame>,
    pub width: f32,
    pub height: f32,
}

impl Engine {
    pub fn new(width: f32, height: f32) -> Result<Engine> {
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

        Ok(Engine {
            graphics: EngineGraphics {
                display: display,
                textured_program: textured_program,
                color_program: color_program,
                frame: None,
                width: width,
                height: height,
            },
            keyboard: KeyboardState::default(),
            running: true,
            last_update_time: self::time::get(),
            entities: Vec::new(),
            rng: ::rand::thread_rng(),
        })
    }

    pub fn register_entity<T: EntityTrait + 'static>(&mut self, entity: T) {
        self.entities.push(EntityWrapper::new(Box::new(entity)));
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

    pub fn update_entities(&mut self) {
        let delta_time = self::time::since(&mut self.last_update_time) as f32;
        let mut spawned_entities: Vec<Box<EntityTrait>> = Vec::new();

        for entity in &mut self.entities {
            let mut state = EntityUpdateState {
                state: &mut entity.state,
                delta_time: delta_time,
                keyboard_state: &self.keyboard,
                screen_width: self.graphics.width,
                screen_height: self.graphics.height,
                rng: &mut self.rng,
            };
            let update_result = entity.entity.update(&mut state);
            for result in update_result.into_iter() {
                match result {
                    UpdateResult::SpawnEntity(e) => spawned_entities.push(e),
                }
            }
        }
        self.entities.retain(|e| e.state.active);

        for entity in spawned_entities.into_iter() {
            self.entities.push(EntityWrapper::new(entity));
        }
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
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[allow(dead_code)]
pub fn draw_rectangle(engine: &Engine,
                      frame: &mut Frame,
                      x: f32,
                      y: f32,
                      width: f32,
                      height: f32,
                      color: (f32, f32, f32, f32))
                      -> Result<()> {
    let vertex_buffer = VertexBuffer::new(&engine.graphics.display,
                                          &[Vertex { position: [x, y] },
                                            Vertex { position: [x + width, y] },
                                            Vertex { position: [x, y + height] },
                                            Vertex { position: [x + width, y + height] }])?;

    let index_buffer = IndexBuffer::<u8>::new(&engine.graphics.display,
                                              PrimitiveType::TriangleStrip,
                                              &[0, 1, 2, 3])?;

    let matrix = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0f32]];
    let uniform = UniformsStorage::new("matrix", matrix);
    let uniform = uniform.add("color", color);
    let uniform = uniform.add("screen_size",
                              [engine.graphics.width as f32, engine.graphics.height as f32]);
    frame.draw(&vertex_buffer,
              &index_buffer,
              &engine.graphics.color_program,
              &uniform,
              &DrawParameters::default())?;
    Ok(())
}
