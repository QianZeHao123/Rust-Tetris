// implement 1-bit register
use crate::atom_component::dff::DFlipFlop;
pub struct OneBitRegister {
    dff: DFlipFlop,
    load: i32, // Using an integer to represent the load signal
}

impl OneBitRegister {
    pub fn new() -> Self {
        OneBitRegister {
            dff: DFlipFlop::new(),
            load: 0, // 0 represents false (do not load)
        }
    }

    pub fn set_data_input(&mut self, value: i32) {
        self.dff.set_data_input(value);
    }

    pub fn set_load(&mut self, value: i32) {
        self.load = value;
    }

    pub fn clock_rising_edge(&mut self) {
        if self.load == 1 {
            // Equivalent to if self.load is true
            self.dff.clock_rising_edge();
        }
    }

    pub fn get_output(&self) -> i32 {
        self.dff.get_output()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one_bit_register() {
        let mut register = OneBitRegister::new();
        register.set_data_input(1);
        register.set_load(1);
        register.clock_rising_edge();
        assert_eq!(register.get_output(), 1);

        register.set_data_input(0);
        register.set_load(1);
        register.clock_rising_edge();
        assert_eq!(register.get_output(), 0);

        register.set_data_input(1);
        register.set_load(0);
        register.clock_rising_edge();
        assert_eq!(register.get_output(), 0);

        register.set_data_input(0);
        register.set_load(0);
        register.clock_rising_edge();
        assert_eq!(register.get_output(), 0);
    }
}