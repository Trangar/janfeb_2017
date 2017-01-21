use glium::{IndexBuffer, VertexBuffer};
use glium::texture::{RawImage2d, Texture2d};
use glium::index::PrimitiveType;
use std::io::Cursor;
use image;

use super::{EngineGraphics, Result, TGraphicIndex};

pub struct DrawHelper {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u8>,
    pub texture: Texture2d,
    pub width: f32,
    pub height: f32,
}

impl DrawHelper {
    pub fn new<T: TGraphicIndex>(engine: &EngineGraphics<T>,
               width: f32,
               height: f32,
               texture: &[u8])
               -> Result<DrawHelper> {

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
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture: texture,
            width: width,
            height: height,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
