use glium::{Blend, DisplayBuild, DrawParameters, IndexBuffer, Frame, VertexBuffer, Program, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::uniforms::UniformsStorage;
use glium::glutin::WindowBuilder;
use glium::index::PrimitiveType;
use std::collections::HashMap;

use super::{DrawHelper, Result, TGraphicIndex};

#[derive(Copy, Clone)]
struct Vertex {
    pub dimension_affinity: [f32; 2],
}

implement_vertex!(Vertex, dimension_affinity);

pub struct EngineGraphics<T: TGraphicIndex> {
    pub display: GlutinFacade,
    pub textured_program: Program,
    pub color_program: Program,
    pub frame: Option<Frame>,
    pub width: f32,
    pub height: f32,

    graphics: HashMap<T, DrawHelper>,

    rectangle_vertex_buffer: VertexBuffer<Vertex>,
    rectangle_index_buffer: IndexBuffer<u8>,
}

impl<T: TGraphicIndex> EngineGraphics<T> {
    pub fn new(width: f32, height: f32) -> Result<EngineGraphics<T>> {
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

            graphics: HashMap::new(),

            rectangle_vertex_buffer: rectangle_vertex_buffer,
            rectangle_index_buffer: rectangle_index_buffer
        })
    }

    pub fn load_graphic(&mut self, key: T, bytes: &[u8], width: f32, height: f32) -> Result<()> {
        let param = DrawHelper::new(self, width, height, bytes)?;
        self.graphics.insert(key, param);
        Ok(())
    }

    pub fn draw(&mut self, key: T, x: f32, y: f32, rotation: f32, scale: f32) -> Result<()> {
        if let Some(ref helper) = self.graphics.get(&key) {
            if let Some(ref mut frame) = self.frame {
                let matrix = [[scale * rotation.cos(), scale * rotation.sin(), 0.0],
                            [-scale * rotation.sin(), scale * rotation.cos(), 0.0],
                            [x, y, 1.0f32]];
                let uniform = UniformsStorage::new("matrix", matrix);
                let uniform = uniform.add("tex", &helper.texture);
                let uniform = uniform.add("screen_size", [self.width as f32, self.height as f32]);

                let draw_parameters = DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..DrawParameters::default() 
                };

                frame.draw(
                    &helper.vertex_buffer,
                    &helper.index_buffer,
                    &self.textured_program,
                    &uniform,
                    &draw_parameters
                )?;
            }
        } 
        Ok(())
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: (f32, f32, f32, f32)) -> Result<()> {
        if let Some(ref mut frame) = self.frame {
            let uniform = UniformsStorage::new("offset", [x, y]);
            let uniform = uniform.add("dimensions", [width, height]);
            let uniform = uniform.add("color", color);
            let uniform = uniform.add("screen_size", [self.width as f32, self.height as f32]);
            frame.draw(
                &self.rectangle_vertex_buffer,
                &self.rectangle_index_buffer,
                &self.color_program,
                &uniform,
                &DrawParameters::default()
            )?;
        }
        Ok(())
    }
}
