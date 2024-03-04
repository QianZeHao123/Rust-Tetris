// data flip-flop (DFF) component
// A DFF is a 1-bit memory cell. It stores a single bit of data and has two outputs: Q and Q' (the inverse of Q).
// The DFF has two inputs: data input and clock input. The data input is the value to be stored in the memory cell.
// The clock input is used to control when the data input is stored in the memory cell.
// When the clock input is 1, the data input is stored in the memory cell. When the clock input is 0, the data input is ignored.
// The DFF has one output: the value stored in the memory cell.
// The DFF has two states: the current state and the next state. The current state is the value stored in the memory cell.

pub struct DFlipFlop {
    current_data_input: u8, // Data input
    last_data_input: u8,    // Current state/output
}
impl DFlipFlop {
    pub fn new() -> DFlipFlop {
        DFlipFlop {
            current_data_input: 0,
            last_data_input: 0,
        }
    }
    pub fn clock(&mut self, data_input: u8, clock_input: u8) -> u8 {
        if clock_input == 1 {
            self.last_data_input = self.current_data_input;
            self.current_data_input = data_input;
        }
        self.last_data_input
    }
}
