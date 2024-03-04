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
