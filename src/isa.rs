use crate::isa::ISA::{JP, SYS};

type Immediate = u8;
type Register = usize;
type Address = usize;

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
}

pub struct Instruction(pub ISA);

impl Instruction {
    pub fn from([high, low]: [u8; 2]) -> Self {
        let raw = (high as u16) << 8 | (low as u16);
        if raw == 0x00E0 {
            return Instruction(ISA::CLS);
        } else if raw == 0x00EE {
            return Instruction(ISA::RET);
        }

        let opcode = high & 0x0F;
        let (addr, vx, vy, imm) = (addr(raw), vx(high), vy(low), imm(high));
        return Instruction(match opcode {
            0x0 => ISA::SYS { addr },
            0x1 => ISA::JP { addr },
            0x2 => ISA::CALL { addr },
            0x3 => ISA::SEI { vx, imm },
            0x4 => ISA::SNEI { vx, imm },
            0x5 => ISA::SE { vx, vy },
            0x6 => ISA::LDI { vx, imm },
            0x7 => ISA::ADDI { vx, imm },
            0x8 => match variant(low) {
                0x0 => ISA::LD { vx, vy },
                0x1 => ISA::OR { vx, vy },
                0x2 => ISA::AND { vx, vy },
                0x3 => ISA::XOR { vx, vy },
                0x4 => ISA::ADD { vx, vy },
                0x5 => ISA::SUB { vx, vy },
                0x6 => ISA::SHR { vx, vy },
                0x7 => ISA::SUBN { vx, vy },
                0xE => ISA::SHL { vx, vy },
                _ => panic!("help pls"),
            },
            0x9 => ISA::SNE { vx, vy },
            0xA => ISA::LDA { addr },
            0xB => ISA::JPO { addr },
            0xC => ISA::RND { vx, imm },
            0xD => ISA::DRW {
                vx,
                vy,
                size: u8::from_be(low) as usize,
            },
            0xE => match low {
                0x9E => ISA::SKP { vx },
                0xA1 => ISA::SKNP { vx },
                _ => panic!("pls help!"),
            },
            0xF => match low {
                0x07 => ISA::LDDT { vx },
                0x0A => ISA::LDKEY { vx },
                0x15 => ISA::SETDT { vx },
                0x18 => ISA::LDST { vx },
                0x1E => ISA::ADDIR { vx },
                0x29 => ISA::LDSPR { vx },
                0x33 => ISA::LDBCD { vx },
                0x55 => ISA::STR { vx },
                0x65 => ISA::LDR { vx },
                _ => panic!("pls help!"),
            },

            _ => panic!("help pls"),
        });
    }
}

const fn variant(low: u8) -> u8 {
    u8::from_be((low & 0xF0) << 4)
}

const fn vx(high: u8) -> usize {
    let x = (high & 0xF0) << 4;
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
