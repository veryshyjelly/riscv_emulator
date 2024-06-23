use std::cell::RefMut;
use crate::chips::cpu::CPU;
use crate::chips::ram::RAM;
use crate::chips::rom::ROM;
use crate::chips::{Chip, U32};
use std::num::Wrapping;
use crate::chips::register_file::RegFile;

mod chips;

fn main() {
    let ram: RAM<U32> = RAM::new(1024 * 1024);
    let mut rom = ROM::new(1024);

    rom.load(
        [
            0x00058a63, 0x02b572b3, 0x00058533, 0x000285b3, 0xfe0008e3, 0x7ca03823,
        ]
        .map(|x: u32| Wrapping(x))
        .into_iter()
        .collect::<Vec<_>>(),
    );

    let mut cpu = CPU::new(ram, rom);

    set_register(cpu.memory_access.reg_file.borrow_mut(), 10, Wrapping(255));
    set_register(cpu.memory_access.reg_file.borrow_mut(), 11, Wrapping(4));

    for i in 1..10 {
        cpu.compute();
        cpu.clk();
        println!("cycle: {i}");
        cpu.memory_access.reg_file.borrow().print();
    }
}

fn set_register(mut reg_file: RefMut<RegFile<U32>>, idx: usize, value: U32) {
    let reg = reg_file.get(idx);
    *reg.input.borrow_mut() = value;
    *reg.load.borrow_mut() = true;
    reg.compute();
    reg.clk();
}
