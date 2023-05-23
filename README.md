# Introduction

This repository hosts a chip-8 emulator I wrote while rampping up with Rust programming. I found this [this tutorial](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) very helpful while implmenting the CPU. 
The rom folder contains chip8 game pack downloaded from [zophar's domain](https://www.zophar.net/pdroms/chip8.html).

# How to run

This program uses [sdl2 crate](https://crates.io/crates/sdl2) for screen rendering and keypad input. You would need to copy libsdl development library files to your Rust installation folder before building this emulator. Please see [rust-sdl2 documentation](https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md) for more infromation.  