use crate::chips::execute::IOCode;
use crate::chips::ram::RAM;
use crate::chips::register_file::RegFile;
use crate::chips::{Chip, Wire, U32, ZERO};

pub struct MemoryAccess<T = U32> {
    input: Wire<IOCode>,
    reg_file: Wire<RegFile<T>>,
    ram: RAM<T>,
    rd: U32,
}

impl MemoryAccess {
    pub fn new(input: Wire<IOCode>, reg_file: Wire<RegFile<U32>>, ram: RAM<U32>) -> Self {
        Self {
            input,
            reg_file,
            ram,
            rd: ZERO,
        }
    }
}

impl Chip for MemoryAccess {
    fn compute(&mut self) {
        let io_code = self.input.borrow().clone();
        *self.ram.address.borrow_mut() = io_code.address;
        *self.ram.load.borrow_mut() = io_code.store;
        self.rd = io_code.register;

        println!("io_code = {:?}", io_code);

        if io_code.store {
            *self.ram.input.borrow_mut() = self
                .reg_file
                .borrow_mut()
                .get(io_code.register.0 as usize)
                .output
                .borrow()
                .clone();
        }
        self.ram.compute();
        self.ram.clk();

        let mut reg_file = self.reg_file.borrow_mut();
        let rd = reg_file.get(io_code.register.0 as usize);
        if !io_code.store {
            *rd.input.borrow_mut() = self.ram.output.borrow().clone();
            *rd.load.borrow_mut() = true;
        }
        rd.compute();
    }

    fn clk(&mut self) {
        let mut reg_file = self.reg_file.borrow_mut();
        let rd = reg_file.get(self.rd.0 as usize);
        rd.clk();
    }
}
