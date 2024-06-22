use std::cell::RefCell;

mod cpu;
mod decode;
mod dff;
mod execute;
mod fetch;
mod memory_access;
mod pc;
mod ram;
mod register;
mod register_file;
mod rom;

/**
   For sequential circuits the chip trait should be implemented
*/
pub trait Chip {
    /**Compute the sequential logic describe in the circuit*/
    fn compute(&mut self) {}
    /**Update the values for the next cycle*/
    fn clk(&mut self) {}
}

type Wire<T> = RefCell<T>;

fn mux2<T>(a: T, b: T, sel: bool) -> T {
    match sel {
        true => b,
        false => a,
    }
}
