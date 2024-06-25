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
            0x0FF00513, 0x00400593, 0x00058A63, 0x02B572B3, 0x00058533, 0x000285B3, 0xFE0008E3,
            0x7CA03823, 0x00000513, 0x00A00893, 0x00000073,
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
        println!("cycle: {i}");
        println!("ram[2000] = {}", cpu.execute.ram.peek(Wrapping(2000)));
        // cpu.memory_access.reg_file.borrow().print();
    }
    
}
