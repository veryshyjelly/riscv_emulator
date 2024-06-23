use crate::chips::decode::Instruction;
use crate::chips::dff::DFF;
use crate::chips::pc::PC;
use crate::chips::register_file::RegFile;
use crate::chips::{mux2, wire, Chip, Wire, ONE, U32, ZERO};
use std::num::Wrapping;

pub struct Execute<T = U32> {
    input: Wire<Instruction<T>>,
    pub output: Wire<IOCode<T>>,
    reg_file: Wire<RegFile<T>>,
    pc: Wire<PC<T>>,
    out: DFF<IOCode<T>>, // stores the value before outputting it
    rd: U32,             // this is the affected register value is stored to target it at clk
}

#[derive(Default, Clone, Debug)]
pub struct IOCode<T = U32> {
    pub register: T,
    pub address: T,
    pub store: bool,
}

impl Execute {
    pub fn new(
        input: Wire<Instruction>,
        output: Wire<IOCode>,
        reg_file: Wire<RegFile<U32>>,
        pc: Wire<PC>,
    ) -> Self {
        Self {
            input,
            output: output.clone(),
            reg_file,
            pc,
            out: DFF::new(wire(IOCode::default()), output),
            rd: ZERO,
        }
    }
}

impl Chip for Execute {
    fn compute(&mut self) {
        // start by resetting the output otherwise it will stick loading or storing
        *self.out.input.borrow_mut() = IOCode::default();

        let instruction = self.input.borrow().clone();
        println!("instruction: {:?}", instruction);

        let mut reg_file = self.reg_file.borrow_mut();
        let rs1 = reg_file
            .get(instruction.rs1.0 as usize)
            .output
            .borrow()
            .clone();
        let rs2 = reg_file
            .get(instruction.rs2.0 as usize)
            .output
            .borrow()
            .clone();
        let rd = reg_file.get(instruction.rd.0 as usize);
        // store the value of rd for future use
        self.rd = instruction.rd;

        match instruction.opcode.0 {
            OP => {
                let result = alu(instruction.funct3, instruction.funct7, rs1, rs2, rs2)
                    .expect(&format!("invalid instruction: {instruction:?}"));
                *rd.input.borrow_mut() = result;
                *rd.load.borrow_mut() = true;
            }
            OP_IMM => {
                let result = alu(
                    instruction.funct3,
                    instruction.funct7,
                    rs1,
                    instruction.shamtw,
                    instruction.imm_i,
                )
                .expect(&format!("invalid instruction: {instruction:?}"));
                *rd.input.borrow_mut() = result;
                *rd.load.borrow_mut() = true;
            }
            LUI => {
                *rd.input.borrow_mut() = instruction.imm_u;
                *rd.load.borrow_mut() = true;
            }
            AUIPC => {
                *self.pc.borrow_mut().input.borrow_mut() =
                    self.pc.borrow().output.borrow().clone() + instruction.imm_u;
                *self.pc.borrow_mut().load.borrow_mut() = true;
            }
            JAL => {
                *rd.input.borrow_mut() = self.pc.borrow().output.borrow().clone() + Wrapping(4);
                *rd.load.borrow_mut() = true;
                *self.pc.borrow_mut().input.borrow_mut() =
                    self.pc.borrow().output.borrow().clone() + instruction.imm_j;
                *self.pc.borrow_mut().load.borrow_mut() = true;
            }
            JALR => {
                *rd.input.borrow_mut() = self.pc.borrow().output.borrow().clone() + Wrapping(4);
                *rd.load.borrow_mut() = true;
                *self.pc.borrow_mut().input.borrow_mut() = (rs1 + instruction.imm_i >> 1) << 1;
                *self.pc.borrow_mut().load.borrow_mut() = true;
            }
            BRANCH => {
                let pc_addr = self.pc.borrow().output.borrow().clone();
                let target_addr = pc_addr + instruction.imm_b;
                let final_addr = match instruction.funct3.0 {
                    0b000 => mux2(pc_addr, target_addr, rs1 == rs2), // BEQ
                    0b001 => mux2(pc_addr, target_addr, rs1 != rs2), // BNE
                    0b100 => mux2(pc_addr, target_addr, (rs1.0 as i32) < (rs2.0 as i32)), // BLT
                    0b101 => mux2(pc_addr, target_addr, (rs1.0 as i32) >= (rs2.0 as i32)), // BGE
                    0b110 => mux2(pc_addr, target_addr, rs1 < rs2),  // BLTU
                    0b111 => mux2(pc_addr, target_addr, rs1 >= rs2), // BGEU
                    _ => panic!("invalid instruction"),
                };
                if final_addr % Wrapping(4) != ZERO {
                    panic!("address-misaligned")
                }
                *self.pc.borrow_mut().input.borrow_mut() = final_addr;
            }
            LOAD => {
                *self.out.input.borrow_mut() = IOCode {
                    register: instruction.rd,
                    address: rs1 + instruction.imm_i,
                    store: false,
                }
            }
            STORE => {
                *self.out.input.borrow_mut() = IOCode {
                    register: instruction.rd,
                    address: rs1 + instruction.imm_s,
                    store: true,
                }
            }
            _ => {}
        }
        rd.compute();
    }

    fn clk(&mut self) {
        // Get the register that got updated
        let mut reg_file = self.reg_file.borrow_mut();
        let rd = reg_file.get(self.rd.0 as usize);

        // Clock the register and self output
        rd.clk();
        self.out.clk();
    }
}

fn alu(funct3: U32, funct7: U32, rs1: U32, shamt: U32, imm: U32) -> Option<U32> {
    let result = if funct7 == ZERO {
        match funct3.0 {
            0b000 => rs1 + imm,                                        // ADD
            0b001 => rs1 << shamt.0 as usize,                          // SLL
            0b010 => mux2(ZERO, ONE, (rs1.0 as i32) < (imm.0 as i32)), // SLT
            0b011 => mux2(ZERO, ONE, rs1 < imm),                       // SLTU
            0b100 => rs1 ^ imm,                                        // XOR
            0b101 => rs1 >> shamt.0 as usize,                          // SRL
            0b110 => rs1 | imm,                                        // OR
            0b111 => rs1 & imm,                                        // AND
            _ => None?,
        }
    } else {
        match funct3.0 {
            0b000 => rs1 - imm,                                    // SUB
            0b101 => Wrapping(((rs1.0 as i32) >> shamt.0) as u32), // SRA
            _ => None?,
        }
    };
    Some(result)
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
