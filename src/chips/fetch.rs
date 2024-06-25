use crate::chips::rom::ROM;
use crate::chips::{Chip, Wire, U32};

pub struct Fetch<T = U32> {
    pub pc: Wire<T>,
    pub rom: Wire<ROM<T>>,
}

impl<T> Fetch<T>
where
    T: Copy,
{
    // Give the loaded rom to fetch
    pub fn new(pc: Wire<T>, rom: Wire<ROM<T>>) -> Self {
        Self { pc, rom }
    }
}

impl Chip for Fetch<U32> {
    fn compute(&mut self) {
        // set the address of rom
        *self.rom.borrow_mut().address.borrow_mut() = self.pc.borrow().clone();
        self.rom.borrow_mut().compute();
    }

    fn clk(&mut self) {
        // since the output is already piped through the rom clocking the rom should do the job
        self.rom.borrow_mut().clk()
    }
}
