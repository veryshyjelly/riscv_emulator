// Values of register file can be accessed in the same clock
// instead of second like ram

use crate::chips::register::Register;
use crate::chips::Chip;

pub struct RegFile<T> {
    registers: Vec<Register<T>>,
}

impl<T> RegFile<T>
where
    T: Clone + Default,
{
    pub fn new(size: usize) -> Self {
        let mut registers = Vec::with_capacity(size);
        for _ in 0..size {
            registers.push(Register::default());
        }
        Self { registers }
    }

    pub fn get(&mut self, index: usize) -> &mut Register<T> {
        if index == 0 {
            self.registers[index] = Register::default();
        }
        &mut self.registers[index]
    }
}

impl<T> Chip for RegFile<T>
where
    T: Copy + Default,
{
    fn compute(&mut self) {
        self.registers.iter_mut().for_each(|v| v.compute())
    }

    fn clk(&mut self) {
        self.registers.iter_mut().for_each(|v| v.clk())
    }
}
