use crate::memory::Memory;
use crate::display::{Display, DISPLAY_WIDTH, DISPLAY_HEIGHT};

pub struct Bus {
    memory: Memory,
    display: Display,
}

impl Bus {
    pub fn new() -> Bus {
        Self {
            memory: Memory::new(),
            display: Display::new(),
        }
    }

    pub fn write_mem_byte(&mut self, addr: u16, byte: u8) {
        self.memory.write_byte(addr, &byte);
    }

    pub fn read_mem_byte(&mut self, addr: u16) -> u8 {
        self.memory.read_byte(addr)
    }

    pub fn clear_display(&mut self) {
        println!("clear display");
        self.display.clear();
    }

    pub fn draw_sprite(&mut self, x_coord: u16, y_coord: u16, n: u8, start_addr: u16) -> bool {
        let mut flipped = false;
        for row in 0..n as u16  {
            let addr = start_addr + row as u16;
            let pixels = self.memory.read_byte(addr);
            for col in 0..8 {
                if (pixels & (0x80 >> col)) != 0 {
                    // wrap coordinates
                    let x = (x_coord + col) as usize  % DISPLAY_WIDTH;
                    let y = (y_coord + row)  as usize % DISPLAY_HEIGHT;
                    let index = x + y * DISPLAY_WIDTH;
                    flipped |= self.display.get_pixel(index);
                    self.display.set_pixel(index);
                }
            }            
        }
        flipped
    }

    pub fn get_display_buffer(&self) -> &[bool]{
        self.display.get_buffer()
    }    

 }