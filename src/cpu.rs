use std::fmt::{Debug, Error, Formatter};
use std::fs::File;
use std::io::Read;

use rand::prelude::*;

use crate::isa::Instruction;
use std::{fmt, panic};

const MEM_SIZE: usize = 0xFFF;
const PROG_START: usize = 0x200;

pub struct Memory([u8; MEM_SIZE]);

impl Memory {
    pub fn get_instruction(&self, addr: usize) -> Result<Instruction, String> {
        Instruction::try_from([self.0[addr], self.0[addr + 1]])
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>)  -> fmt::Result {
        write!(f, "Memory[..]")
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
            writeln!(f, "V{:X}: {}", i, val);
        };
        writeln!(f, "dt = {}", self.dt);
        writeln!(f, "st = {}", self.st);
        writeln!(f, "pc = {}", self.pc);
        writeln!(f, "I = {}", self.i);
        writeln!(f, "stack: {:#?}", self.stack)
    }
}
impl Program {
    pub fn from(mut file: File) -> Self {
        let mut program = Self {
            v: Default::default(),
            dt: 0,
            st: 0,
            pc: PROG_START,
            stack: vec![],
            i: 0,
            rng: rand::thread_rng(),
            mem: Memory([0; MEM_SIZE]),
        };
        file.read(&mut program.mem.0[PROG_START..]).unwrap();
        program
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
            Instruction::SYS { addr } => self.pc = addr,
            Instruction::CLS => todo!(),
            Instruction::RET => self.pc = self.stack.pop().expect("The stack was empty!"),
            Instruction::JP { addr } => self.pc = addr,
            Instruction::CALL { addr } => {
                self.stack.push(self.pc);
                self.pc = addr
            }
            Instruction::SEI { vx, imm } => {
                if self.v[vx] == imm as u8 {
                    self.pc += 2
                }
            }
            Instruction::SNEI { vx, imm } => {
                if self.v[vx] != imm as u8 {
                    self.inc_pc()
                }
            }
            Instruction::SE { vx, vy } => {
                if self.v[vx] == self.v[vy] {
                    self.inc_pc()
                }
            }
            Instruction::LDI { vx, imm } => self.v[vx] = imm,
            Instruction::ADDI { vx, imm } => self.v[vx] += imm,
            Instruction::LD { vx, vy } => self.v[vx] = self.v[vy],
            Instruction::OR { vx, vy } => self.v[vx] |= self.v[vy],
            Instruction::AND { vx, vy } => self.v[vx] &= self.v[vy],
            Instruction::XOR { vx, vy } => self.v[vx] ^= self.v[vy],
            Instruction::ADD { vx, vy } => {
                self.v[0xF] = u8::from(((self.v[vx] as u16) + (self.v[vy] as u16)) < 255);
                self.v[vx] = self.v[vx].wrapping_add(self.v[vy])
            }
            Instruction::SUB { vx, vy } => {
                self.v[0xF] = u8::from(self.v[vx] > self.v[vy]);
                self.v[vx] = self.v[vx].wrapping_sub(self.v[vy])
            }
            Instruction::SHR { vx: _, vy: _ } => todo!(),
            Instruction::SUBN { vx, vy } => {
                self.v[0xF] = u8::from(self.v[vy] > self.v[vx]);
                self.v[vx] = self.v[vy].wrapping_sub(self.v[vx])
            }
            Instruction::SHL { .. } => todo!(),
            Instruction::SNE { vx, vy } => {
                if self.v[vx] != self.v[vy] {
                    self.pc += 2
                }
            }
            Instruction::LDA { addr } => self.i = addr,
            Instruction::JPO { addr } => self.pc = addr + self.v[0] as usize,
            Instruction::RND { vx, imm } => self.v[vx] = self.rng.gen::<u8>() & imm,
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
