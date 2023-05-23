pub struct Memory {
    mem: [u8; 0x1000]
}

impl Memory {

    pub fn new() -> Memory {
        Self {
            mem: [0; 0x1000]
        }        
    }

    pub fn write_byte(&mut self, addr: u16, byte: &u8) {
        self.mem[usize::from(addr)] = *byte;
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.mem[usize::from(addr)]
    }
    
}