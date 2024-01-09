use std::fs::File;
use std::io::prelude::*;

pub struct Chip8 {
    pub ram: [u8; 4096],
    pub pc: u16,
    pub i: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v: [u8; 16],
    pub pixels: [[bool; 64]; 32],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            ram: [0x00; 4096],
            pc: 0x200,
            i: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            // TODO: Re-implement this as a bitboard for efficiency.
            pixels: [[false; 64]; 32],
        }
    }

    /// Creates or overwrites the file `rom.ch8` in the current directory and
    /// fills it with the raw binary data currently in the RAM, starting at
    /// memory address 0x200 (512). A CHIP-8 program will often have a large
    /// number of trailing zeroes - this function removes those.
    pub fn dump_rom(&self) -> std::io::Result<()> {
        let mut file = File::create("rom.ch8")?;
        let mut ram_vector = self.ram.to_vec();
        while ram_vector.len() != 0 {
            // Unwrap here is safe because it's guaranteed that the length of
            // the vector is greater than zero.
            let val = ram_vector.pop().unwrap();
            if val != 0 {
                ram_vector.push(val);
                break;
            }
        }
        file.write_all(&ram_vector)?;
        Ok(())
    }

    /// Reads a CHIP-8 ROM and inserts it into RAM. ROM is stored as pure binary
    /// data in a file conventionally ending in `.ch8`. Insertion starts at the
    /// memory address 0x200 (512) - the first 512 bytes of memory were
    /// historically used for the CHIP-8 interpreter itself.
    pub fn read_rom(&mut self, filename: &str) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut ram_vector: Vec<u8> = vec![];
        file.read_to_end(&mut ram_vector)?;
        for (pos, val) in ram_vector.iter().enumerate() {
            self.ram[pos + 512] = *val;
        }
        Ok(())
    }

    pub fn fetch_curr_instruction(&self) -> u16 {
        let i1: u16 = self.ram[self.pc as usize].into();
        let i2: u16 = self.ram[(self.pc + 1) as usize].into();
        let i: u16 = (i1 << 8) + i2;
        i
    }

    /// 00E0: Clear screen. Sets all pixels to black.
    pub fn clear_screen(&mut self) {
        self.pixels = [[false; 64]; 32];
    }

    /// 1NNN: Set PC to NNN.
    pub fn jump(&mut self, xyz: u16) {
        self.pc = xyz;
    }

    /// 6XNN: Set register `v[X]` to NN.
    pub fn set(&mut self, register: u16, nn: u16) {
        self.v[register as usize] = nn as u8;
    }

    /// 0xANNN: Set the index register to NNN.
    pub fn set_index(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /// 7XNN: Add NN to register `v[X]`. This implementation assumes that addition wraps rather than saturates (i.e. 255 + 1 = 0, rather than 255). This function also does not set CHIP-8's carry flag.
    pub fn add(&mut self, register: u16, nn: u16) {
        self.v[register as usize] = self.v[register as usize].wrapping_add(nn as u8);
    }

    /// DXYN: Display a sprite on the screen. The sprite is 8 pixels wide and N pixels tall.
    pub fn display(&mut self, x: u16, y: u16, n: u16) {
        let init_x: usize = (self.v[x as usize] % 64).into();
        let init_y: usize = (self.v[y as usize] % 64).into();

        let mut curr_x = init_x;
        let mut curr_y = init_y;

        self.v[0xF] = 0;
        for offset in 0..n {
            let sprite_row = self.ram[(self.i + offset) as usize];
            for shift in 0..8 {
                if sprite_row & (0b10000000 >> shift) != 0 {
                    if self.pixels[curr_y][curr_x] {
                        self.pixels[curr_y][curr_x] = false;
                    } else {
                        self.pixels[curr_y][curr_x] = true;
                    }
                }
                if curr_x == 63 {
                    break;
                } else {
                    curr_x += 1;
                }
            }
            if curr_y == 31 {
                break;
            } else {
                curr_y += 1;
                curr_x = (self.v[x as usize] % 64).into();
            }
        }
    }

    // /// 2NNN: Start subroutine at NNN. Push current PC onto stack.
    // pub fn enter_subroutine(&mut self, xyz: u16) {
    //     self.stack.push(self.pc);
    //     self.pc = xyz;
    // }

    // pub fn return_subroutine(&mut self) {
    //     match self.stack.pop() {
    //         Some(address) => self.pc = address,
    //         None => panic!("Attempt to pop from CHIP-8's stack despite it being empty."),
    //     }
    // }
}
