use crate::chips::dff::DFF;
use crate::chips::{mux2, wire, Chip, Wire, ONE, U32, ZERO};
use std::num::Wrapping;

pub struct Decode<T = U32> {
    pub input: Wire<T>,
    pub output: Wire<Instruction<T>>,
    // out stores the result of the current operation and transfers it to output at clk
    out: DFF<Instruction<T>>,
}

impl Decode {
    pub fn new(input: Wire<U32>, output: Wire<Instruction>) -> Self {
        Self {
            input,
            output: output.clone(),
            // new dff with wire connected to Decode's output
            out: DFF::new(wire(Instruction::default()), output),
        }
    }
}

fn bit_range(v: U32, msb: usize, lsb: usize) -> U32 {
    let mask = (ONE << (msb - lsb + 1)) - ONE;
    (v >> lsb) & mask
}

impl Decode {
    fn decode(&self, inst: U32) -> Instruction {
        let neg = (inst >> 31) == ONE;
        let ones_21 = Wrapping((0xFFFFF8 << 8) as u32);
        let ones_20 = Wrapping((0xFFFFF << 12) as u32);
        let ones_12 = Wrapping((0xFFF << 20) as u32);

        let imm_30_25 = bit_range(inst, 30, 25);
        let imm_24_21 = bit_range(inst, 24, 21);
        let imm_20 = bit_range(inst, 20, 20);
        let imm_11_8 = bit_range(inst, 11, 8);
        let imm_7 = bit_range(inst, 7, 7);
        let imm_30_20 = bit_range(inst, 30, 20);
        let imm_19_12 = bit_range(inst, 19, 12);

        let imm_i = imm_30_25 << 5 | imm_24_21 << 1 | imm_20;
        let imm_s = imm_30_25 << 5 | imm_11_8 << 1 | imm_7;
        let imm_b = imm_7 << 11 | imm_30_25 << 5 | imm_11_8 << 1;
        let imm_u = imm_30_20 << 20 | imm_19_12 << 12;
        let imm_j = imm_19_12 << 12 | imm_20 << 11 | imm_30_25 << 5 | imm_24_21 << 1;
        let imm_i = mux2(imm_i, imm_i | ones_21, neg);
        let imm_s = mux2(imm_s, imm_s | ones_21, neg);
        let imm_b = mux2(imm_b, imm_b | ones_20, neg);
        let imm_u = mux2(imm_u, imm_u | (ONE << 31), neg);
        let imm_j = mux2(imm_j, imm_j | ones_12, neg);

        let shamtw = bit_range(inst, 24, 20);
        let funct3 = bit_range(inst, 14, 12);
        let funct7 = bit_range(inst, 31, 25);
        let rd = bit_range(inst, 11, 7);
        let rs1 = bit_range(inst, 19, 15);
        let rs2 = bit_range(inst, 24, 20);
        let opcode = bit_range(inst, 6, 0);
        let imm;

        use Operation::*;
        let op = match opcode.0 {
            0b0110011 => {
                imm = ZERO;
                // OP
                if funct7 == ZERO {
                    match funct3.0 {
                        0b000 => ADD,
                        0b001 => SLL,
                        0b010 => SLT,
                        0b011 => SLTU,
                        0b100 => XOR,
                        0b101 => SRL,
                        0b110 => OR,
                        0b111 => AND,
                        _ => panic!("invalid funct3 {funct3} for op"),
                    }
                } else if funct7.0 == 0b0100000 {
                    match funct3.0 {
                        0b000 => SUB,
                        0b101 => SRA,
                        _ => panic!("invalid funct3 {funct3} for op"),
                    }
                } else if funct7.0 == 0b0000001 {
                    match funct3.0 {
                        0b000 => MUL,
                        0b001 => MULH,
                        0b010 => MULHSU,
                        0b011 => MULHU,
                        0b100 => DIV,
                        0b101 => DIVU,
                        0b110 => REM,
                        0b111 => REMU,
                        _ => panic!("invalid funct3 {funct3} for op"),
                    }
                } else {
                    panic!("not recognized {funct7}")
                }
            }
            0b0010011 => {
                // OP-IMM
                imm = imm_i;
                match funct3.0 {
                    0b000 => ADDI,
                    0b001 => SLLI,
                    0b010 => SLTI,
                    0b011 => SLTIU,
                    0b100 => XORI,
                    0b101 => {
                        if funct7 == ZERO {
                            SRLI
                        } else if funct7.0 == 0b0100000 {
                            SRAI
                        } else {
                            panic!("not recognized {funct7}")
                        }
                    }
                    0b110 => ORI,
                    0b111 => ANDI,
                    _ => panic!("invalid funct3 {funct3} for op-imm"),
                }
            }
            0b0110111 => {
                imm = imm_u;
                LUI
            }
            0b0010111 => {
                imm = imm_u;
                AUIPC
            }
            0b1101111 => {
                imm = imm_j;
                JAL
            }
            0b1100111 => {
                imm = imm_i;
                JALR
            }
            0b1100011 => {
                // BRANCH
                imm = imm_b;
                match funct3.0 {
                    0b000 => BEQ,
                    0b001 => BNE,
                    0b100 => BLT,
                    0b101 => BGE,
                    0b110 => BLTU,
                    0b111 => BGEU,
                    _ => panic!("invalid instruction"),
                }
            }
            0b0000011 => {
                // LOAD
                imm = imm_i;
                match funct3.0 {
                    0b000 => LB,
                    0b001 => LH,
                    0b010 => LW,
                    0b100 => LBU,
                    0b101 => LHU,
                    _ => LW,
                }
            }
            0b0100011 => {
                // STORE
                imm = imm_s;
                match funct3.0 {
                    0b000 => SB,
                    0b001 => SH,
                    0b010 => SW,
                    _ => SW,
                }
            }
            0b1110011 => {
                // SYSTEM
                imm = ZERO;
                match funct7.0 {
                    0 => ECALL,
                    1 => EBREAK,
                    _ => panic!("invalid instruction"),
                }
            }
            _ => {
                imm = ZERO;
                ADDI
            }
        };

        Instruction {
            rd,
            rs1,
            rs2,
            imm,
            shamtw,
            op,
        }
    }
}

impl Chip for Decode {
    fn compute(&mut self) {
        let inst = self.input.borrow().clone();
        println!("fetch inst: {inst:032b}");

        *self.out.input.borrow_mut() = self.decode(inst);
        self.out.compute(); // compute karna na bhule
    }

    fn clk(&mut self) {
        // show the effect to the world
        self.out.clk();
    }
}

#[derive(Default, Clone, Debug)]
pub struct Instruction<T = U32> {
    pub rd: T,  // "rd", 11:7
    pub rs1: T, // "rs1", 19:15
    pub rs2: T, // "rs2", 24:20
    pub imm: T,
    pub shamtw: T, // "shamtw", 24:20
    pub op: Operation,
}

#[derive(Clone, Debug)]
pub enum Operation {
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
    ECALL,
    EBREAK,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::ADDI
    }
}
