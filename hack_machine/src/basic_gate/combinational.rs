use crate::basic_gate::basic::and_gate;

pub fn and16_gate(input_a: [u8; 16], input_b: [u8; 16]) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..16 {
        result[i] = and_gate(input_a[i], input_b[i]);
    }
    result
}