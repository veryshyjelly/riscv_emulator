use crate::chips::register::Register;
use crate::chips::{wire, Chip, Wire, U32, ZERO};

pub struct RAM<T> {
    pub input: Wire<T>,
    pub output: Wire<T>,
    pub address: Wire<U32>,
    pub load: Wire<bool>,
    registers: Vec<Register<T>>,
}

impl<T> RAM<T>
where
    T: Default + Clone,
{
    pub fn new(size: usize) -> Self {
        // Create the specified number of registers
        let mut registers = Vec::with_capacity(size);
        for _ in 0..size {
            registers.push(Register::default());
        }
        Self {
            input: wire(T::default()),
            output: wire(T::default()),
            address: wire(ZERO),
            load: wire(false),
            registers,
        }
    }
}

impl<T> Chip for RAM<T>
where
    T: Default + Clone,
{
    fn compute(&mut self) {
        let addr = self.address.borrow().clone();
        // Transfer the input and load from ram's interface to the selected register's interface
        *self.registers[addr.0 as usize >> 2].input.borrow_mut() = self.input.borrow().clone();
        *self.registers[addr.0 as usize >> 2].load.borrow_mut() = self.load.borrow().clone();
        // Now compute the selected ram
        self.registers[addr.0 as usize >> 2].compute();
    }

    fn clk(&mut self) {
        let addr = self.address.borrow().clone();
        // Clock the selected register
        self.registers[addr.0 as usize >> 2].clk();
        // Now move the output to ram's interface to see the result
        *self.output.borrow_mut() = self.registers[addr.0 as usize].output.borrow().clone();
    }
}
