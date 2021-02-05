use rand::prelude::*;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::iter;

use crate::isa::Instruction;
use std::borrow::{Borrow, BorrowMut};
use std::{fmt, panic};

const PROG_START: usize = 0x200;
// const BUILT_IN_SPRITES: [[u8; 5]; 16] = [
//     [0xf0, 0x90, 0x90, 0x90, 0xf0], //0
//     [0x20, 0x60, 0x20, 0x20, 0x70], //1
//     [0xf0, 0x10, 0xf0, 0x80, 0xf0], //2
//     [0xf0, 0x10, 0xf0, 0x10, 0xf0], //3
//     [0x90, 0x90, 0xf0, 0x10, 0x10], //4
//     [0xf0, 0x80, 0xf0, 0x10, 0xf0], //5
//     [0xf0, 0x80, 0xf0, 0x90, 0xf0], //6
//     [0xf0, 0x10, 0x20, 0x40, 0x40], //7
//     [0xf0, 0x90, 0xf0, 0x90, 0xf0], //8
//     [0xf0, 0x90, 0xf0, 0x10, 0xf0], //9
//     [0xf0, 0x90, 0xf0, 0x90, 0x90], //A
//     [0xe0, 0x90, 0xe0, 0x90, 0xe0], //B
//     [0xf0, 0x80, 0x80, 0x80, 0xf0], //c
//     [0xe0, 0x90, 0x90, 0x90, 0xe0], //d
//     [0xf0, 0x80, 0xf0, 0x80, 0xf0], //e
//     [0xf0, 0x80, 0xf0, 0x80, 0x80], //f
// ];
const BUILT_IN_SPRITES: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, //2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, //3
    0x90, 0x90, 0xf0, 0x10, 0x10, //4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, //5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, //6
    0xf0, 0x10, 0x20, 0x40, 0x40, //7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, //8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, //9
    0xf0, 0x90, 0xf0, 0x90, 0x90, //A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, //B
    0xf0, 0x80, 0x80, 0x80, 0xf0, //c
    0xe0, 0x90, 0x90, 0x90, 0xe0, //d
    0xf0, 0x80, 0xf0, 0x80, 0xf0, //e
    0xf0, 0x80, 0xf0, 0x80, 0x80, //f
];

pub struct Memory(Vec<u8>);

impl Memory {
    pub fn get_instruction(&self, addr: usize) -> Result<Instruction, String> {
        Instruction::try_from([self.0[addr], self.0[addr + 1]])
    }
}

pub struct Program {
    framebuffer: [[bool; 64]; 32],
    v: [u8; 16],
    dt: u8,
    st: u8,
    pc: usize,
    i: u16,
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
    pub fn from(mut raw: Vec<u8>) -> Self {
        let mem = {
            let mut preamble = vec![0u8; PROG_START];
            preamble.append(&mut raw);
            for (i, byte) in BUILT_IN_SPRITES.iter().enumerate() {
                preamble[i] = *byte;
            }
            Memory(preamble)
        };
        Self {
            framebuffer: [[false; 64]; 32],
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

    pub fn run(&mut self) {
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
            Instruction::SHR { vx, .. } => {
                self.v[0xf] = u8::from(self.v[vx.0] % 2 != 0);
                self.v[vx.0] /= 2;
            }
            Instruction::SUBN { vx, vy } => {
                self.v[0xF] = u8::from(self.v[vy.0] > self.v[vx.0]);
                self.v[vx.0] = self.v[vy.0].wrapping_sub(self.v[vx.0])
            }
            Instruction::SHL { vx, .. } => {
                self.v[0xf] = u8::from(self.v[vx.0] >= 128); //if most significant bit is set
                self.v[vx.0] *= 2;
            }
            Instruction::SNE { vx, vy } => {
                if self.v[vx.0] != self.v[vy.0] {
                    self.pc += 2
                }
            }
            Instruction::LDA { addr } => self.i = addr.0 as u16,
            Instruction::JPO { addr } => self.pc = addr.0 + self.v[0] as usize,
            Instruction::RND { vx, imm } => self.v[vx.0] = self.rng.gen::<u8>() & imm,
            Instruction::DRW { vx, vy, size } => self.draw_sprite(
                self.v[vx.0 as usize] as usize,
                self.v[vy.0 as usize] as usize,
                self.i as usize,
                size,
            ),
            Instruction::SKP { .. } => {
                let is_pressed = true;
                if is_pressed {
                    self.pc += 2;
                }
            }
            Instruction::SKNP { .. } => {
                let is_pressed = true;
                if !is_pressed {
                    self.pc += 2;
                }
            }
            Instruction::LDDT { vx } => self.v[vx.0] = self.dt,
            Instruction::LDKEY { .. } => {
                println!("Waiting for key press!");
                loop {}
            }
            Instruction::SETDT { vx } => self.dt = self.v[vx.0],
            Instruction::LDST { vx } => self.st = self.v[vx.0],
            Instruction::ADDIR { vx } => self.i += self.v[vx.0] as u16,
            Instruction::LDSPR { vx } => self.i = 5 * self.v[vx.0] as u16,
            Instruction::LDBCD { .. } => todo!(),
            Instruction::STR { vx } => {
                for (regvec, regval) in self.v[0..vx.0 + 1].iter().enumerate() {
                    self.mem.0[self.i as usize + regvec] = *regval;
                }
            }
            Instruction::LDR { vx } => {
                for (regvec, regval) in self.v[0..vx.0 + 1].iter_mut().enumerate() {
                    *regval = self.mem.0[self.i as usize + regvec];
                }
            }
        };
        self.inc_pc()
    }
    fn draw_sprite(&mut self, x: usize, y: usize, sprite_addr: usize, sprite_size: usize) {
        let sprite = &self.mem.0[sprite_addr..sprite_addr + sprite_size];
        let mask: u8 = 0b1000_0000;
        for (y_offset, byte) in sprite.iter().enumerate() {
            let y_cur = y + y_offset;
            if y_cur >= self.framebuffer.len() {
                break;
            }
            for x_offset in 0..8 {
                let x_cur = x + x_offset;
                if x_cur >= self.framebuffer[y_cur].len() {
                    break;
                }
                self.framebuffer[y_cur][x_cur] ^= (byte & (mask >> x_offset)) != 0;
            }
        }
    }
}
