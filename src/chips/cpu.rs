use crate::chips::decode::{Decode, Instruction};
use crate::chips::execute::Execute;
use crate::chips::fetch::Fetch;
use crate::chips::pc::PC;
use crate::chips::ram::RAM;
use crate::chips::register_file::RegFile;
use crate::chips::rom::ROM;
use crate::chips::{wire, Chip, Wire, U32};

pub struct CPU<T = U32> {
    pub fetch: Fetch<T>,
    pub decode: Decode<T>,
    pub execute: Execute<T>,
    pc: Wire<PC>,
}

impl CPU {
    pub fn new(ram: RAM<U32>, rom: ROM) -> Self {
        let pc = wire(PC::default());
        let reg_file = wire(RegFile::new(32));
        let rom = wire(rom);

        let fetch = Fetch::new(pc.borrow().output.clone(), rom.clone());
        let decode = Decode::new(rom.clone(), wire(Instruction::default()));
        let execute = Execute::new(
            decode.output.clone(),
            ram,
            rom,
            reg_file.clone(),
            pc.clone(),
        );

        Self {
            fetch,
            decode,
            execute,
            pc,
        }
    }
}

impl Chip for CPU {
    fn compute(&mut self) {
        self.fetch.compute();
        self.decode.compute();
        self.execute.compute();
        self.pc.borrow_mut().compute();
    }

    fn clk(&mut self) {
        self.fetch.clk();
        self.decode.clk();
        self.execute.clk();
        self.pc.borrow_mut().clk();
    }
}
