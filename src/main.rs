mod cpu;
mod isa;

use crate::cpu::Program;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Bytes};
use std::path::Path;

fn main() {
    let file = {
        let args = env::args().nth(1);
        let filename = args.expect("Usage: chip8 filename");
        File::open(filename).expect("Could not find file.")
    };
    Program::from(file).run_program()
}
