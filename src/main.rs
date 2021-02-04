use std::env;
use std::fs::File;

use crate::cpu::Program;
use std::io::Read;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod cpu;
mod isa;

fn main() {

    let mut program = {
        let filename = env::args().nth(1).expect("chip8: missing file operand");
        let mut file = File::open(filename.clone())
            .expect(&*format!("chip8: {}: No such file or directory", filename));
        let mut bytes: Vec<u8> = vec![];
        file.read_to_end(&mut bytes).expect("Failed to read file!");
        Program::from(bytes)
    };
    program.run();
}
