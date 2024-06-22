use crate::chips::rom::ROM;
use crate::chips::{Chip, Wire};

struct Fetch<T = u32> {
    pub instruction: Wire<T>,
    pub pc: Wire<T>,
    rom: ROM<T>,
}

impl<T> Fetch<T>
where
    T: Copy,
{
    // Give the loaded rom to fetch
    fn new(pc: Wire<T>, rom: ROM<T>) -> Self {
        Self {
            instruction: rom.output.clone(),
            pc,
            rom,
        }
    }
}

impl Chip for Fetch<u32> {
    fn compute(&mut self) {
        // set the address of rom
        *self.rom.address.borrow_mut() = self.pc.borrow().clone();
        self.rom.compute();
    }

    fn clk(&mut self) {
        // since the output is already piped through the rom clocking the rom should do the job
        self.rom.clk()
    }
}
