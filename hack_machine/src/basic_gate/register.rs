use crate::basic_gate::bit::OneBitRegister;

pub struct SixteenBitRegister {
    bits: [OneBitRegister; 16],
}

impl SixteenBitRegister {
    // pub fn new() -> Self {
    //     SixteenBitRegister {
    //         bits: [OneBitRegister::new(); 16], // Initialize all 16 OneBitRegisters
    //     }
    // }
    pub fn new() -> Self {
        SixteenBitRegister {
            bits: std::array::from_fn(|_| OneBitRegister::new()), // Initialize each element individually
        }
    }

    // Takes a slice of 16 i32 values as the data input
    // pub fn set_data_input(&mut self, values: &[i32; 16]) {
    //     for (i, &value) in values.iter().enumerate() {
    //         // Ensure each value is only 0 or 1
    //         if value == 0 || value == 1 {
    //             self.bits[i].set_data_input(value);
    //         } else {
    //             panic!("Invalid bit value: {}. Each bit must be 0 or 1.", value);
    //         }
    //     }
    // }

    // pub fn set_load(&mut self, value: i32) {
    //     for bit in self.bits.iter_mut() {
    //         bit.set_load(value); // Set the load signal for all OneBitRegisters
    //     }
    // }
    pub fn set_data_input(&mut self, values: &[i32; 16]) {
        for (i, &value) in values.iter().enumerate() {
            let value_u8: u8 = value.try_into().expect("Value must be 0 or 1");
            self.bits[i].set_data_input(value_u8);
        }
    }

    pub fn set_load(&mut self, value: i32) {
        let value_u8: u8 = value.try_into().expect("Load value must be 0 or 1");
        for bit in self.bits.iter_mut() {
            bit.set_load(value_u8);
        }
    }

    pub fn clock_rising_edge(&mut self) {
        for bit in self.bits.iter_mut() {
            bit.clock_rising_edge(); // Simulate the clock rising edge for all OneBitRegisters
        }
    }

    pub fn get_output(&self) -> [i32; 16] {
        let mut output = [0; 16];
        for (i, bit) in self.bits.iter().enumerate() {
            output[i] = bit.get_output(); // Get the output of each OneBitRegister
        }
        output
    }
}

// -------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sixteen_bit_register_load_and_hold() {
        let mut register = SixteenBitRegister::new();

        // Test loading a value into the register
        let input_value = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        register.set_data_input(&input_value);
        register.set_load(1); // Enable load
        register.clock_rising_edge(); // Load the input value
        assert_eq!(
            register.get_output(),
            input_value,
            "The register output should match the loaded input"
        );

        // Test holding the current value when load is disabled
        let new_input_value = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        register.set_data_input(&new_input_value);
        register.set_load(0); // Disable load
        register.clock_rising_edge(); // Attempt to load the new input value
        assert_eq!(
            register.get_output(),
            input_value,
            "The register output should remain unchanged when load is disabled"
        );
    }

    #[test]
    fn test_sixteen_bit_register_update() {
        let mut register = SixteenBitRegister::new();

        // Test updating the register with a new value
        let initial_value = [0; 16]; // Start with all zeros
        register.set_data_input(&initial_value);
        register.set_load(1);
        register.clock_rising_edge();
        assert_eq!(
            register.get_output(),
            initial_value,
            "The register should initially be all zeros"
        );

        let updated_value = [1; 16]; // Update to all ones
        register.set_data_input(&updated_value);
        register.set_load(1); // Enable load
        register.clock_rising_edge(); // Load the updated value
        assert_eq!(
            register.get_output(),
            updated_value,
            "The register output should be updated to all ones"
        );
    }

    // #[test]
    // #[should_panic(expected = "Invalid bit value")]
    // fn test_sixteen_bit_register_invalid_input() {
    //     let mut register = SixteenBitRegister::new();

    //     // Test providing an invalid input value (values other than 0 or 1)
    //     let invalid_input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    //     register.set_data_input(&invalid_input); // This should panic
    // }
}
