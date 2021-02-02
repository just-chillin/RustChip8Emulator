use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fmt::Error;
use crate::isa::Instruction;

pub struct Memory([u8; 0xFFF]);

impl Memory {
    pub fn get_instruction(&self, addr: usize) -> Instruction {
        let offset = addr * 2;
        Instruction::from([self.0[offset], self.0[offset+1]])
    }
}

pub struct Program {
    v: [i8; 16],
    dt: u8,
    st: u8,
    pc: u16,
    mem: Memory,
}

impl Program {
    pub fn from(mut file: File) -> Self {
        let mut program = Self {
            v: Default::default(),
            dt: Default::default(),
            st: Default::default(),
            pc: 0x200,
            mem: Memory([0; 0xFFF]),
        };
        file.read(&mut program.mem.0[0x200..]).unwrap();
        program
    }


    pub fn run_program(&mut self) {
        loop {
            let inst = self.mem.get_instruction(self.pc as usize);

        }
    }

    fn exec(&mut self, instruction: Instruction) {

    }
}