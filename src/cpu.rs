use std::fmt::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use rand::{thread_rng, Rng};

use crate::isa::{Instruction, ISA};
use rand::rngs::ThreadRng;
use std::borrow::{Borrow, BorrowMut};

const MEM_SIZE: usize = 0xFFF;
const PROG_START: usize = 0x200;

pub struct Memory([u8; MEM_SIZE]);

impl Memory {
    pub fn get_instruction(&self, addr: usize) -> Instruction {
        Instruction::from([self.0[addr], self.0[addr + 1]])
    }
    pub fn get_u16(&self, addr: usize) -> u16 {
        u16::from_be_bytes([self.0[addr], self.0[addr + 1]])
    }
    pub fn set_u16(&mut self, addr: usize, val: u16) {
        for (offset, byte) in val.to_be_bytes().iter().enumerate() {
            self.0[addr + offset] = *byte;
        }
    }
}

pub struct Program {
    v: [i8; 0xF],
    dt: u8,
    st: u8,
    pc: u16,
    stk: usize,
    i: u16,
    rng: ThreadRng,
    mem: Memory,
}

impl Program {
    pub fn from(mut file: File) -> Self {
        let mut program = Self {
            v: Default::default(),
            dt: 0,
            st: 0,
            pc: PROG_START as u16,
            stk: 0x0,
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

    fn exec(&mut self, instruction: Instruction) {
        let inc: bool = true;
        match instruction.0 {
            ISA::SYS { addr } => { self.pc = addr as u16 },
            ISA::CLS => {}
            ISA::RET => {
                self.stk -= 2;
                self.pc = self.mem.get_u16(self.stk);
            }
            ISA::JP { addr } => { self.pc = addr as u16 },
            ISA::CALL { addr } => {
                self.stk += 2;
                self.mem.set_u16(self.stk, self.pc);
                self.pc = addr as u16;
            }
            ISA::SEI { vx, imm } => {
                if self.v[vx] == imm as i8 {
                    self.pc += 2;
                }
            }
            ISA::SNEI { vx, imm } => {
                if self.v[vx] != imm as i8 {
                    self.pc += 2;
                }
            }
            ISA::SE { vx, vy } => {
                if self.v[vx] == self.v[vy] {
                    self.pc += 2;
                }
            }
            ISA::LDI { vx, imm } => { self.v[vx] = imm as i8 }
            ISA::ADDI { vx, imm } => { self.v[vx] += imm }
            ISA::LD { vx, vy } => { self.v[vx] = self.v[vy] }
            ISA::OR { vx, vy } => { self.v[vx] |= self.v[vy] }
            ISA::AND { vx, vy } => { self.v[vx] &= self.v[vy] }
            ISA::XOR { vx, vy } => { self.v[vx] ^= self.v[vy] }
            ISA::ADD { vx, vy } => {
                self.v[vx] += self.v[vy];
                self.v[0xF] = i8::from(self.v[vx] as u8 > 255);
            }
            ISA::SUB { vx, vy } => {
                self.v[0xF] = i8::from(self.v[vx] > self.v[vy]);
                self.v[vx] -= self.v[vy];
            }
            ISA::SHR { vx, vy } => { unimplemented!() }
            ISA::SUBN { vx, vy } => {
                self.v[0xF] = i8::from(self.v[vx] < self.v[vy]);
                self.v[vx] = self.v[vy] - self.v[vx];
            }
            ISA::SHL { .. } => { unimplemented!() }
            ISA::SNE { vx, vy } => {
                if self.v[vx] != self.v[vy] {
                    self.pc += 2;
                }
            }
            ISA::LDA { addr } => { self.i = addr as u16; }
            ISA::JPO { addr } => { self.pc = (addr + self.v[0] as usize) as u16 }
            ISA::RND { vx, imm } => {
                self.v[vx] = self.rng.borrow_mut().gen_range((..255)) & imm;
            }
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
        };
    }
}