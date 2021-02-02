use std::env;
use std::fs::File;

use crate::cpu::Program;

mod cpu;
mod isa;

fn main() {
    let file = {
        let args = env::args().nth(1);
        let filename = args.expect("Usage: chip8 filename");
        File::open(filename).expect("Could not find file.")
    };
    Program::from(file).run_program()
}
