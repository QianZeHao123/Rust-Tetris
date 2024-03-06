// BitRegister
// clockUp: if (load == 1), then state = input
// clockDown: output = state

pub use crate::atom_component::dff;
use crate::basic_gate::basic::mux_gate;

pub struct BitRegister {
    dff: dff::DFlipFlop,
    state: u8
}

// implement BitRegister using DFlipFlop
impl BitRegister {
    pub fn new() -> BitRegister {
        BitRegister {
            dff: dff::DFlipFlop::new(),
            state: 0
        }
    }
    pub fn clock(&mut self, input: u8, load: u8, clock: u8) -> u8 {
        let new_state = self.dff.clock(input, clock);
        self.state = mux_gate(self.state, new_state, load);
        self.state
    }
}
