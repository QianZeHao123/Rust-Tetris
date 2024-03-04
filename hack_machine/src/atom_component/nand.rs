pub fn nand_gate(input_a: u8, input_b: u8) -> u8 {
    match (input_a, input_b) {
        (1, 1) => 0,
        _ => 1,
    }
}
