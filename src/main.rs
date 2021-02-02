mod isa;
mod cpu;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{Bytes, BufReader};
use crate::cpu::Chip8Program;

fn main() {
    let mut file = {
        let args = env::args().nth(1);
        let filename = args.expect("Usage: chip8 filename");
        File::open(filename).expect("Could not find file.")
    };
    Chip8Program::from(file).run_program()
}
