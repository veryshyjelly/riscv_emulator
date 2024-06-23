use std::cell::RefCell;
use std::num::Wrapping;
use std::rc::Rc;

pub mod cpu;
pub mod decode;
pub mod dff;
pub mod execute;
pub mod fetch;
pub mod memory_access;
pub mod pc;
pub mod ram;
pub mod register;
pub mod register_file;
pub mod rom;

/**
   For sequential circuits the chip trait should be implemented
*/
pub trait Chip {
    /**Compute the sequential logic describe in the circuit*/
    fn compute(&mut self) {}
    /**Update the values for the next cycle*/
    fn clk(&mut self) {}
}

type Wire<T> = Rc<RefCell<T>>;

fn wire<T>(t: T) -> Wire<T> {
    Rc::new(RefCell::new(t))
}

pub type U32 = Wrapping<u32>;

const ZERO: U32 = Wrapping(0);
const ONE: U32 = Wrapping(1);

fn mux2<T>(a: T, b: T, sel: bool) -> T {
    match sel {
        true => b,
        false => a,
    }
}
