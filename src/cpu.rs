use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::iter;
use rand::prelude::*;

use crate::isa::Instruction;
use std::{fmt, panic};

const PROG_START: usize = 0x200;

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn get_instruction(&self, addr: usize) -> Result<Instruction, String> {
        Instruction::try_from([self.0[addr], self.0[addr + 1]])
    }
}

pub struct Program {
    v: [u8; 16],
    dt: u8,
    st: u8,
    pc: usize,
    i: usize,
    rng: ThreadRng,
    stack: Vec<usize>,
    mem: Memory,
}
impl Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, val) in self.v.iter().enumerate() {
            writeln!(f, "V{:X}: {}", i, val).unwrap();
        }
        writeln!(f, "dt = {}", self.dt).unwrap();
        writeln!(f, "st = {}", self.st).unwrap();
        writeln!(f, "pc = {}", self.pc).unwrap();
        writeln!(f, "I = {}", self.i).unwrap();
        writeln!(f, "stack: {:#?}", self.stack)
    }
}
impl Program {
    pub fn from(mut file: File) -> Self {
        let mem = {
            let mut preamble = vec![0u8; PROG_START];
            let _ = file.read_to_end(&mut preamble).unwrap();
            preamble.append(&mut vec![0u8; 0xFFF - preamble.len()]);
            Memory(preamble)
        };
        Self {
            v: Default::default(),
            dt: 0,
            st: 0,
            pc: PROG_START,
            stack: vec![],
            i: 0,
            rng: rand::thread_rng(),
            mem,
        }
    }

    pub fn run_program(&mut self) {
        loop {
            let inst = self.mem.get_instruction(self.pc as usize);
            if inst.is_err() {
                panic!(inst);
            }
            let inst = inst.unwrap();
            println!("{:?}", inst);
            self.exec(inst);
        }
    }

    fn inc_pc(&mut self) {
        self.pc += 2
    }

    fn exec(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SYS { addr } => self.pc = addr.0,
            Instruction::CLS => todo!(),
            Instruction::RET => self.pc = self.stack.pop().expect("The stack was empty!"),
            Instruction::JP { addr } => self.pc = addr.0,
            Instruction::CALL { addr } => {
                self.stack.push(self.pc);
                self.pc = addr.0 - 2
            }
            Instruction::SEI { vx, imm } => {
                if self.v[vx.0] == imm as u8 {
                    self.pc += 2
                }
            }
            Instruction::SNEI { vx, imm } => {
                if self.v[vx.0] != imm as u8 {
                    self.inc_pc()
                }
            }
            Instruction::SE { vx, vy } => {
                if self.v[vx.0] == self.v[vy.0] {
                    self.inc_pc()
                }
            }
            Instruction::LDI { vx, imm } => self.v[vx.0] = imm,
            Instruction::ADDI { vx, imm } => self.v[vx.0] += imm,
            Instruction::LD { vx, vy } => self.v[vx.0] = self.v[vy.0],
            Instruction::OR { vx, vy } => self.v[vx.0] |= self.v[vy.0],
            Instruction::AND { vx, vy } => self.v[vx.0] &= self.v[vy.0],
            Instruction::XOR { vx, vy } => self.v[vx.0] ^= self.v[vy.0],
            Instruction::ADD { vx, vy } => {
                self.v[0xF] = u8::from(((self.v[vx.0] as u16) + (self.v[vy.0] as u16)) < 255);
                self.v[vx.0] = self.v[vx.0].wrapping_add(self.v[vy.0])
            }
            Instruction::SUB { vx, vy } => {
                self.v[0xF] = u8::from(self.v[vx.0] > self.v[vy.0]);
                self.v[vx.0] = self.v[vx.0].wrapping_sub(self.v[vy.0])
            }
            Instruction::SHR { .. } => todo!(),
            Instruction::SUBN { vx, vy } => {
                self.v[0xF] = u8::from(self.v[vy.0] > self.v[vx.0]);
                self.v[vx.0] = self.v[vy.0].wrapping_sub(self.v[vx.0])
            }
            Instruction::SHL { .. } => todo!(),
            Instruction::SNE { vx, vy } => {
                if self.v[vx.0] != self.v[vy.0] {
                    self.pc += 2
                }
            }
            Instruction::LDA { addr } => self.i = addr.0,
            Instruction::JPO { addr } => self.pc = addr.0 + self.v[0] as usize,
            Instruction::RND { vx, imm } => self.v[vx.0] = self.rng.gen::<u8>() & imm,
            Instruction::DRW { .. } => todo!(),
            Instruction::SKP { .. } => todo!(),
            Instruction::SKNP { .. } => todo!(),
            Instruction::LDDT { .. } => todo!(),
            Instruction::LDKEY { .. } => todo!(),
            Instruction::SETDT { .. } => todo!(),
            Instruction::LDST { .. } => todo!(),
            Instruction::ADDIR { .. } => todo!(),
            Instruction::LDSPR { .. } => todo!(),
            Instruction::LDBCD { .. } => todo!(),
            Instruction::STR { .. } => todo!(),
            Instruction::LDR { .. } => todo!(),
        };
        self.inc_pc()
    }
}
