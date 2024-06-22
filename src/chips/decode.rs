use crate::chips::dff::DFF;
use crate::chips::{mux2, Chip, Wire};
use std::cell::RefCell;

pub struct Decode {
    pub input: Wire<u32>,
    pub output: Wire<Instruction>,
    out: DFF<Instruction>,
}

impl Decode {
    fn new(input: Wire<u32>, output: Wire<Instruction>) -> Self {
        Self {
            input,
            output: output.clone(),
            out: DFF::new(RefCell::new(Instruction::default()), output),
        }
    }
}

fn bit_range(v: u32, msb: usize, lsb: usize) -> u32 {
    let mask = (1u32 << (msb - lsb + 1)) - 1;
    (v >> lsb) & mask
}

impl Chip for Decode {
    fn compute(&mut self) {
        let inst = self.input.borrow().clone();
        let neg = (inst >> 31) == 1;

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

        *self.out.input.borrow_mut() = Instruction {
            rd: bit_range(inst, 11, 7),
            rs1: bit_range(inst, 19, 15),
            rs2: bit_range(inst, 24, 20),
            funct3: bit_range(inst, 14, 12),
            funct7: bit_range(inst, 31, 25),
            imm_i: mux2(imm_i, (!imm_i) + 1, neg),
            imm_s: mux2(imm_s, (!imm_s) + 1, neg),
            imm_b: mux2(imm_b, (!imm_b) + 1, neg),
            imm_u: mux2(imm_u, (!imm_u) + 1, neg),
            imm_j: mux2(imm_j, (!imm_j) + 1, neg),
            shamtw: bit_range(inst, 24, 20),
            opcode: bit_range(inst, 6, 0),
        };

        self.out.compute(); // compute karna na bhule
    }

    fn clk(&mut self) {
        self.out.clk();
    }
}

#[derive(Default, Clone)]
pub struct Instruction<T = u32> {
    pub rd: T,     // "rd", 11:7
    pub rs1: T,    // "rs1", 19:15
    pub rs2: T,    // "rs2", 24:20
    pub funct3: T, // "funct3", 14:12
    pub funct7: T, // "funct7", 31:25
    pub imm_i: T,  // "imm_i", 31...30:25,24:21,20
    pub imm_s: T,  // "imm_s", 31...30:25,11:8,7
    pub imm_b: T,  // "imm_b", 31...7,30:25,11:8
    pub imm_u: T,  // "imm_u", 31,30:20,19:12,0...
    pub imm_j: T,  // "imm_j",
    pub shamtw: T, // "shamtw", 24:20
    pub opcode: T, // "opcode", 6:0
}
