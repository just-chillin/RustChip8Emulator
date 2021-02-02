use std::error::Error;
use std::fmt;

type Immediate = u8;
type Register = usize;
type Address = usize;

#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    SYS {
        addr: Address,
    },
    CLS,
    RET,
    JP {
        addr: Address,
    },
    CALL {
        addr: Address,
    },
    SEI {
        vx: Register,
        imm: Immediate,
    },
    SNEI {
        vx: Register,
        imm: Immediate,
    },
    SE {
        vx: Register,
        vy: Register,
    },
    LDI {
        vx: Register,
        imm: Immediate,
    },
    ADDI {
        vx: Register,
        imm: Immediate,
    },
    LD {
        vx: Register,
        vy: Register,
    },
    OR {
        vx: Register,
        vy: Register,
    },
    AND {
        vx: Register,
        vy: Register,
    },
    XOR {
        vx: Register,
        vy: Register,
    },
    ADD {
        vx: Register,
        vy: Register,
    },
    SUB {
        vx: Register,
        vy: Register,
    },
    SHR {
        vx: Register,
        vy: Register,
    },
    SUBN {
        vx: Register,
        vy: Register,
    },
    SHL {
        vx: Register,
        vy: Register,
    },
    SNE {
        vx: Register,
        vy: Register,
    },
    LDA {
        addr: Address,
    },
    JPO {
        addr: Address,
    },
    RND {
        vx: Register,
        imm: Immediate,
    },
    DRW {
        vx: Register,
        vy: Register,
        size: usize,
    },
    SKP {
        vx: Register,
    },
    SKNP {
        vx: Register,
    },
    LDDT {
        vx: Register,
    },
    LDKEY {
        vx: Register,
    },
    SETDT {
        vx: Register,
    },
    LDST {
        vx: Register,
    },
    ADDIR {
        vx: Register,
    },
    LDSPR {
        vx: Register,
    },
    LDBCD {
        vx: Register,
    },
    STR {
        vx: Register,
    },
    LDR {
        vx: Register,
    },
    INVALID,
}

#[derive(Debug)]
struct InvalidInstructionError(pub String);
impl fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for InvalidInstructionError {}

impl Instruction {
    pub fn try_from([b1, b2]: [u8; 2]) -> Instruction {
        let raw = (b1 as u16) << 8 | (b2 as u16);
        if raw == 0x00E0 {
            return Instruction::CLS;
        } else if raw == 0x00EE {
            return Instruction::RET;
        }
        let opcode = b1 & 0xF0;
        let (addr, vx, vy, imm) = (addr(raw), vx(b1), vy(b2), imm(b1));
        match opcode {
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
                _ => Instruction::INVALID,
            },
            0x9 => Instruction::SNE { vx, vy },
            0xA => Instruction::LDA { addr },
            0xB => Instruction::JPO { addr },
            0xC => Instruction::RND { vx, imm },
            0xD => Instruction::DRW {
                vx,
                vy,
                size: u8::from_be(b2) as usize,
            },
            0xE => match b2 {
                0x9E => Instruction::SKP { vx },
                0xA1 => Instruction::SKNP { vx },
                _ => Instruction::INVALID,
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
                _ => Instruction::INVALID,
            },
            _ => Instruction::INVALID,
        }
    }
}

const fn variant(b2: u8) -> u8 {
    u8::from_be((b2 & 0xF0) << 4)
}
const fn vx(b1: u8) -> usize {
    u8::from_be((b1 & 0x0F) << 4) as usize
}
const fn vy(b2: u8) -> usize {
    u8::from_be(b2 & 0xF0) as usize
}
const fn imm(b2: u8) -> u8 {
    u8::from_be(b2)
}
const fn addr(raw: u16) -> usize {
    u16::from_be((raw & 0x0FFF) << 4) as usize
}
