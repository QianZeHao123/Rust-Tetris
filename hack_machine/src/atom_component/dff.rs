struct DFlipFlop {
    data_input: i32,
    output_q: i32,
}

impl DFlipFlop {
    fn new() -> Self {
        DFlipFlop {
            data_input: 0, // 0 represents false
            output_q: 0,   // 0 represents false
        }
    }

    fn set_data_input(&mut self, value: i32) {
        self.data_input = value;
    }

    fn clock_rising_edge(&mut self) {
        self.output_q = self.data_input;
    }

    fn get_output(&self) -> i32 {
        self.output_q
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dff2() {
        let mut dff = DFlipFlop::new();
        dff.set_data_input(1);
        dff.clock_rising_edge();
        assert_eq!(dff.get_output(), 1);

        dff.set_data_input(0);
        dff.clock_rising_edge();
        assert_eq!(dff.get_output(), 0);
    }
}
