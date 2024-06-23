use crate::chips::dff::DFF;
use crate::chips::{mux2, wire, Chip, Wire};

#[derive(Clone)]
pub struct Register<T> {
    pub input: Wire<T>,
    pub output: Wire<T>,
    pub load: Wire<bool>,
    dff: DFF<T>,
}

impl<T> Register<T>
where
    T: Clone + Default,
{
    pub fn new(input: Wire<T>, output: Wire<T>, load: Wire<bool>) -> Self {
        Self {
            input: input.clone(),
            output: output.clone(),
            load,
            dff: DFF::new(input, output),
        }
    }
}

impl<T> Default for Register<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        // The default register with required connections
        let out_wire = wire(T::default());
        let in_wire = wire(T::default());
        let dff = DFF::new(in_wire.clone(), out_wire.clone());
        Self {
            input: in_wire,
            output: out_wire,
            load: wire(false),
            dff,
        }
    }
}

impl<T> Chip for Register<T>
where
    T: Clone + Default,
{
    fn compute(&mut self) {
        self.dff.input = mux2(
            self.dff.output.clone(),
            self.input.clone(),
            *self.load.borrow(),
        );
        /*Compute all the components after doing connection*/
        self.dff.compute();
    }

    fn clk(&mut self) {
        /*since output is piped during the new function only clocking dff will do*/
        self.dff.clk();
    }
}
