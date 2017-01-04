use glium::{Blend, DrawParameters, IndexBuffer, Frame, VertexBuffer, Surface};
use glium::texture::{RawImage2d, Texture2d};
use glium::uniforms::UniformsStorage;
use glium::index::PrimitiveType;
use std::io::Cursor;
use image;

use super::Result;
use super::Engine;

static mut DRAW_HELPER_ID: u64 = 1;

pub struct DrawHelper<'a> {
    pub id: u64,
    pub draw_parameters: DrawParameters<'a>,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u8>,
    pub texture: Texture2d,
    pub width: f32,
    pub height: f32,
}

impl<'a> DrawHelper<'a> {
    pub fn new(
        engine: &Engine<'a>,
        width: f32,
        height: f32,
        texture: &[u8]
    ) -> Result<DrawHelper<'a>> {
        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            .. DrawParameters::default()
        };

        let id = unsafe {
            let id = DRAW_HELPER_ID;
            DRAW_HELPER_ID += 1;
            id
        };

        let vertex_buffer = VertexBuffer::new(&engine.display, &[
            Vertex {
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
            }
        ])?;

        let index_buffer = IndexBuffer::<u8>::new(&engine.display, PrimitiveType::TriangleStrip, &[0, 1, 2, 3])?;

        let image = image::load(Cursor::new(texture), image::PNG)?.to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = Texture2d::new(&engine.display, image)?;

        Ok(DrawHelper {
            id: id,
            draw_parameters: draw_parameters,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture: texture,
            width: width,
            height: height,
        })
    }

    pub fn draw_at(&self, engine: &mut Engine, frame: &mut Frame, x: f32, y: f32, _rotation: f32,_scale: f32) -> Result<()> {
        //let uniform = UniformsStorage::new("matrix", [
        //    [1.0, 0.0, 0.0],
        //    [0.0, 1.0, 0.0],
        //    [x, y, 1f32]
        //]);
        let uniform = UniformsStorage::new("offset_position", [x, y]);
        let uniform = uniform.add("tex", &self.texture);
        let uniform = uniform.add("screen_size", [engine.width as f32, engine.height as f32]);
        //let uniform = uniform.add("")
        
        frame.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &engine.program,
            &uniform,
            &self.draw_parameters
        )?;

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);