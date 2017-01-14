use glium::{Blend, DrawParameters, IndexBuffer, VertexBuffer, Surface};
use glium::texture::{RawImage2d, Texture2d};
use glium::uniforms::UniformsStorage;
use glium::index::PrimitiveType;
use std::io::Cursor;
use image;

use super::{EngineGraphics, Result};

static mut DRAW_HELPER_ID: u64 = 1;

pub struct DrawHelper {
    pub id: u64,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u8>,
    pub texture: Texture2d,
    pub width: f32,
    pub height: f32,
}

impl DrawHelper {
    pub fn new(engine: &EngineGraphics,
               width: f32,
               height: f32,
               texture: &[u8])
               -> Result<DrawHelper> {
        let id = unsafe {
            let id = DRAW_HELPER_ID;
            DRAW_HELPER_ID += 1;
            id
        };

        let vertex_buffer = VertexBuffer::new(&engine.display,
                                              &[Vertex {
                                                    position: [-width / 2f32, -height / 2f32],
                                                    tex_coords: [0.0, 0.0],
                                                },
                                                Vertex {
                                                    position: [width / 2f32, -height / 2f32],
                                                    tex_coords: [1.0, 0.0],
                                                },
                                                Vertex {
                                                    position: [-width / 2f32, height / 2f32],
                                                    tex_coords: [0.0, 1.0],
                                                },
                                                Vertex {
                                                    position: [width / 2f32, height / 2f32],
                                                    tex_coords: [1.0, 1.0],
                                                }])?;

        let index_buffer =
            IndexBuffer::<u8>::new(&engine.display, PrimitiveType::TriangleStrip, &[0, 1, 2, 3])?;

        let image = image::load(Cursor::new(texture), image::PNG)?.to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = Texture2d::new(&engine.display, image)?;

        Ok(DrawHelper {
            id: id,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture: texture,
            width: width,
            height: height,
        })
    }

    pub fn draw_at(&self,
                   graphics: &mut EngineGraphics,
                   x: f32,
                   y: f32,
                   rotation: f32,
                   scale: f32)
                   -> Result<()> {
        let matrix = [[scale * rotation.cos(), scale * rotation.sin(), 0.0],
                      [-scale * rotation.sin(), scale * rotation.cos(), 0.0],
                      [x, y, 1.0f32]];
        let uniform = UniformsStorage::new("matrix", matrix);
        let uniform = uniform.add("tex", &self.texture);
        let uniform = uniform.add("screen_size",
                                  [graphics.width as f32, graphics.height as f32]);

        let draw_parameters =
            DrawParameters { blend: Blend::alpha_blending(), ..DrawParameters::default() };

        if let Some(ref mut frame) = graphics.frame {
            frame.draw(&self.vertex_buffer,
                      &self.index_buffer,
                      &graphics.textured_program,
                      &uniform,
                      &draw_parameters)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
