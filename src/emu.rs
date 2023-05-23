const PROGRAM_START: u16 = 0x200;
const SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
];

use crate::cpu::Cpu;

pub struct Emu {
    cpu: Cpu,    
}

impl Emu {
   pub fn new() -> Emu {
        Self {
            cpu: Cpu::new(PROGRAM_START),
        }
    }

    pub fn init(&mut self) {
        // load font sprites to address 0
        self.load_fonts();
    }

    pub fn load_rom(&mut self, data: &[u8]) {        
        let mut i: u16 = 0; 
        for byte in data {
            self.cpu.bus.write_mem_byte(PROGRAM_START + i, *byte);
            i += 1;
        }
    }

    pub fn key_pressed(& mut self, code: usize, released: bool) {
        self.cpu.keys_map[code] = released;
    }

    pub fn run_instruction(&mut self) {
        let _nextaddr = self.cpu.run_instruction();
    }

    pub fn tick_timers(&mut self) {
        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1;
        }
        if self.cpu.sound_timer > 0 {
            self.cpu.sound_timer -= 1;
        }
    }

    pub fn get_display_buffer(&self) -> &[bool] {
        self.cpu.bus.get_display_buffer()
    }

    fn load_fonts(&mut self) {
        let mut addr: u16 = 0;
        for sprite in SPRITES {
            for byte in sprite {
                self.cpu.bus.write_mem_byte(addr, byte);
                addr += 1;
            }
        }
    }
   
}