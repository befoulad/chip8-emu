mod emu;
mod bus;
mod memory;
mod cpu;
mod display;

use std::{fs::File, io::Read, env};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect};
use crate::emu::Emu;
use crate::display::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

// sdl canvas is x20 of chip8 display
const DISPLAY_SCALE: usize = 15;
const CPU_INSTR_PER_CYCLE: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("enter rom file path");
        return;
    }
    let rom_file_path = &args[1];
    let mut file = File::open(rom_file_path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buf) {
        Ok(size) => {
            // initialize sdl2
            let sdl_ctx = sdl2::init().unwrap();
            let mut sdl_evt_pump = sdl_ctx.event_pump().unwrap();
            let video = sdl_ctx.video().unwrap();
            let window = video.window(
                "CHIP-8 Emulator", 
                (DISPLAY_WIDTH * DISPLAY_SCALE) as u32, 
                (DISPLAY_HEIGHT * DISPLAY_SCALE) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
            let mut canvas = window.into_canvas()
            .present_vsync()
            .build()
            .unwrap();
            canvas.clear();
            canvas.present();
            // launch emulator
            let mut emu = Emu::new();
            emu.init();
            emu.load_rom(&buf);
            println!("[*] loaded program file into memory, size: {}", size);
            // process keypad events
            'emu_loop: loop {
                for evt in sdl_evt_pump.poll_iter() {
                    match evt {
                        Event::KeyDown { keycode, .. } => {
                            let keycode = keycode.unwrap();
                            if keycode == Keycode::Escape {
                                println!("exiting");
                                break 'emu_loop;
                            }
                            if let Some(k) = map_keycode(keycode) {
                                emu.key_pressed(k, true);
                            }
                        },
                        Event::KeyUp { keycode, .. } => {
                            let keycode = keycode.unwrap();
                            if let Some(k) = map_keycode(keycode) {                                
                                emu.key_pressed(k, false);
                            }
                        }                       
                        _ => {}
                    }                
                }
                // run batch of CPU instructions
                for _ in 0..CPU_INSTR_PER_CYCLE {
                    emu.run_instruction();
                }
                // update timers
                emu.tick_timers();
                // redraw display
                redraw_display(&mut canvas, &emu);                
            };
               
        },
        Err(_err) => {
            println!("Error reading file");
        }
    }   

}


fn redraw_display(canvas: &mut Canvas<Window>, emu: &Emu) {
    // clear canvas with black color
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    // iterate through pixels and draw
    // active pixels as white
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let buffer = emu.get_display_buffer();  
    for (i, pixel) in buffer.iter().enumerate() {
        if *pixel {
            // convert index to (x,y) coordinates in chip8 plane
            let x = i % DISPLAY_WIDTH;
            let y = i / DISPLAY_WIDTH;
            // draw pixel as a rectangle
            let rect = Rect::new((x * DISPLAY_SCALE) as i32, (y * DISPLAY_SCALE) as i32, DISPLAY_SCALE as u32, DISPLAY_SCALE as u32);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

// 4 x 4 keypad, beginning at 
// top left side of the keyboard
fn map_keycode(keycode: Keycode) -> Option<usize> {
    match keycode {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xc),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None
    }
}