use crate::chips::decode::{Instruction, Operation};
use crate::chips::dff::DFF;
use crate::chips::pc::PC;
use crate::chips::register::Register;
use crate::chips::register_file::RegFile;
use crate::chips::{mux2, wire, Chip, Wire, FOUR, MAX, ONE, U32, ZERO};
use std::io;
use std::io::Read;
use std::num::Wrapping;
use std::process::exit;

pub struct Execute<T = U32> {
    pub input: Wire<Instruction<T>>,
    pub output: Wire<IOCode<T>>,
    pub reg_file: Wire<RegFile<T>>,
    pc: Wire<PC<T>>,
    out: DFF<IOCode<T>>, // stores the value before outputting it
    rd: T,               // this is the affected register value is stored to target it at clk
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
        // connect the DFF output to Execute output interface
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
    // #[rustfmt::skip]
    fn compute(&mut self) {
        // start by resetting the output otherwise it will stick loading or storing
        *self.out.input.borrow_mut() = IOCode::default();

        let instruction = self.input.borrow().clone();
        println!("decoded instruction: {:?}", instruction);

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

        let imm = mux2(instruction.imm, rs2, rs2 != ZERO);
        let shamt = mux2(instruction.shamtw, rs2, rs2 != ZERO);

        use crate::chips::decode::Operation::*;

        let pc_addr = self.pc.borrow().output.borrow().clone();
        let target_addr = pc_addr + instruction.imm;

        // Set the load bits accordingly
        match instruction.op {
            JAL | JALR => {
                *rd.load.borrow_mut() = true;
                *self.pc.borrow_mut().load.borrow_mut() = true
            }
            AUIPC | BEQ | BNE | BLT | BGE | BLTU | BGEU => {
                *rd.load.borrow_mut() = false;
                *self.pc.borrow_mut().load.borrow_mut() = true
            }
            LUI | ADDI | SLTI | SLTIU | XORI | ORI | ANDI | SLLI | SRLI | SRAI | ADD | SUB
            | SLL | SLT | SLTU | XOR | SRL | SRA | OR | AND | MUL | MULH | MULHSU | MULHU | DIV
            | DIVU | REM | REMU => *rd.load.borrow_mut() = true,
            _ => {
                *rd.load.borrow_mut() = false;
                *self.pc.borrow_mut().load.borrow_mut() = false
            }
        }

        // ARITHMETIC and LOGIC INSTRUCTIONS
        *rd.input.borrow_mut() = match instruction.op {
            ADDI | ADD => rs1 + imm,
            SLTI | SLT => mux2(ZERO, ONE, (rs1.0 as i32) < (imm.0 as i32)),
            SLTIU | SLTU => mux2(ZERO, ONE, rs1 < imm),
            XORI | XOR => rs1 ^ imm,
            ORI | OR => rs1 | imm,
            ANDI | AND => rs1 & imm,
            SLLI | SLL => rs1 << shamt.0 as usize,
            SRLI | SRL => rs1 >> shamt.0 as usize,
            SRAI | SRA => Wrapping(((rs1.0 as i32) >> shamt.0) as u32),
            SUB => rs1 - rs2,
            MUL => rs1 * rs2,
            MULH => Wrapping(((((rs1.0 as i32) as i64) * ((rs2.0 as i32) as i64)) >> 32) as u32),
            MULHSU => Wrapping(((((rs1.0 as i32) as i64) * (rs2.0 as i64)) >> 32) as u32),
            MULHU => Wrapping((((rs1.0 as i64) * (rs2.0 as i64)) >> 32) as u32),
            DIV => Wrapping((rs1.0 as i32).checked_div(rs2.0 as i32).unwrap_or(i32::MAX) as u32),
            DIVU => Wrapping(rs1.0.checked_div(rs2.0).unwrap_or(u32::MAX)),
            REM => Wrapping(
                (rs1.0 as i32)
                    .checked_rem(rs2.0 as i32)
                    .unwrap_or(rs1.0 as i32) as u32,
            ),
            REMU => Wrapping(rs1.0.checked_rem(rs2.0).unwrap_or(rs1.0)),
            _ => ZERO,
        };

        // BRANCH INSTRUCTIONS
        let final_addr = match instruction.op {
            BEQ => mux2(pc_addr, target_addr, rs1 == rs2),
            BNE => mux2(pc_addr, target_addr, rs1 != rs2),
            BLT => mux2(pc_addr, target_addr, (rs1.0 as i32) < (rs2.0 as i32)),
            BGE => mux2(pc_addr, target_addr, (rs1.0 as i32) >= (rs2.0 as i32)),
            BLTU => mux2(pc_addr, target_addr, rs1 < rs2),
            BGEU => mux2(pc_addr, target_addr, rs1 >= rs2),
            _ => ZERO,
        };
        if final_addr % FOUR != ZERO {
            panic!("address-misaligned")
        }
        *self.pc.borrow_mut().input.borrow_mut() = final_addr;

        match instruction.op {
            LUI => *rd.input.borrow_mut() = imm,
            AUIPC => {
                *self.pc.borrow_mut().input.borrow_mut() =
                    self.pc.borrow().output.borrow().clone() + imm
            }
            JAL => {
                *rd.input.borrow_mut() = self.pc.borrow().output.borrow().clone() + FOUR;
                *self.pc.borrow_mut().input.borrow_mut() =
                    self.pc.borrow().output.borrow().clone() + imm;
            }
            JALR => {
                *rd.input.borrow_mut() = self.pc.borrow().output.borrow().clone() + FOUR;
                *self.pc.borrow_mut().input.borrow_mut() = (rs1 + imm >> 1) << 1;
            }
            LB | LH | LW | LBU | LHU => {
                *self.out.input.borrow_mut() = IOCode {
                    register: self.rd,
                    address: rs1 + imm,
                    store: false,
                }
            }
            SB | SH | SW => {
                *self.out.input.borrow_mut() = IOCode {
                    register: instruction.rd,
                    address: rs1 + imm,
                    store: true,
                }
            }
            _ => {}
        }

        rd.compute();
        // drop(rd);

        match instruction.op {
            ECALL => {
                self.rd = Wrapping(10);
                let a7 = self.reg_file.borrow_mut().get(17).output.borrow().clone();
                let a0 = reg_file.get(10);
                ecall(a7, a0);
            }
            EBREAK => {
                println!("ebreak not implemented yet")
            }
            _ => {}
        }
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

fn ecall(a7: U32, a0: &mut Register<U32>) {
    match a7.0 {
        1 => {
            let val = a0.output.borrow().clone();
            print!("{}", String::from_utf8_lossy(&[val.0 as u8]))
        }
        2 => {
            let mut buf = [0; 1];
            io::stdin().read_exact(&mut buf).unwrap();
            *a0.input.borrow_mut() = Wrapping(buf[0] as u32);
            *a0.load.borrow_mut() = true;
        }
        9 => {
            let val = a0.output.borrow().clone();
            exit(val.0 as i32)
        }
        _ => {}
    }
}
