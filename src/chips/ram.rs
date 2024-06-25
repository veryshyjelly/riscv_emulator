use crate::chips::register::Register;
use crate::chips::{Chip, Wire, U32, ZERO};
use std::fmt::Debug;

pub struct RAM<T> {
    pub input: Wire<T>,
    pub output: Wire<T>,
    pub address: Wire<U32>,
    pub load: Wire<bool>,
    addr: U32,
    registers: Vec<Register<T>>,
}

impl<T> RAM<T>
where
    T: Default + Clone,
{
    pub fn new(
        input: Wire<T>,
        output: Wire<T>,
        address: Wire<U32>,
        load: Wire<bool>,
        size: usize,
    ) -> Self {
        // Create the specified number of registers
        let mut registers = Vec::with_capacity(size);
        for _ in 0..size {
            registers.push(Register::default());
        }
        Self {
            input,
            output,
            address,
            addr: ZERO,
            load,
            registers,
        }
    }

    pub fn peek(&self, addr: U32) -> T {
        self.registers[addr.0 as usize >> 2].output.borrow().clone()
    }
}

impl<T> Chip for RAM<T>
where
    T: Default + Clone + Debug,
{
    fn compute(&mut self) {
        let addr = self.address.borrow().clone();
        self.addr = addr;
        // Transfer the input and load from ram's interface to the selected register's interface
        *self.registers[addr.0 as usize >> 2].input.borrow_mut() = self.input.borrow().clone();
        *self.registers[addr.0 as usize >> 2].load.borrow_mut() = self.load.borrow().clone();
        // Now compute the selected ram
        self.registers[addr.0 as usize >> 2].compute();
    }

    fn clk(&mut self) {
        let addr = self.addr;
        // Clock the selected register
        self.registers[addr.0 as usize >> 2].clk();
        // Now move the output to ram's interface to see the result
        *self.output.borrow_mut() = self.registers[addr.0 as usize >> 2].output.borrow().clone();
    }
}
