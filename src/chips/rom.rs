use crate::chips::{Chip, Wire, U32};

pub struct ROM<T = U32> {
    pub address: Wire<T>,
    pub output: Wire<T>,
    registers: Vec<T>,
}

impl<T> ROM<T>
where
    T: Default + Clone,
{
    pub fn new(output: Wire<T>, address: Wire<T>, size: usize) -> Self {
        Self {
            output,
            address,
            registers: vec![T::default(); size],
        }
    }

    pub fn load(&mut self, program: Vec<T>) {
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
        *self.output.borrow_mut() = self.registers[addr.0 as usize / 4].clone();
    }
}
