use std::error::Error;
use std::fmt;

type Immediate = u8;
type Register = usize;
type Address = usize;

#[derive(Eq, PartialEq)]
pub enum ISA {
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

pub struct Instruction(pub ISA);
impl Instruction {
    pub fn try_from([b1, b2]: [u8; 2]) -> Instruction {
        let raw = (b1 as u16) << 8 | (b2 as u16);
        if raw == 0x00E0 {
            return Instruction(ISA::CLS);
        } else if raw == 0x00EE {
            return Instruction(ISA::RET);
        }
        let opcode = b1 & 0xF0;
        let (addr, vx, vy, imm) = (addr(raw), vx(b1), vy(b2), imm(b1));
        Instruction(match opcode {
            0x0 => ISA::SYS { addr },
            0x1 => ISA::JP { addr },
            0x2 => ISA::CALL { addr },
            0x3 => ISA::SEI { vx, imm },
            0x4 => ISA::SNEI { vx, imm },
            0x5 => ISA::SE { vx, vy },
            0x6 => ISA::LDI { vx, imm },
            0x7 => ISA::ADDI { vx, imm },
            0x8 => match variant(b2) {
                0x0 => ISA::LD { vx, vy },
                0x1 => ISA::OR { vx, vy },
                0x2 => ISA::AND { vx, vy },
                0x3 => ISA::XOR { vx, vy },
                0x4 => ISA::ADD { vx, vy },
                0x5 => ISA::SUB { vx, vy },
                0x6 => ISA::SHR { vx, vy },
                0x7 => ISA::SUBN { vx, vy },
                0xE => ISA::SHL { vx, vy },
                _ => ISA::INVALID,
            },
            0x9 => ISA::SNE { vx, vy },
            0xA => ISA::LDA { addr },
            0xB => ISA::JPO { addr },
            0xC => ISA::RND { vx, imm },
            0xD => ISA::DRW {
                vx,
                vy,
                size: u8::from_be(b2) as usize,
            },
            0xE => match b2 {
                0x9E => ISA::SKP { vx },
                0xA1 => ISA::SKNP { vx },
                _ => ISA::INVALID,
            },
            0xF => match b2 {
                0x07 => ISA::LDDT { vx },
                0x0A => ISA::LDKEY { vx },
                0x15 => ISA::SETDT { vx },
                0x18 => ISA::LDST { vx },
                0x1E => ISA::ADDIR { vx },
                0x29 => ISA::LDSPR { vx },
                0x33 => ISA::LDBCD { vx },
                0x55 => ISA::STR { vx },
                0x65 => ISA::LDR { vx },
                _ => ISA::INVALID,
            },
            _ => ISA::INVALID,
        })
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
