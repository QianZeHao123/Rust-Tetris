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