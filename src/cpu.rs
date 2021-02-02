use crate::isa::{Instruction, ISA};
use rand::prelude::*;
use std::fs::File;
use std::io::Read;

const MEM_SIZE: usize = 0xFFF;
const PROG_START: usize = 0x200;

pub struct Memory([u8; MEM_SIZE]);

impl Memory {
    pub fn get_instruction(&self, addr: usize) -> Instruction {
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
            self.exec(inst);
        }
    }

    fn inc_pc(&mut self) {
        self.pc += 2
    }

    fn exec(&mut self, instruction: Instruction) {
        match instruction.0 {
            ISA::SYS { addr } => self.pc = addr,
            ISA::CLS => {}
            ISA::RET => self.pc = self.stack.pop().expect("The stack was empty!"),
            ISA::JP { addr } => self.pc = addr,
            ISA::CALL { addr } => {
                self.stack.push(self.pc);
                self.pc = addr
            }
            ISA::SEI { vx, imm } => {
                if self.v[vx] == imm as u8 {
                    self.pc += 2
                }
            }
            ISA::SNEI { vx, imm } => {
                if self.v[vx] != imm as u8 {
                    self.inc_pc()
                }
            }
            ISA::SE { vx, vy } => {
                if self.v[vx] == self.v[vy] {
                    self.inc_pc()
                }
            }
            ISA::LDI { vx, imm } => self.v[vx] = imm,
            ISA::ADDI { vx, imm } => self.v[vx] += imm,
            ISA::LD { vx, vy } => self.v[vx] = self.v[vy],
            ISA::OR { vx, vy } => self.v[vx] |= self.v[vy],
            ISA::AND { vx, vy } => self.v[vx] &= self.v[vy],
            ISA::XOR { vx, vy } => self.v[vx] ^= self.v[vy],
            ISA::ADD { vx, vy } => {
                self.v[0xF] = u8::from(((self.v[vx] as u16) + (self.v[vy] as u16)) < 255);
                self.v[vx] = self.v[vx].wrapping_add(self.v[vy])
            }
            ISA::SUB { vx, vy } => {
                self.v[0xF] = u8::from(self.v[vx] > self.v[vy]);
                self.v[vx] = self.v[vx].wrapping_sub(self.v[vy])
            }
            ISA::SHR { vx: _, vy: _ } => {
                unimplemented!()
            }
            ISA::SUBN { vx, vy } => {
                self.v[0xF] = u8::from(self.v[vy] > self.v[vx]);
                self.v[vx] = self.v[vy].wrapping_sub(self.v[vx])
            }
            ISA::SHL { .. } => {
                unimplemented!()
            }
            ISA::SNE { vx, vy } => {
                if self.v[vx] != self.v[vy] {
                    self.pc += 2
                }
            }
            ISA::LDA { addr } => self.i = addr,
            ISA::JPO { addr } => self.pc = addr + self.v[0] as usize,
            ISA::RND { vx, imm } => self.v[vx] = self.rng.gen::<u8>() & imm,
            ISA::DRW { .. } => {}
            ISA::SKP { .. } => {}
            ISA::SKNP { .. } => {}
            ISA::LDDT { .. } => {}
            ISA::LDKEY { .. } => {}
            ISA::SETDT { .. } => {}
            ISA::LDST { .. } => {}
            ISA::ADDIR { .. } => {}
            ISA::LDSPR { .. } => {}
            ISA::LDBCD { .. } => {}
            ISA::STR { .. } => {}
            ISA::LDR { .. } => {}
            _ => {}
        };
        self.inc_pc()
    }
}
