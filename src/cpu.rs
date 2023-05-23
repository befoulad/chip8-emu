use crate::bus::Bus;
use rand::Rng;

const NUM_KEYS: usize = 16;

pub struct Cpu {
    pub bus: Bus,
    vx: [u8; 16],
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    pub keys_map: [bool; NUM_KEYS],
    pub delay_timer: u8,
    pub sound_timer: u8
}

impl Cpu {
    pub fn new(start_addr: u16) -> Cpu {
        Self {
            bus: Bus::new(),
            vx: [0;16],
            pc: start_addr,
            i: 0,
            stack: Vec::<u16>::new(),
            keys_map: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0
        }
    } 

    pub fn run_instruction(&mut self) -> u16{
        // fetch opcode
        let first_byte = self.bus.read_mem_byte(self.pc);
        let second_byte = self.bus.read_mem_byte(self.pc + 1);
        let opcode: u16 = u16::from_be_bytes([first_byte, second_byte]);
        // run
        //println!("{:#X}", opcode);
        let nibble =  ((opcode >> 8) >> 4) & 0xf;
        let operands = opcode & 0xfff;
        let x = ((opcode >> 8) & 0xf) as usize;
        let y = ((opcode >> 4) & 0xf) as usize;
        let nnn = opcode & 0xfff;
        let nn = (opcode & 0xff) as u8;
        let n = (opcode & 0xf) as u8;

        match nibble {
            0x0 => {
                if operands == 0xe0 {
                    self.bus.clear_display();
                    self.pc += 2; 
                }
                // RET
                else if operands == 0xee {
                    let ret_addr = self.stack.pop().unwrap();
                    self.pc = ret_addr;
                }
            }
            // JUMP
            0x1 => {
                self.pc = nnn;
            },
            // CALL
            0x2 => {
                self.stack.push(self.pc + 2);
                self.pc = nnn;
            },
            // SKIP
            0x3 => {
                if self.vx[x] == nn {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            // SKIP
            0x4 => {
                if self.vx[x] != nn {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            // SKIP
            0x5 => {
                if self.vx[x] == self.vx[y] {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            // SET REG
            0x6 => {
                self.vx[x] = nn;
                self.pc += 2;         
            },
            // ADD nn
            0x7 => {
                self.vx[x] = self.vx[x].wrapping_add(nn);
                self.pc += 2;
            },
            // LOGICAL OPS
            0x8 => {
                match n {
                   0 => {self.vx[x] = self.vx[y]},
                   1 => {self.vx[x] = self.vx[x] | self.vx[y]},
                   2 => {self.vx[x] = self.vx[x] & self.vx[y]},
                   3 => {self.vx[x] = self.vx[x] ^ self.vx[y]},
                   4 => {self.vx[x] = self.vx[x].wrapping_add(self.vx[y])},
                   5 => {                        
                        let res = self.vx[x] as i8 - self.vx[y] as i8;
                        self.vx[x] = res as u8;
                        if res > 0 {
                            self.vx[0xf] = 1;
                        }
                        else {
                            self.vx[0xf] = 0;
                        }
                    },
                    6 => {
                        let bit = self.vx[x] & 0x1;
                        self.vx[x] = self.vx[x] >> 1;
                        if bit == 1 {
                            self.vx[0xf] = 1;
                        }
                        else {
                            self.vx[0xf] = 0;
                        }
                    },
                    7 => {
                        let res = self.vx[y] as i8 - self.vx[x] as i8;
                        self.vx[x] = res as u8;
                        if res < 0 {
                            self.vx[0xf] = 1;
                        }
                        else {
                            self.vx[0xf] = 0;
                        } 
                    },
                    0xe => {
                        let bit = (self.vx[x] & 0x80) >> 7;
                        self.vx[x] = self.vx[x] << 1;
                        if bit == 1 {
                            self.vx[0xf] = 1;
                        }
                        else {
                            self.vx[0xf] = 0;
                        }
                    }                    
                   _ => {panic!("invalid 0x8XYN opcode: {:#x}", opcode)}                
                }
                self.pc += 2;
            },
            // SKIP
            0x9 => {
                if self.vx[x] != self.vx[y] {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            // SET I
            0xa => {
                self.i = nnn;
                self.pc += 2;
            },
            // JUMP OFFSET
            0xb => {
                let loc = nnn + self.vx[0] as u16;
                self.pc = loc;
            },
            // RND
            0xc => {
                let rnd:u8 = rand::thread_rng().gen();
                self.vx[x] = rnd & nn;
                self.pc += 2;
            },         
            0xd => {
                let n = (operands & 0xf) as u8;
                let start_addr = self.i;
                let x = self.vx[x as usize] as u16;
                let y = self.vx[y as usize] as u16;       
                let should_set_vf = self.bus.draw_sprite(x, y, n, start_addr);
                if should_set_vf {
                    self.vx[0xf] = 1;
                }
                else {
                    self.vx[0xf] = 0;
                }
                self.pc += 2;
            },
            0xe => {
                match nn {
                    0xa1 => {
                        // is key not pressed?
                        let idx = self.vx[x] as usize;
                        if !self.keys_map[idx] {
                            self.pc += 4;
                        }
                        else {
                            self.pc += 2;
                        }
                    }
                    0x9e => {
                        // is key pressed?
                        let idx = self.vx[x] as usize;
                        if self.keys_map[idx] {
                            self.pc += 4;
                        }
                        else {
                            self.pc += 2;
                        } 
                    },
                    _ => {panic!("invalid EXNN opcode: {:#x}", opcode)}
                }              
            }
            // Timer
            0xf => {
                match nn {
                    0x7 => {self.vx[x] = self.delay_timer},
                    0xa => {
                        println!("waiting for keypad input");
                        // wait for a keypad input
                        let mut pressed = false;
                        for i in 0..NUM_KEYS {
                            if self.keys_map[i] {
                                self.vx[x] = i as u8;
                                println!("keypad input received: {}", self.vx[x]);
                                pressed = true;
                            }                            
                        }
                        // repeat the same instruction
                        if !pressed {
                            self.pc -= 2;
                        }                        
                    },
                    0x15 => {self.delay_timer = self.vx[x]},
                    0x18 => {self.sound_timer = self.vx[x]},
                    0x1e => {self.i = self.i.wrapping_add(self.vx[x] as u16)},
                    0x29 => {self.i = (self.vx[x] as u16) * 5; },
                    0x33 => {
                        let mut n = self.vx[x];
                        let mut k= 0;
                        while k < 3 {
                            let digit = n % 10;
                            self.bus.write_mem_byte(self.i + (2 - k) , digit);
                            n = n / 10;
                            k += 1;
                        }                    
                    }
                    0x55 => {
                        for index in 0..=x {
                            self.bus.write_mem_byte(self.i + index as u16, self.vx[index]);
                        }                        
                    },
                    0x65 => {
                        for index in 0..=x {
                            let val = self.bus.read_mem_byte(self.i + index as u16);
                            self.vx[index] = val;
                        }
                    }
                    _ => {panic!("invalid FXNN opcode: {:#x}", opcode)}
                }
                self.pc += 2;
            }
            _ => {panic!("invalid opcode: {:#x} at {:#02x}", opcode, self.pc);}
        }
        self.pc
    } 
   
}