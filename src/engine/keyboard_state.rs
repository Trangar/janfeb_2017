use glium::glutin::VirtualKeyCode;

#[derive(Default)]
pub struct KeyboardState {
    pressed_keys: Vec<VirtualKeyCode>,
    pressed_keys_this_frame: Vec<VirtualKeyCode>,
    released_keys_this_frame: Vec<VirtualKeyCode>,
}

#[allow(dead_code)]
impl KeyboardState {
    pub fn frame_start(&mut self) {
        self.pressed_keys_this_frame.clear();
        self.released_keys_this_frame.clear();
    }
    pub fn set_keydown(&mut self, key: VirtualKeyCode) {
        if !self.is_keydown(key) {
            self.pressed_keys.push(key);
            self.pressed_keys_this_frame.push(key);
        }
    }

    pub fn clear_keydown(&mut self, key: VirtualKeyCode) {
        self.pressed_keys.retain(|k| *k != key);
        self.released_keys_this_frame.push(key);
    }

    pub fn is_keydown(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.iter().position(|k| *k == key).is_some()
    }

    pub fn is_pressed_this_frame(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys_this_frame.iter().position(|k| *k == key).is_some()
    }

    pub fn is_released_this_frame(&self, key: VirtualKeyCode) -> bool {
        self.released_keys_this_frame.iter().position(|k| *k == key).is_some()
    }
}
