// implement 1-bit register
use crate::atom_component::dff::DFlipFlop;
use crate::basic_gate::basic::mux_gate;

pub struct OneBitRegister {
    dff: DFlipFlop,
    load: u8,          // Using u8 to match the mux_gate signature
    current_input: u8, // To hold the current input value
}

impl OneBitRegister {
    pub fn new() -> Self {
        OneBitRegister {
            dff: DFlipFlop::new(),
            load: 0,          // 0 represents false (do not load)
            current_input: 0, // Initial input value
        }
    }

    pub fn set_data_input(&mut self, value: u8) {
        self.current_input = value; // Just store the value, actual loading happens in clock_rising_edge
    }

    pub fn set_load(&mut self, value: u8) {
        self.load = value;
    }

    pub fn clock_rising_edge(&mut self) {
        let input_to_dff = mux_gate(self.dff.get_output() as u8, self.current_input, self.load);
        self.dff.set_data_input(input_to_dff as i32);
        self.dff.clock_rising_edge();
    }

    pub fn get_output(&self) -> i32 {
        self.dff.get_output()
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Make sure to import everything from the parent module

    // Assuming the existence of DFlipFlop and mux_gate as well as their supportive functions.
    // You might need to adjust the imports based on your actual module structure.

    #[test]
    fn test_register_loads_new_input() {
        let mut register = OneBitRegister::new();
        register.set_data_input(1); // Set new input to be 1
        register.set_load(1); // Enable loading
        register.clock_rising_edge(); // Simulate clock rising edge

        // Expect the register to update its output to the new input
        assert_eq!(
            register.get_output(),
            1,
            "Register did not load the new input when load was enabled."
        );
    }

    #[test]
    fn test_register_holds_current_value() {
        let mut register = OneBitRegister::new();
        register.set_data_input(1); // Set new input to be 1
        register.set_load(1); // Enable loading
        register.clock_rising_edge(); // Simulate clock rising edge

        // Change the input but disable loading
        register.set_data_input(0); // Change input to 0
        register.set_load(0); // Disable loading
        register.clock_rising_edge(); // Simulate another clock rising edge

        // Expect the register to retain its current output, ignoring the new input
        assert_eq!(
            register.get_output(),
            1,
            "Register did not hold its current value when load was disabled."
        );
    }
}
