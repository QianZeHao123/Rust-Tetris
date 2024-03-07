// data flip-flop (DFF) component
// A DFF is a 1-bit memory cell. It stores a single bit of data and has two outputs: Q and Q' (the inverse of Q).
// The DFF has two inputs: data input and clock input. The data input is the value to be stored in the memory cell.
// The clock input is used to control when the data input is stored in the memory cell.
// When the clock input is 1, the data input is stored in the memory cell. When the clock input is 0, the data input is ignored.
// The DFF has one output: the value stored in the memory cell.
// The DFF has two states: the current state and the next state. The current state is the value stored in the memory cell.

// from 0 to 1: clockUp --> Get Input, state = input
// from 1 to 0: clockDown --> Output, output = state

pub struct DFlipFlop {
    state: u8,
}

impl DFlipFlop {
    pub fn new() -> DFlipFlop {
        DFlipFlop { state: 0 }
    }
    pub fn clock(&mut self, input: u8, clock: u8) -> u8 {
        if clock == 1 {
            self.state = input;
        }
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dff() {
        let mut dff = DFlipFlop::new();
        assert_eq!(dff.clock(0, 0), 0);
        assert_eq!(dff.clock(0, 1), 0);
        assert_eq!(dff.clock(1, 1), 1);
        assert_eq!(dff.clock(0, 1), 0);
        assert_eq!(dff.clock(1, 0), 0);
        assert_eq!(dff.clock(0, 0), 0);
    }
}
