// mod atom_component;
pub use crate::atom_component::nand;

pub fn and_gate(input_a: u8, input_b: u8) -> u8 {
    nand::nand_gate(
        nand::nand_gate(input_a, input_b),
        nand::nand_gate(input_a, input_b),
    )
}

pub fn not_gate(input_a: u8) -> u8 {
    nand::nand_gate(input_a, input_a)
}

pub fn or_gate(input_a: u8, input_b: u8) -> u8 {
    nand::nand_gate(
        nand::nand_gate(input_a, input_a),
        nand::nand_gate(input_b, input_b),
    )
}

// Xor gate
pub fn xor_gate(input_a: u8, input_b: u8) -> u8 {
    nand::nand_gate(
        nand::nand_gate(input_a, nand::nand_gate(input_a, input_b)),
        nand::nand_gate(input_b, nand::nand_gate(input_a, input_b)),
    )
}

// Mux gate
pub fn mux_gate(input_a: u8, input_b: u8, sel: u8) -> u8 {
    or_gate(and_gate(input_a, not_gate(sel)), and_gate(input_b, sel))
}

// DMux gate
pub fn dmux_gate(input: u8, sel: u8) -> (u8, u8) {
    (and_gate(input, not_gate(sel)), and_gate(input, sel))
}
// test dmux_gate
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_and_gate() {
        assert_eq!(and_gate(0, 0), 0);
        assert_eq!(and_gate(0, 1), 0);
        assert_eq!(and_gate(1, 0), 0);
        assert_eq!(and_gate(1, 1), 1);
    }
    #[test]
    fn test_not_gate() {
        assert_eq!(not_gate(0), 1);
        assert_eq!(not_gate(1), 0);
    }
    #[test]
    fn test_or_gate() {
        assert_eq!(or_gate(0, 0), 0);
        assert_eq!(or_gate(0, 1), 1);
        assert_eq!(or_gate(1, 0), 1);
        assert_eq!(or_gate(1, 1), 1);
    }
    #[test]
    fn test_xor_gate() {
        assert_eq!(xor_gate(0, 0), 0);
        assert_eq!(xor_gate(0, 1), 1);
        assert_eq!(xor_gate(1, 0), 1);
        assert_eq!(xor_gate(1, 1), 0);
    }
    #[test]
    fn test_mux_gate() {
        assert_eq!(mux_gate(0, 0, 0), 0);
        assert_eq!(mux_gate(0, 1, 0), 0);
        assert_eq!(mux_gate(1, 0, 0), 1);
        assert_eq!(mux_gate(1, 1, 0), 1);
        assert_eq!(mux_gate(0, 0, 1), 0);
        assert_eq!(mux_gate(0, 1, 1), 1);
        assert_eq!(mux_gate(1, 0, 1), 0);
        assert_eq!(mux_gate(1, 1, 1), 1);
    }
    #[test]
    fn test_dmux_gate() {
        assert_eq!(dmux_gate(0, 0), (0, 0));
        assert_eq!(dmux_gate(0, 1), (0, 0));
        assert_eq!(dmux_gate(1, 0), (1, 0));
        assert_eq!(dmux_gate(1, 1), (0, 1));
    }
}
