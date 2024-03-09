// BitRegister
// clockUp: if (load == 1), then state = input
// clockDown: output = state

use crate::atom_component::dff;
use crate::basic_gate::basic::mux_gate;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bit_register() {
        let mut bit_register = BitRegister::new();
        println!("bit_register.clock(1, 1, 0) = {}", bit_register.clock(1, 1, 0));
        println!("bit_register.clock(1, 1, 1) = {}", bit_register.clock(1, 1, 1));
        println!("bit_register.clock(0, 0, 0) = {}", bit_register.clock(0, 0, 0));
        println!("bit_register.clock(0, 0, 1) = {}", bit_register.clock(0, 0, 1));
        println!("bit_register.clock(1, 0, 0) = {}", bit_register.clock(1, 0, 0));
        println!("bit_register.clock(1, 0, 1) = {}", bit_register.clock(1, 0, 1));
        println!("bit_register.clock(0, 1, 0) = {}", bit_register.clock(0, 1, 0));
        println!("bit_register.clock(0, 1, 1) = {}", bit_register.clock(0, 1, 1));
    }
}
