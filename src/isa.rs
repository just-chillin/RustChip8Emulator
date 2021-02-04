use std::fmt::{Debug, Formatter};
use std::fmt;

type Immediate = u8;

#[derive(Eq, PartialEq)]
pub struct Register(pub usize);

#[derive(Eq, PartialEq)]
pub struct Address(pub usize);

impl Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "v{:x}", self.0)
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    SYS { addr: Address },
    CLS,
    RET,
    JP { addr: Address },
    CALL { addr: Address },
    SEI { vx: Register, imm: Immediate },
    SNEI { vx: Register, imm: Immediate },
    SE { vx: Register, vy: Register },
    LDI { vx: Register, imm: Immediate },
    ADDI { vx: Register, imm: Immediate },
    LD { vx: Register, vy: Register },
    OR { vx: Register, vy: Register },
    AND { vx: Register, vy: Register },
    XOR { vx: Register, vy: Register },
    ADD { vx: Register, vy: Register },
    SUB { vx: Register, vy: Register },
    SHR { vx: Register, vy: Register },
    SUBN { vx: Register, vy: Register },
    SHL { vx: Register, vy: Register },
    SNE { vx: Register, vy: Register },
    LDA { addr: Address },
    JPO { addr: Address },
    RND { vx: Register, imm: Immediate },
    DRW { vx: Register, vy: Register, size: usize },
    SKP { vx: Register },
    SKNP { vx: Register },
    LDDT { vx: Register },
    LDKEY { vx: Register },
    SETDT { vx: Register },
    LDST { vx: Register },
    ADDIR { vx: Register },
    LDSPR { vx: Register },
    LDBCD { vx: Register },
    STR { vx: Register },
    LDR { vx: Register },
}

impl Instruction {
    pub fn try_from([b1, b2]: [u8; 2]) -> Result<Instruction, String> {
        let raw = (b1 as u16) << 8 | (b2 as u16);
        if raw == 0x00E0 {
            return Ok(Instruction::CLS);
        } else if raw == 0x00EE {
            return Ok(Instruction::RET);
        }
        let (addr, vx, vy, imm) = (addr(raw), vx(b1), vy(b2), imm(b2));
        let err = Err(format!("Invalid Instruction: {:X}", raw));
        Ok(match opcode(b1) {
            0x0 => Instruction::SYS { addr },
            0x1 => Instruction::JP { addr },
            0x2 => Instruction::CALL { addr },
            0x3 => Instruction::SEI { vx, imm },
            0x4 => Instruction::SNEI { vx, imm },
            0x5 => Instruction::SE { vx, vy },
            0x6 => Instruction::LDI { vx, imm },
            0x7 => Instruction::ADDI { vx, imm },
            0x8 => match variant(b2) {
                0x0 => Instruction::LD { vx, vy },
                0x1 => Instruction::OR { vx, vy },
                0x2 => Instruction::AND { vx, vy },
                0x3 => Instruction::XOR { vx, vy },
                0x4 => Instruction::ADD { vx, vy },
                0x5 => Instruction::SUB { vx, vy },
                0x6 => Instruction::SHR { vx, vy },
                0x7 => Instruction::SUBN { vx, vy },
                0xE => Instruction::SHL { vx, vy },
                _ => return err,
            },
            0x9 => Instruction::SNE { vx, vy },
            0xA => Instruction::LDA { addr },
            0xB => Instruction::JPO { addr },
            0xC => Instruction::RND { vx, imm },
            0xD => Instruction::DRW {
                vx,
                vy,
                size: u8::from_be(0x0F & b2) as usize,
            },
            0xE => match b2 {
                0x9E => Instruction::SKP { vx },
                0xA1 => Instruction::SKNP { vx },
                _ => return err,
            },
            0xF => match b2 {
                0x07 => Instruction::LDDT { vx },
                0x0A => Instruction::LDKEY { vx },
                0x15 => Instruction::SETDT { vx },
                0x18 => Instruction::LDST { vx },
                0x1E => Instruction::ADDIR { vx },
                0x29 => Instruction::LDSPR { vx },
                0x33 => Instruction::LDBCD { vx },
                0x55 => Instruction::STR { vx },
                0x65 => Instruction::LDR { vx },
                _ => return err,
            },
            _ => return err,
        })
    }
}

const fn variant(b2: u8) -> u8 {
    u8::from_be(0x0F & b2)
}

const fn vx(b1: u8) -> Register {
    Register(u8::from_be(0x0F & b1) as usize)
}

const fn vy(b2: u8) -> Register {
    Register(u8::from_be((0xF0 & b2) >> 4) as usize)
}

const fn imm(b2: u8) -> Immediate {
    u8::from_be(b2)
}

const fn addr(raw: u16) -> Address {
    Address(0x0FFF & raw as usize)
}

const fn opcode(b1: u8) -> u8 {
    (b1 & 0xF0) >> 4
}