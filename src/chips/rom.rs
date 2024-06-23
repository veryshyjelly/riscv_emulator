use crate::chips::{wire, Chip, Wire, U32, ZERO};

pub struct ROM<T = U32> {
    pub output: Wire<T>,
    pub address: Wire<T>,
    registers: Vec<T>,
}

impl ROM<U32> {
    pub fn new(size: usize) -> Self {
        Self {
            output: wire(ZERO),
            address: wire(ZERO),
            registers: vec![ZERO; size],
        }
    }

    pub fn load(&mut self, program: Vec<U32>) {
        // load the program into the rom
        program
            .into_iter()
            .enumerate()
            .for_each(|(i, p)| self.registers[i] = p);
    }
}

impl Chip for ROM<U32> {
    fn compute(&mut self) {
        // nothing to do
    }

    fn clk(&mut self) {
        let addr = self.address.borrow().clone();
        *self.output.borrow_mut() = self.registers[addr.0 as usize].clone();
    }
}
