use chips::screen::Screen;

use crate::chips::cpu::CPU;
use crate::chips::ram::RAM;
use crate::chips::rom::ROM;
use crate::chips::{wire, Chip, U32, ZERO};
use std::num::Wrapping;
use std::ops::{Deref, DerefMut};
use std::thread;

mod chips;

fn main() {
    let ram: RAM<U32> = RAM::new(wire(ZERO), wire(ZERO), wire(ZERO), wire(false), 1024 * 1024);
    let mut rom = ROM::new(wire(ZERO), wire(ZERO), 1024);

    rom.load(
        [
            0x00000263, 0x03800513, 0x00D00593, 0x00400893, 0x00000073, 0x00A00493, 0x00100893,
            0x00000073, 0x00200893, 0x00000073, 0xFE9518E3, 0x00000513, 0x00A00893, 0x00000073,
            0x00048037, 0x00065037, 0x0006C037, 0x0006C037, 0x0006F037, 0x0002C037, 0x00020037,
            0x00057037, 0x0006F037, 0x00072037, 0x0006C037, 0x00064037, 0x0000A037,
        ]
        .map(|x: u32| Wrapping(x))
        .into_iter()
        .collect::<Vec<_>>(),
    );

    let screen = Screen::new(wire(ZERO), wire(ZERO));

    let mut cpu = CPU::new(ram, rom, screen);

    // let mut i = 0;
    loop {
        // i += 1;
        cpu.compute();
        cpu.clk();
        // println!("cycle: {i}");
        // println!("ram[2000] = {}", cpu.execute.ram.peek(Wrapping(2000)));
        // cpu.memory_access.reg_file.borrow().print();
    }
}
