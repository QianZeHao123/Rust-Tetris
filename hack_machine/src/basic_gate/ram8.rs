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
}
