pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT : usize = 32;

pub struct Display {
    buffer: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]
}

impl Display {

    pub fn new() -> Display {
        Self {
            buffer: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }
  
    pub fn clear(&mut self) {
        self.buffer = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }

    pub fn set_pixel(&mut self, index: usize) {
        self.buffer[index] ^= true;
    }

    pub fn get_pixel(&self, index: usize) -> bool {
        self.buffer[index]
    }
    
    pub fn get_buffer(&self) -> &[bool] {
        &self.buffer
    }
}

