use crate::chips::cpu::CPU;
use crate::chips::ram::RAM;
use crate::chips::rom::ROM;
use crate::chips::{wire, Chip, U32, ZERO};
use std::num::Wrapping;

mod chips;

fn main() {
    let ram: RAM<U32> = RAM::new(wire(ZERO), wire(ZERO), wire(ZERO), wire(false), 1024 * 1024);
    let mut rom = ROM::new(wire(ZERO), wire(ZERO), 1024);

    rom.load(
        [
        		0x00200893,
        		0x04800513,
        		0x00000073,
        		0x06500513,
        		0x00000073,
        		0x06C00513,
        		0x00000073,
        		0x00000073,
        		0x06F00513,
        		0x00000073,
        		0x02000513,
        		0x00000073,
        		0x05700513,
        		0x00000073,
        		0x06F00513,
        		0x00000073,
        		0x07200513,
        		0x00000073,
        		0x06C00513,
        		0x00000073,
        		0x06400513,
        		0x00000073,
        		0x00000513,
        		0x00A00893,
        		0x00000073,
        ]
        .map(|x: u32| Wrapping(x))
        .into_iter()
        .collect::<Vec<_>>(),
    );

    let mut cpu = CPU::new(ram, rom);

    let mut i = 0;
    loop {
        i += 1;
        cpu.compute();
        cpu.clk();
        // println!("cycle: {i}");
        // println!("ram[2000] = {}", cpu.execute.ram.peek(Wrapping(2000)));
        // cpu.memory_access.reg_file.borrow().print();
    }
}
