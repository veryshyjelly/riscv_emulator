use crate::chips::register::Register;
use crate::chips::{Chip, Wire};
use std::cell::RefCell;

pub struct PC {
    pub input: Wire<u32>,
    pub reset: Wire<bool>,
    pub load: Wire<bool>,
    pub inc: Wire<bool>,
    pub output: Wire<u32>,
    register: Register<u32>,
}

impl PC {
    pub fn new(
        input: Wire<u32>,
        reset: Wire<bool>,
        load: Wire<bool>,
        inc: Wire<bool>,
        output: Wire<u32>,
    ) -> Self {
        Self {
            input,
            reset,
            load,
            inc,
            output: output.clone(),
            register: Register::new(RefCell::new(0), output, RefCell::new(true)),
        }
    }
}

impl Chip for PC {
    fn compute(&mut self) {
        let val = if *self.reset.borrow() {
            0
        } else if *self.load.borrow() {
            self.input.borrow().clone()
        } else if *self.inc.borrow() {
            self.output.borrow().clone() + 1
        } else {
            self.output.borrow().clone()
        };
        *self.register.input.borrow_mut() = val;

        self.register.compute(); // compute call karna na bhule
    }

    fn clk(&mut self) {
        self.register.clk()
    }
}
