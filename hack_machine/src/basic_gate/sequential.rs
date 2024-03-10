// BitRegister
// clockUp: if (load == 1), then state = input
// clockDown: output = state

use crate::atom_component::dff;
use crate::basic_gate::basic::mux_gate;
use crate::basic_gate::combinational::dmux8way_gate;
use crate::basic_gate::combinational::mux8way16_gate;
// -------------------------------------------------------------------------
// BitRegister
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

// -------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bit_register() {
        let mut bit_register = BitRegister::new();
        assert_eq!(bit_register.clock(1, 1, 0), 0);
        assert_eq!(bit_register.clock(1, 1, 1), 1);
    }
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

}
