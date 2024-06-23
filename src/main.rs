use crate::chips::cpu::CPU;
use crate::chips::ram::RAM;
use crate::chips::rom::ROM;
use crate::chips::{Chip, U32};
use std::num::Wrapping;

mod chips;

fn main() {
    let ram: RAM<U32> = RAM::new(1024 * 1024);
    let mut rom = ROM::new(1024);

    rom.load(
        [
            0b000000000010_00000_000_00001_0010011,
            0b000000000001_00001_000_00011_0010011,
            0b000000_00001_00010_000_00100_0110011,
            0b010000_00001_00010_000_00100_0110011,
        ]
        .map(|x: u32| Wrapping(x))
        .into_iter()
        .collect::<Vec<_>>(),
    );

    let mut cpu = CPU::new(ram, rom);

    for _ in 1..10 {
        cpu.compute();
        cpu.clk();
    }
}
