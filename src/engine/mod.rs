mod error;
pub use self::error::Result;

mod draw_helper;
pub use self::draw_helper::DrawHelper;

use glium::{Blend, DisplayBuild, DrawParameters, IndexBuffer, Frame, Program, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::WindowBuilder;
use glium::index::PrimitiveType;

pub struct Engine<'a> {
    pub display: GlutinFacade,
    pub program: Program,
    pub draw_parameters: DrawParameters<'a>,
    pub index_buffer: IndexBuffer<u8>,
    pub width: u32,
    pub height: u32,
    pub last_drawn_texture: Option<u64>,
}

impl<'a> Engine<'a> {
    pub fn new(width: u32, height: u32) -> Result<Engine<'a>> {
        let display = WindowBuilder::new()
            .with_dimensions(width, height)
            .with_min_dimensions(width, height)
            .with_max_dimensions(width, height)
            .with_vsync()
            .build_glium()?;
        let program = Program::from_source(&display, include_str!("../../assets/shader.vert"), include_str!("../../assets/shader.frag"), None)?;
        let index_buffer = IndexBuffer::<u8>::new(&display, PrimitiveType::TriangleStrip, &[0, 1, 2, 3])?;

        println!("{:?}", display.get_opengl_version());

        Ok(Engine {
            display: display,
            program: program,
            width: width,
            height: height,
            draw_parameters: DrawParameters {
                blend: Blend::alpha_blending(),
                .. DrawParameters::default()
            },
            index_buffer: index_buffer,
            last_drawn_texture: None
        })
    }

    pub fn begin_draw(&mut self) -> Frame {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        frame
    }
}

