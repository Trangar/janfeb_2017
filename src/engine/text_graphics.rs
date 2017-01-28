use glium_text::{draw, FontTexture, TextDisplay, TextSystem};
use glium::backend::glutin_backend::GlutinFacade;
use std::collections::HashMap;
use super::{Color, Result};
use std::rc::Rc;
use glium::Frame;


pub struct TextGraphics {
    pub system: TextSystem,
    pub font: Rc<FontTexture>,
    pub cache: HashMap<String, TextDisplay<Rc<FontTexture>>>,
    pub unused_keys: Vec<String>,
}

#[allow(dead_code)]
impl TextGraphics {
    pub fn new(display: &GlutinFacade) -> Result<TextGraphics> {
        let system = TextSystem::new(display);
        let font =
            Rc::new(FontTexture::new(display, &include_bytes!("../../assets/arial.ttf")[..], 16)?);
        Ok(TextGraphics {
            system: system,
            font: font,
            cache: HashMap::new(),
            unused_keys: Vec::new(),
        })
    }

    pub fn frame_end(&mut self) {
        for key in &self.unused_keys {
            self.cache.remove(key);
        }
        self.unused_keys.clear();
        for key in self.cache.keys() {
            self.unused_keys.push(key.clone());
        }
    }

    pub fn draw_at(&mut self,
                   frame: &mut Frame,
                   name: String,
                   screen_width: f32,
                   screen_height: f32,
                   x: f32,
                   y: f32,
                   color: Color)
                   -> Result<()> {
        self.unused_keys.retain(|k| *k != name);

        let horizontal_scale = 0.025f32;
        let vertical_scale = 0.05f32;

        const VERTICAL_OFFSET: f32 = 12f32;

        let x = (x / screen_width) * 2f32 - 1f32;
        let y = 1f32 - ((y + VERTICAL_OFFSET) / screen_height) * 2f32;
        // let x = 0.5f32;
        // let y = 0.5f32;
        let matrix = [[horizontal_scale, 0.0, 0.0, 0.0],
                      [0.0, vertical_scale, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [x, y, 0.0, 1.0]];

        if let Some(ref text) = self.cache.get(&name) {
            self.unused_keys.retain(|s| *s == name);
            draw(text, &self.system, frame, matrix, color);
            return Ok(());
        }
        let text = TextDisplay::new(&self.system, self.font.clone(), &name);
        draw(&text, &self.system, frame, matrix, color);
        self.cache.insert(name, text);
        Ok(())
    }
}
