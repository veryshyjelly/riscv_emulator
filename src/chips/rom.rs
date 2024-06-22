use crate::chips::{Chip, Wire};

pub struct ROM<T = u32> {
    pub output: Wire<T>,
    pub address: Wire<T>,
    registers: Vec<T>,
}

impl ROM<u32> {
    fn new(output: Wire<u32>, address: Wire<u32>, size: usize) -> Self {
        Self {
            output,
            address,
            registers: vec![0; size],
        }
    }

    fn load(&mut self, program: Vec<u32>) {
        // load the program into the rom
        program
            .into_iter()
            .enumerate()
            .for_each(|(i, p)| self.registers[i] = p);
    }
}

impl Chip for ROM<u32> {
    fn compute(&mut self) {
        // nothing to do
    }

    fn clk(&mut self) {
        let addr = self.address.borrow().clone();
        *self.output.borrow_mut() = self.registers[addr as usize].clone();
    }
}
