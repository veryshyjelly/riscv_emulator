use crate::chips::decode::{Decode, Instruction};
use crate::chips::execute::{Execute, IOCode};
use crate::chips::fetch::Fetch;
use crate::chips::memory_access::MemoryAccess;
use crate::chips::pc::PC;
use crate::chips::ram::RAM;
use crate::chips::register_file::RegFile;
use crate::chips::rom::ROM;
use crate::chips::{wire, Chip, Wire, U32, ZERO};

pub struct CPU {
    fetch: Fetch,
    decode: Decode,
    execute: Execute,
    memory_access: MemoryAccess,
    pc: Wire<PC>,
}

impl CPU {
    pub fn new(ram: RAM<U32>, rom: ROM) -> Self {
        let pc = wire(PC::new(
            wire(ZERO),
            wire(false),
            wire(false),
            wire(true),
            wire(ZERO),
        ));
        let reg_file = wire(RegFile::new(32));

        let fetch = Fetch::new(pc.borrow().output.clone(), rom);
        let decode = Decode::new(fetch.instruction.clone(), wire(Instruction::default()));
        let execute = Execute::new(
            decode.output.clone(),
            wire(IOCode::default()),
            reg_file.clone(),
            pc.clone(),
        );
        let memory_access = MemoryAccess::new(execute.output.clone(), reg_file, ram);

        Self {
            fetch,
            decode,
            execute,
            memory_access,
            pc,
        }
    }
}

impl Chip for CPU {
    fn compute(&mut self) {
        self.fetch.compute();
        self.decode.compute();
        self.execute.compute();
        self.memory_access.compute();
        self.pc.borrow_mut().compute();
    }

    fn clk(&mut self) {
        self.fetch.clk();
        self.decode.clk();
        self.execute.clk();
        self.memory_access.clk();
        self.pc.borrow_mut().clk();
    }
}
