use crate::basic_gate::register::SixteenBitRegister;

pub struct RAM8 {
    registers: [SixteenBitRegister; 8], // 8 registers, each 16 bits wide
}

impl RAM8 {
    pub fn new() -> Self {
        RAM8 {
            registers: std::array::from_fn(|_| SixteenBitRegister::new()),
        }
    }

    // Writes a 16-bit value to one of the 8 registers based on the address
    pub fn write(&mut self, address: usize, value: &[i32; 16]) {
        if address < 8 {
            self.registers[address].set_data_input(value);
            self.registers[address].set_load(1); // Enable load to write the value
            self.registers[address].clock_rising_edge(); // Simulate the clock edge to store the value
            self.registers[address].set_load(0); // Disable load after writing
        } else {
            panic!("Invalid address: {}. RAM8 supports addresses 0-7.", address);
        }
    }

    // Reads a 16-bit value from one of the 8 registers based on the address
    pub fn read(&self, address: usize) -> [i32; 16] {
        if address < 8 {
            self.registers[address].get_output()
        } else {
            panic!("Invalid address: {}. RAM8 supports addresses 0-7.", address);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram8_read_write() {
        let mut ram8 = RAM8::new();

        // Test writing and reading from a specific address
        let test_address = 3; // Choose an address to test
        let test_value = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        ram8.write(test_address, &test_value);

        assert_eq!(
            ram8.read(test_address),
            test_value,
            "The value read from address {} should match the value written",
            test_address
        );

        // Ensure other addresses are unaffected
        for address in 0..8 {
            if address != test_address {
                assert_eq!(
                    ram8.read(address),
                    [0; 16],
                    "Address {} should be unaffected and remain initialized to zero",
                    address
                );
            }
        }
    }
    #[test]
    fn test_ram8_write_read_all_addresses() {
        let mut ram8 = RAM8::new();
        let test_values = [
            [1; 16],
            [0; 16],
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1],
            [1; 16],
            [0; 16],
        ];

        // Write to each address and then immediately read back the value
        for (address, &value) in test_values.iter().enumerate() {
            ram8.write(address, &value);
            let read_value = ram8.read(address);
            assert_eq!(read_value, value, "Mismatch at address {}", address);
        }
    }

    #[test]
    fn test_ram8_no_overwrite() {
        let mut ram8 = RAM8::new();
        let initial_value = [0; 16];
        let test_value = [1; 16];
        let test_address = 4;

        // Initially, all addresses should hold the initial value
        for address in 0..8 {
            assert_eq!(
                ram8.read(address),
                initial_value,
                "RAM not initialized to 0 at address {}",
                address
            );
        }

        // Write the test value to the test address
        ram8.write(test_address, &test_value);

        // All addresses except the test address should still hold the initial value
        for address in 0..8 {
            if address != test_address {
                assert_eq!(
                    ram8.read(address),
                    initial_value,
                    "Data at address {} was inadvertently modified",
                    address
                );
            }
        }

        // The test address should hold the test value
        assert_eq!(
            ram8.read(test_address),
            test_value,
            "Test value not correctly written to address {}",
            test_address
        );
    }
    #[test]
    fn test_ram8_edge_cases() {
        let mut ram8 = RAM8::new();
        let first_address = 0;
        let last_address = 7;
        let first_value = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let last_value = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];

        // Write and read from the first address
        ram8.write(first_address, &first_value);
        assert_eq!(ram8.read(first_address), first_value, "Failed to write/read first address");

        // Write and read from the last address
        ram8.write(last_address, &last_value);
        assert_eq!(ram8.read(last_address), last_value, "Failed to write/read last address");
    }
}
