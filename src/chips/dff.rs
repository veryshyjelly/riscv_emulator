use crate::chips::{Chip, Wire};

/**Modelling a D-Flip-Flop */
/**Equivalent to reg data type in verilog*/
#[derive(Clone)]
pub struct DFF<T> {
    pub input: Wire<T>,
    pub output: Wire<T>,
    next_value: T,
}

impl<T> DFF<T>
where
    T: Clone + Default,
{
    pub fn new(input: Wire<T>, output: Wire<T>) -> Self {
        Self {
            next_value: T::default(),
            input,
            output,
        }
    }
}

impl<T> Chip for DFF<T>
where
    T: Clone + Default,
{
    fn compute(&mut self) {
        self.next_value = self.input.borrow().clone();
    }

    fn clk(&mut self) {
        *self.output.borrow_mut() = self.next_value.clone();
    }
}
