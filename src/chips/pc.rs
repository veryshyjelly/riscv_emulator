use crate::chips::register::Register;
use crate::chips::{wire, Chip, Wire, ONE, U32, ZERO};

#[derive(Clone)]
pub struct PC<T = U32> {
    pub input: Wire<T>,
    pub reset: Wire<bool>,
    pub load: Wire<bool>,
    pub inc: Wire<bool>,
    pub output: Wire<T>,
    register: Register<T>,
}

impl PC {
    pub fn new(
        input: Wire<U32>,
        reset: Wire<bool>,
        load: Wire<bool>,
        inc: Wire<bool>,
        output: Wire<U32>,
    ) -> Self {
        Self {
            input,
            reset,
            load,
            inc,
            output: output.clone(),
            // connect the output of the register to pc's output
            register: Register::new(wire(ZERO), output, wire(true)),
        }
    }
}

impl Chip for PC {
    fn compute(&mut self) {
        let val = if *self.reset.borrow() {
            ZERO
        } else if *self.load.borrow() {
            self.input.borrow().clone()
        } else if *self.inc.borrow() {
            self.output.borrow().clone() + ONE
        } else {
            self.output.borrow().clone()
        };
        *self.register.input.borrow_mut() = val;

        self.register.compute(); // compute call karna na bhule
    }

    fn clk(&mut self) {
        // this load should automatically get false
        *self.load.borrow_mut() = false;
        self.register.clk()
    }
}
