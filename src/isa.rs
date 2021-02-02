use ux::{u12, u4};
use crate::isa::ISA::{SYS, JP};

enum ISA {
    SYS { addr: usize },
    CLS,
    RET,
    JP { addr: usize },
    CALL { addr: usize },
    SEI { vx: usize, imm: u8 },
    SNEI { vx: usize, imm: u8 },
    SE { vx: usize, vy: usize },
    LDI { vx: usize, imm: u8 },
    ADDI { vx: usize, imm: u8 },
    LD { vx: usize, vy: usize },
    OR { vx: usize, vy: usize },
    AND { vx: usize, vy: usize },
    XOR { vx: usize, vy: usize },
    ADD { vx: usize, vy: usize },
    SUB { vx: usize, vy: usize },
    SHR { vx: usize, vy: usize },
    SUBN { vx: usize, vy: usize },
    SHL { vx: usize, vy: usize },
    SNE { vx: usize, vy: usize },
    LDA { addr: usize },
    JPO { addr: usize },
    RND { vx: usize, imm: u8 },
    DRW { vx: usize, vy: usize, size: usize },
    SKP { vx: usize },
    SKNP { vx: usize },
    LDDT { vx: usize },
    LDST { vx: usize },
    ADDIR { vx: usize },
    LDSPR { vx: usize },
    STR { vx: usize },
    LDR { vx: usize },
}

pub struct Instruction(ISA);

impl Instruction {
    pub fn from([high, low]: [u8; 2]) -> Self {
        let raw = (high as u16) << 8 | (low as u16);
        if raw == 0x00E0 {
            return Instruction(ISA::CLS);
        } else if raw == 0x00EE {
            return Instruction(ISA::RET);
        }

        let opcode = high & 0b11110000;
        return Instruction(match (opcode, low) {
            (0x0, _) => ISA::SYS { addr: addr(raw) },
            (0x1, _) => ISA::JP { addr: addr(raw) },
            (0x2, _) => ISA::CALL {addr: addr(raw) }
            _ => panic!("help pls")
        })
    }
}
const fn vx(high: u8) -> usize {
    let x = (high & 0xF0FF) << 4;
    u8::from_be(x) as usize
}
const fn vy(low: u8) -> usize {
    let y = (low & 0x0F);
    u8::from_be(y) as usize
}
const fn imm(low: u8) -> u8 {
    u8::from_be(low)
}
const fn addr(raw: u16) -> usize {
    let addr = (raw & 0xF000) << 4;
    u16::from_be(addr) as usize
}