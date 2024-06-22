use crate::chips::decode::Instruction;
use crate::chips::pc::PC;
use crate::chips::register_file::RegFile;
use crate::chips::{Chip, Wire};

pub struct Execute {
    pub input: Wire<Instruction>,
    pub output: Wire<IOCode>,
    reg_file: Wire<RegFile<u32>>,
    pc: Wire<PC>,
}

#[derive(Default)]
pub struct IOCode {
    register: u32,
    address: u32,
    load: bool,
}

impl Execute {
    pub fn new(
        input: Wire<Instruction>,
        output: Wire<IOCode>,
        reg_file: Wire<RegFile<u32>>,
        pc: Wire<PC>,
    ) -> Self {
        Self {
            input,
            output,
            reg_file,
            pc,
        }
    }
}

impl Chip for Execute {
    fn compute(&mut self) {
        let instruction = self.input.borrow().clone();
        match instruction.opcode {
            OP => {
                
            }
            OP_IMM => {
                
            }
            BRANCH => {
                
            }
            LUI => {
                
            }
            AUIPC => {
                
            }
            JAL => {
                
            }
            JALR => {
                
            }
            LOAD => {
                
            }
            STORE => {
                
            }
            _ => {}
        }

    }

    fn clk(&mut self) {
        todo!()
    }
}

const OP: u32 = 0b0110011;
const OP_IMM: u32 = 0b0010011;
const LUI: u32 = 0b0110111;
const AUIPC: u32 = 0b0010111;
const JAL: u32 = 0b1101111;
const JALR: u32 = 0b1100111;
const BRANCH: u32 = 0b1100011;
const LOAD: u32 = 0b0000011;
const STORE: u32 = 0b0100011;
