// BitRegister, Register, RAM8

use crate::atom_component::dff;
use crate::basic_gate::basic::mux_gate;
use crate::basic_gate::combinational::dmux8way_gate;
use crate::basic_gate::combinational::mux8way16_gate;
// -------------------------------------------------------------------------
// BitRegister
// clockUp: if (load == 1), then state = input
// clockDown: output = state
pub struct BitRegister {
    dff: dff::DFlipFlop,
    state: u8,
}

// implement BitRegister using DFlipFlop
impl BitRegister {
    pub fn new() -> BitRegister {
        BitRegister {
            dff: dff::DFlipFlop::new(),
            state: 0,
        }
    }
    pub fn clock(&mut self, input: u8, load: u8, clock: u8) -> u8 {
        let new_state = self.dff.clock(input, clock);
        self.state = mux_gate(self.state, new_state, load);
        self.state
    }
}

// -------------------------------------------------------------------------
// 16 bit register
pub struct Register {
    bit_register: Vec<BitRegister>, // Changed to Vec for dynamic allocation
    state: [u8; 16],
}

impl Register {
    pub fn new() -> Register {
        let mut bit_registers = Vec::with_capacity(16);
        for _ in 0..16 {
            bit_registers.push(BitRegister::new());
        }

        Register {
            bit_register: bit_registers,
            state: [0; 16],
        }
    }

    pub fn clock(&mut self, input: [u8; 16], load: u8, clock: u8) -> [u8; 16] {
        for i in 0..16 {
            self.state[i] = self.bit_register[i].clock(input[i], load, clock);
        }
        self.state
    }
}

// -------------------------------------------------------------------------
// RAM 8 (Random Access Memory)
// implement with REgister and Mux8Way16
// Read:  out(t) = RAM8[address(t)](t)
// Write: If load(t-1) then RAM8[address(t-1)](t) = in(t-1)

pub struct RAM8 {
    registers: Vec<Register>,
    output: [u8; 16],              // Current output state
    read_address: Option<[u8; 3]>, // Stores the read address for the next cycle
}

impl RAM8 {
    pub fn new() -> RAM8 {
        let mut registers = Vec::with_capacity(8);
        for _ in 0..8 {
            registers.push(Register::new());
        }

        RAM8 {
            registers,
            output: [0; 16],    // Initialize with zeros
            read_address: None, // Initialize without a read address
        }
    }

    pub fn clock(&mut self, input: [u8; 16], load: u8, address: [u8; 3], clock: u8) {
        let load_signals = dmux8way_gate(load, address);

        // Write to the appropriate register
        for i in 0..8 {
            // if load_signals[i] == 1 {
            self.registers[i].clock(input, load_signals[i], clock);
            // }
        }

        // If there was a read address from the previous cycle, use it to update the output
        if let Some(read_address) = self.read_address {
            self.output = self.read(read_address);
        }

        // Store the current address for reading in the next cycle
        self.read_address = Some(address);
    }

    // Separate read method to get the value from the specified address
    pub fn read(&self, address: [u8; 3]) -> [u8; 16] {
        mux8way16_gate(
            self.registers[0].state,
            self.registers[1].state,
            self.registers[2].state,
            self.registers[3].state,
            self.registers[4].state,
            self.registers[5].state,
            self.registers[6].state,
            self.registers[7].state,
            address,
        )
    }
}

// -------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // test BitRegister
    #[test]
    fn test_bit_register() {
        let mut bit_register = BitRegister::new();
        assert_eq!(bit_register.clock(1, 1, 0), 0);
        assert_eq!(bit_register.clock(1, 1, 1), 1);
    }

    // test Register
    #[test]
    fn test_register() {
        let mut register = Register::new();
        // assert_eq!(register.clock([1; 16], 1, 0), [0; 16]);
        assert_eq!(register.clock([1; 16], 1, 1), [1; 16]);
        // assert_eq!(
        //     register.clock([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1, 0),
        //     [1; 16]
        // );
        assert_eq!(
            register.clock([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1, 1),
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    // test RAM8
    #[test]
    fn test_ram8() {
        let mut ram8 = RAM8::new();
        ram8.clock([1; 16], 1, [0, 0, 0], 1);
        ram8.clock(
            [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            1,
            [1, 0, 1],
            1,
        );
        assert_eq!(ram8.read([0, 0, 0]), [1; 16]);
        assert_eq!(
            ram8.read([1, 0, 1]),
            [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        ram8.clock(
            [1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
            [1, 0, 1],
            1,
        );
        assert_eq!(
            ram8.read([1, 0, 1]),
            [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        ram8.clock(
            [1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            1,
            [1, 0, 1],
            1,
        );
        assert_eq!(
            ram8.read([1, 0, 1]),
            [1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
