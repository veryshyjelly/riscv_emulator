use crate::chips::decode::Instruction;
use crate::chips::pc::PC;
use crate::chips::register::Register;
use crate::chips::register_file::RegFile;
use crate::chips::rom::ROM;
use crate::chips::{mux2, Chip, Wire, FOUR, ONE, U32, ZERO};
use std::io;
use std::io::Read;
use std::num::Wrapping;
use std::process::exit;

use super::memory::Memory;

pub struct Execute<T = U32> {
    pub input: Wire<Instruction<T>>,
    pub reg_file: Wire<RegFile<T>>,
    pub memory: Memory<T>,
    rom: Wire<ROM<T>>,
    pc: Wire<PC<T>>,
    rd: T, // this is the affected register value is stored to target it at clk
    halt: usize,
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
        memory: Memory<U32>,
        rom: Wire<ROM>,
        reg_file: Wire<RegFile<U32>>,
        pc: Wire<PC>,
    ) -> Self {
        // connect the DFF output to Execute output interface
        Self {
            input,
            memory,
            rom,
            reg_file,
            pc,
            rd: ZERO,
            halt: 0,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        // store the value of rd for future use
        self.rd = instruction.rd;

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

        use crate::chips::decode::Operation::*;

        let imm = match instruction.op {
            ADD | SLT | SLTU | XOR | OR | AND | SLL | SRL | SRA => rs2,
            _ => instruction.imm,
        };
        let shamt = match instruction.op {
            ADD | SLT | SLTU | XOR | OR | AND | SLL | SRL | SRA => rs2,
            _ => instruction.shamtw,
        };

        let pc_addr = self.pc.borrow().output.borrow().clone();
        let target_addr = pc_addr + instruction.imm - FOUR - FOUR;

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
            LB | LH | LW | LBU | LHU | LUI | ADDI | SLTI | SLTIU | XORI | ORI | ANDI | SLLI
            | SRLI | SRAI | ADD | SUB | SLL | SLT | SLTU | XOR | SRL | SRA | OR | AND | MUL
            | MULH | MULHSU | MULHU | DIV | DIVU | REM | REMU => {
                *rd.load.borrow_mut() = true;
                *self.pc.borrow_mut().load.borrow_mut() = false
            }
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
            _ => pc_addr,
        };
        if final_addr % FOUR != ZERO {
            panic!("address-misaligned")
        }
        if pc_addr != final_addr {
            self.halt = 2;
        }
        *self.pc.borrow_mut().input.borrow_mut() = final_addr;

        let imm = instruction.imm;
        match instruction.op {
            LUI => *rd.input.borrow_mut() = imm,
            AUIPC => {
                *self.pc.borrow_mut().input.borrow_mut() = pc_addr + imm - FOUR - FOUR;
                self.halt = 2;
            }
            JAL => {
                *rd.input.borrow_mut() = pc_addr - FOUR;
                *self.pc.borrow_mut().input.borrow_mut() =
                    self.pc.borrow().output.borrow().clone() + instruction.imm - FOUR - FOUR;
                self.halt = 2;
            }
            JALR => {
                *rd.input.borrow_mut() = pc_addr - FOUR;
                *self.pc.borrow_mut().input.borrow_mut() = (rs1 + imm >> 1) << 1;
                self.halt = 2;
            }
            LB | LH | LW | LBU | LHU => {
                *self.memory.address.borrow_mut() = rs1 + imm;
                *self.memory.load.borrow_mut() = false;
                self.memory.compute();
                self.memory.clk();
                *rd.input.borrow_mut() = self.memory.output.borrow().clone();
            }
            SB | SH | SW => {
                *self.memory.address.borrow_mut() = rs1 + imm;
                *self.memory.load.borrow_mut() = true;
                *self.memory.input.borrow_mut() = rs2;
                self.memory.compute();
                self.memory.clk();
            }
            _ => {}
        }

        rd.compute();

        match instruction.op {
            ECALL => {
                let a7 = reg_file.get(17).output.borrow().clone();
                let a1 = reg_file.get(11).output.borrow().clone();
                let a0 = reg_file.get(10);
                ecall(a7, a0, a1, self.rom.clone());
            }
            EBREAK => {
                println!("ebreak not implemented yet")
            }
            _ => {}
        }
    }
}

impl Chip for Execute {
    // #[rustfmt::skip]
    fn compute(&mut self) {
        let instruction = self.input.borrow().clone();
        // println!("decoded instruction: {:?}", instruction);

        if self.halt != 0 {
            // println!("halted");
            self.halt -= 1;
            return;
        }
        self.execute(instruction)
    }

    fn clk(&mut self) {
        // Get the register that got updated
        let mut reg_file = self.reg_file.borrow_mut();
        let rd = reg_file.get(self.rd.0 as usize);
        // Clock the register and self output
        rd.clk();
    }
}

fn ecall(a7: U32, a0: &mut Register<U32>, a1: U32, rom: Wire<ROM>) {
    // handle syscall
    match a7.0 {
        1 => {
            // read char
            let mut buf = [0; 1];
            io::stdin().read_exact(&mut buf).unwrap();
            *a0.input.borrow_mut() = Wrapping(buf[0] as u32);
            *a0.load.borrow_mut() = true;
            a0.compute();
            a0.clk();
        }
        2 => {
            // print char
            let val = a0.output.borrow().clone();
            print!("{}", String::from_utf8_lossy(&[val.0 as u8]))
        }
        3 => {
            // read string
        }
        4 => {
            // should be encoded as lui no op
            // lui x0, `data`
            // print string
            let mut string = vec![];
            let a0 = a0.output.borrow().clone();
            for i in 0..a1.0 {
                let val = rom.borrow().peek(a0 + Wrapping(4 * i));
                string.push((val.0 >> 12) as u8);
            }
            print!("{}", String::from_utf8_lossy(&string))
        }
        10 => {
            let val = a0.output.borrow().clone();
            exit(val.0 as i32)
        }
        _ => {}
    }
}
