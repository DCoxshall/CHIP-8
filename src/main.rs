use chip8::Chip8;
use macroquad::prelude::*;

mod chip8;
mod gui;

use gui::render;

#[macroquad::main("CHIP-8")]
async fn main() {
    let mut chip = Chip8::new();

    chip.dump_rom();

    match chip.read_rom("src/roms/font_test.ch8") {
        Ok(()) => {}
        Err(e) => panic!("{}", e),
    }

    loop {
        // Fetch
        let instruction = chip.fetch_curr_instruction();
        chip.pc += 2;
        // Decode
        // The instruction is a u16 - get the first, second, third and fourth
        // nibbles. `wxyz` represent these nibbles in all variable names.
        let w: u16 = (instruction & 0xF000) >> 12;
        let x: u16 = (instruction & 0x0F00) >> 8;
        let y: u16 = (instruction & 0x00F0) >> 4;
        let z: u16 = (instruction & 0x000F) >> 0;

        // The second byte (third and fourth nibbles). An 8-bit immediate number.
        let yz: u16 = instruction & 0x00FF;

        // The second, third and fourth nibbles. A 12-bit immediate memory address.
        let xyz: u16 = instruction & 0x0FFF;

        // Execute
        match w {
            0x0 => match xyz {
                0x0E0 => {
                    chip.clear_screen();
                    gui::render(chip.pixels);
                }
                //0x0EE => chip.return_subroutine(),
                _ => panic!("Invalid instruction read."),
            },
            0x1 => chip.jump(xyz),
            0x6 => chip.set(x, yz),
            0x7 => chip.add(x, yz),
            //0x2 => chip.enter_subroutine(xyz),
            0xA => chip.set_index(xyz),
            0xD => {
                chip.display(x, y, z);
            }
            _ => panic!("Invalid instruction read."),
        }
        next_frame().await;
        render(chip.pixels);
    }
}
