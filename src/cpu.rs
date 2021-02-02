use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fmt::Error;

pub struct Chip8Program {
    v: [i8; 16],
    dt: u8,
    st: u8,
    pc: u16,
    mem: [u8; 0xFFF],
}

impl Chip8Program {
    pub fn from(mut file: File) -> Self {
        let mut program = Chip8Program {
            v: Default::default(),
            dt: Default::default(),
            st: Default::default(),
            pc: Default::default(),
            mem: [0; 0xFFF],
        };
        file.read(&mut program.mem[0x200..]).unwrap();
        program
    }
    pub fn run_program(&mut self) {}
}