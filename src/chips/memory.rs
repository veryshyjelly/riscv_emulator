use super::{ram::RAM, screen::Screen, Chip, Wire, U32, ZERO};
use std::num::Wrapping;

pub struct Memory<T> {
    pub input: Wire<T>,
    pub output: Wire<T>,
    pub address: Wire<U32>,
    pub load: Wire<bool>,
    screen: Screen,
    ram: RAM<T>,
}

impl<T> Memory<T>
where
    T: Default + Clone,
{
    pub fn new(
        input: Wire<T>,
        address: Wire<U32>,
        load: Wire<bool>,
        screen: Screen,
        ram: RAM<T>,
    ) -> Self {
        Self {
            input,
            output: ram.output.clone(),
            address,
            load,
            screen,
            ram,
        }
    }
}

const SCREEN: u32 = 1024 * 1024 * 4;

impl Chip for Memory<U32> {
    fn compute(&mut self) {
        let addr = self.address.borrow().clone();
        if addr.0 >= SCREEN {
            *self.screen.address.borrow_mut() = Wrapping(addr.0 - SCREEN);
            *self.screen.input.borrow_mut() = self.input.borrow().clone();
        } else {
            *self.ram.address.borrow_mut() = addr;
            *self.ram.input.borrow_mut() = self.input.borrow().clone();
        }
    }

    fn clk(&mut self) {
        self.ram.clk();
        self.screen.clk();
    }
}
