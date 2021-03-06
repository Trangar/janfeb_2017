#![allow(deprecated)]

use super::{Color, DrawHelper, Result, TGraphicIndex, TextGraphics};
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, WindowBuilder};
use glium::index::PrimitiveType;
use glium::uniforms::UniformsStorage;
use glium::{Blend, Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use winit::EventsLoop;

#[derive(Copy, Clone)]
struct Vertex {
    pub dimension_affinity: [f32; 2],
}

implement_vertex!(Vertex, dimension_affinity);

pub struct EngineGraphics<T: TGraphicIndex> {
    pub display: Display,
    pub events_loop: EventsLoop,
    pub textured_program: Program,
    pub color_program: Program,
    pub frame: Option<Frame>,
    pub width: f32,
    pub height: f32,
    pub text_graphics: TextGraphics,

    graphics: HashMap<T, DrawHelper>,

    rectangle_vertex_buffer: VertexBuffer<Vertex>,
    rectangle_index_buffer: IndexBuffer<u8>,
}

impl<T: TGraphicIndex> EngineGraphics<T> {
    pub fn new(width: f32, height: f32) -> Result<EngineGraphics<T>> {
        let wb = WindowBuilder::new()
            .with_dimensions(LogicalSize::new(width as _, height as _))
            .with_min_dimensions(LogicalSize::new(width as _, height as _))
            .with_max_dimensions(LogicalSize::new(width as _, height as _));
        let cb = ContextBuilder::new();
        let events_loop = EventsLoop::new();
        let display = Display::new(wb, cb, &events_loop).unwrap();

        let textured_program = Program::from_source(
            &display,
            include_str!("../assets/textured_shader.vert"),
            include_str!("../assets/textured_shader.frag"),
            None,
        )?;
        let color_program = Program::from_source(
            &display,
            include_str!("../assets/color_shader.vert"),
            include_str!("../assets/color_shader.frag"),
            None,
        )?;

        println!("{:?}", display.get_opengl_version());

        let rectangle_vertex_buffer = VertexBuffer::new(
            &display,
            &[
                Vertex {
                    dimension_affinity: [0f32, 0f32],
                },
                Vertex {
                    dimension_affinity: [1f32, 0f32],
                },
                Vertex {
                    dimension_affinity: [0f32, 1f32],
                },
                Vertex {
                    dimension_affinity: [1f32, 1f32],
                },
            ],
        )?;

        let rectangle_index_buffer =
            IndexBuffer::<u8>::new(&display, PrimitiveType::TriangleStrip, &[0, 1, 2, 3])?;
        let text = TextGraphics::new(&display)?;

        Ok(EngineGraphics {
            display,
            events_loop,
            textured_program,
            color_program,
            frame: None,
            width,
            height,
            text_graphics: text,

            graphics: HashMap::new(),

            rectangle_vertex_buffer,
            rectangle_index_buffer,
        })
    }

    pub fn load_graphic(&mut self, key: T, file: &str, width: f32, height: f32) -> Result<()> {
        let bytes = {
            let mut file = File::open(file)?;
            let mut vec = Vec::new();
            file.read_to_end(&mut vec)?;
            vec
        };
        let param = DrawHelper::new(self, width, height, &bytes)?;
        self.graphics.insert(key, param);
        Ok(())
    }

    // pub fn get_graphic(&self, key: T) -> Option<&DrawHelper> {
    // self.graphics.get(&key)
    // }
    //
    // pub fn draw_directly(&mut self, wrapper: &EntityWrapper<T>) -> Result<()> {
    // if let Some(helper) = wrapper.drawable {
    //
    // }
    // }
    pub fn draw_text_at(&mut self, string: String, x: f32, y: f32, color: Color) -> Result<()> {
        if let Some(frame) = &mut self.frame {
            self.text_graphics
                .draw_at(frame, string, (self.width, self.height), (x, y), color)?;
        }
        Ok(())
    }
    pub fn draw(&mut self, key: T, x: f32, y: f32, rotation: f32, scale: f32) -> Result<()> {
        if let Some(ref helper) = self.graphics.get(&key) {
            if let Some(frame) = &mut self.frame {
                EngineGraphics::<T>::draw_at(
                    &self.textured_program,
                    frame,
                    helper,
                    (self.width, self.height),
                    (x, y),
                    rotation,
                    scale,
                )?;
            }
        }
        Ok(())
    }
    fn draw_at(
        textured_program: &Program,
        frame: &mut Frame,
        helper: &DrawHelper,
        (width, height): (f32, f32),
        (x, y): (f32, f32),
        rotation: f32,
        scale: f32,
    ) -> Result<()> {
        let matrix = [
            [scale * rotation.cos(), scale * rotation.sin(), 0.0],
            [-scale * rotation.sin(), scale * rotation.cos(), 0.0],
            [x, y, 1.0f32],
        ];
        let uniform = UniformsStorage::new("matrix", matrix);
        let uniform = uniform.add("tex", &helper.texture);
        let uniform = uniform.add("screen_size", [width, height]);

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            ..DrawParameters::default()
        };

        frame.draw(
            &helper.vertex_buffer,
            &helper.index_buffer,
            textured_program,
            &uniform,
            &draw_parameters,
        )?;
        Ok(())
    }

    pub fn draw_rectangle(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    ) -> Result<()> {
        if let Some(frame) = &mut self.frame {
            let uniform = UniformsStorage::new("offset", [x, y]);
            let uniform = uniform.add("dimensions", [width, height]);
            let uniform = uniform.add("color", color);
            let uniform = uniform.add("screen_size", [self.width as f32, self.height as f32]);
            frame.draw(
                &self.rectangle_vertex_buffer,
                &self.rectangle_index_buffer,
                &self.color_program,
                &uniform,
                &DrawParameters::default(),
            )?;
        }
        Ok(())
    }
}
