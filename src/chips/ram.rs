use crate::chips::register::Register;
use crate::chips::{Chip, Wire};

pub struct RAM<T> {
    pub input: Wire<T>,
    pub output: Wire<T>,
    pub address: Wire<u32>,
    pub load: Wire<bool>,
    registers: Vec<Register<T>>,
}

impl<T> RAM<T>
where
    T: Default + Clone,
{
    pub fn new(
        input: Wire<T>,
        output: Wire<T>,
        address: Wire<u32>,
        load: Wire<bool>,
        size: usize,
    ) -> Self {
        let mut registers = Vec::with_capacity(size);
        for _ in 0..size {
            registers.push(Register::default());
        }
        Self {
            input,
            output,
            address,
            load,
            registers,
        }
    }
}

impl<T> Chip for RAM<T>
where
    T: Default + Clone,
{
    fn compute(&mut self) {
        // Make the connections to the respective register
        let addr = self.address.borrow().clone();
        *self.registers[addr as usize].input.borrow_mut() = self.input.borrow().clone();
        *self.registers[addr as usize].load.borrow_mut() = self.load.borrow().clone();
        self.registers[addr as usize].compute();
    }

    fn clk(&mut self) {
        let addr = self.address.borrow().clone();
        // clock the register
        self.registers[addr as usize].clk();
        // then copy the value
        *self.output.borrow_mut() = self.registers[addr as usize].output.borrow().clone();
    }
}
