pub fn nand_gate(input_a: u8, input_b: u8) -> u8 {
    match (input_a, input_b) {
        (1, 1) => 0,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nand_gate() {
        assert_eq!(nand_gate(0, 0), 1);
        assert_eq!(nand_gate(0, 1), 1);
        assert_eq!(nand_gate(1, 0), 1);
        assert_eq!(nand_gate(1, 1), 0);
    }
}
