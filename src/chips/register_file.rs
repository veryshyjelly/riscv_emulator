use crate::chips::register::Register;
use crate::chips::Chip;
use std::fmt::Debug;

#[derive(Clone)]
pub struct RegFile<T> {
    registers: Vec<Register<T>>,
}

impl<T> RegFile<T>
where
    T: Clone + Default + Debug,
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

    pub fn print(&self) {
        self.registers
            .iter()
            .enumerate()
            .for_each(|(i, v)| println!("x{i}: {:?}", v.output.borrow().clone()));
    }
}

impl<T> Chip for RegFile<T>
where
    T: Copy + Default,
{
    fn compute(&mut self) {
        // call compute on all the registers
        self.registers.iter_mut().for_each(|v| v.compute())
    }

    fn clk(&mut self) {
        // clock all the registers and reset the load
        self.registers.iter_mut().for_each(|v| {
            v.clk();
            *v.load.borrow_mut() = false;
        });
    }
}
