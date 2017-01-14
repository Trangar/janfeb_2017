mod error;
mod draw_helper;
mod entity;
mod keyboard_state;

pub use self::draw_helper::DrawHelper;
pub use self::error::Result;
pub use self::entity::Entity;
use self::keyboard_state::KeyboardState;

use glium::{Blend, DisplayBuild, DrawParameters, IndexBuffer, Frame, VertexBuffer, Program, Surface};
use glium::glutin::{Event, VirtualKeyCode, ElementState, WindowBuilder};
use glium::backend::glutin_backend::GlutinFacade;
use glium::uniforms::UniformsStorage;
use glium::index::PrimitiveType;

pub struct Engine<'a> {
    pub display: GlutinFacade,
    pub textured_program: Program,
    pub color_program: Program,
    pub draw_parameters: DrawParameters<'a>,
    pub index_buffer: IndexBuffer<u8>,
    pub width: f32,
    pub height: f32,
    pub last_drawn_texture: Option<u64>,
    pub keyboard: KeyboardState,
    pub running: bool,
}

impl<'a> Engine<'a> {
    pub fn new(width: f32, height: f32) -> Result<Engine<'a>> {
        let display = WindowBuilder::new()
            .with_dimensions(width as u32, height as u32)
            .with_min_dimensions(width as u32, height as u32)
            .with_max_dimensions(width as u32, height as u32)
            .with_vsync()
            .build_glium()?;
        let textured_program = Program::from_source(&display, include_str!("../../assets/textured_shader.vert"), include_str!("../../assets/textured_shader.frag"), None)?;
        let color_program = Program::from_source(&display, include_str!("../../assets/color_shader.vert"), include_str!("../../assets/color_shader.frag"), None)?;
        let index_buffer = IndexBuffer::<u8>::new(&display, PrimitiveType::TriangleStrip, &[0, 1, 2, 3])?;

        println!("{:?}", display.get_opengl_version());

        Ok(Engine {
            display: display,
            textured_program: textured_program,
            color_program: color_program,
            width: width,
            height: height,
            draw_parameters: DrawParameters {
                blend: Blend::alpha_blending(),
                .. DrawParameters::default()
            },
            index_buffer: index_buffer,
            last_drawn_texture: None,
            keyboard: KeyboardState::default(),
            running: true,
        })
    }

    pub fn begin_draw(&mut self) -> Frame {
        self.keyboard.frame_start();
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame
    }

    pub fn update(&mut self) {
        for event in self.display.poll_events() {
            match event {
                Event::Closed |
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    self.running = false; break;
                },
                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                    self.keyboard.set_keydown(code);
                },
                Event::KeyboardInput(ElementState::Released, _, Some(code)) => {
                    self.keyboard.clear_keydown(code);
                },
                _ => (),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32;2],
}

implement_vertex!(Vertex, position);

#[allow(dead_code)]
pub fn draw_rectangle(engine: &Engine, frame: &mut Frame, x: f32, y: f32, width: f32, height: f32, color: (f32, f32, f32, f32)) -> Result<()> {
    let vertex_buffer = VertexBuffer::new(&engine.display, &[
        Vertex {
            position: [x, y],
        },
        Vertex {
            position: [x + width, y],
        },
        Vertex {
            position: [x, y + height],
        },
        Vertex {
            position: [x + width, y + height]
        }
    ])?;

    let index_buffer = IndexBuffer::<u8>::new(&engine.display, PrimitiveType::TriangleStrip, &[0, 1, 2, 3])?;

    let matrix = [
        [
            1.0,
            0.0,
            0.0
        ],
        [
            0.0,
            1.0,
            0.0
        ],
        [0.0, 0.0, 1.0f32]
    ];
    let uniform = UniformsStorage::new("matrix", matrix);
    let uniform = uniform.add("color", color);
    let uniform = uniform.add("screen_size", [engine.width as f32, engine.height as f32]);
    frame.draw(
        &vertex_buffer,
        &index_buffer,
        &engine.color_program,
        &uniform,
        &DrawParameters::default()
    )?;
    Ok(())
}