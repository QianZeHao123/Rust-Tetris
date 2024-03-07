use crate::basic_gate::basic::and_gate;
use crate::basic_gate::basic::dmux_gate;
use crate::basic_gate::basic::mux_gate;
use crate::basic_gate::basic::not_gate;
use crate::basic_gate::basic::or_gate;

pub fn and16_gate(input_a: [u8; 16], input_b: [u8; 16]) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..16 {
        result[i] = and_gate(input_a[i], input_b[i]);
    }
    result
}

pub fn or16_gate(input_a: [u8; 16], input_b: [u8; 16]) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..16 {
        result[i] = or_gate(input_a[i], input_b[i]);
    }
    result
}

pub fn not16_gate(input: [u8; 16]) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..16 {
        result[i] = not_gate(input[i]);
    }
    result
}

pub fn or8way_gate(input: [u8; 8]) -> u8 {
    // let mut result = 0;
    let result = or_gate(
        or_gate(or_gate(input[0], input[1]), or_gate(input[2], input[3])),
        or_gate(or_gate(input[4], input[5]), or_gate(input[6], input[7])),
    );
    result
}

pub fn mux16_gate(input_a: [u8; 16], input_b: [u8; 16], sel: u8) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..16 {
        result[i] = mux_gate(input_a[i], input_b[i], sel);
    }
    result
}

pub fn mux4way16_gate(
    input_a: [u8; 16],
    input_b: [u8; 16],
    input_c: [u8; 16],
    input_d: [u8; 16],
    sel: [u8; 2],
) -> [u8; 16] {
    // let mut result = [0; 16];
    let ab = mux16_gate(input_a, input_b, sel[0]);
    let cd = mux16_gate(input_c, input_d, sel[0]);
    let result = mux16_gate(ab, cd, sel[1]);
    result
}

pub fn dmux4way_gate(input: u8, sel: [u8; 2]) -> [u8; 4] {
    let mut result = [0; 4];
    let (a, b) = dmux_gate(input, sel[0]);
    let (c, d) = dmux_gate(a, sel[1]);
    let (e, f) = dmux_gate(b, sel[1]);
    result[0] = c;
    result[1] = d;
    result[2] = e;
    result[3] = f;
    result
}

pub fn dmux8way_gate(input: u8, sel: [u8; 3]) -> [u8; 8] {
    let mut result = [0; 8];
    let (a, b) = dmux_gate(input, sel[0]);
    let (c, d) = dmux_gate(a, sel[1]);
    let (e, f) = dmux_gate(b, sel[1]);
    let (g, h) = dmux_gate(c, sel[2]);
    let (i, j) = dmux_gate(d, sel[2]);
    let (k, l) = dmux_gate(e, sel[2]);
    let (m, n) = dmux_gate(f, sel[2]);
    result[0] = g;
    result[1] = h;
    result[2] = i;
    result[3] = j;
    result[4] = k;
    result[5] = l;
    result[6] = m;
    result[7] = n;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_and16_gate() {
        assert_eq!(and16_gate([0; 16], [0; 16]), [0; 16]);
        assert_eq!(and16_gate([0; 16], [1; 16]), [0; 16]);
        assert_eq!(and16_gate([1; 16], [0; 16]), [0; 16]);
        assert_eq!(and16_gate([1; 16], [1; 16]), [1; 16]);
    }
    #[test]
    fn test_or16_gate() {
        assert_eq!(or16_gate([0; 16], [0; 16]), [0; 16]);
        assert_eq!(or16_gate([0; 16], [1; 16]), [1; 16]);
        assert_eq!(or16_gate([1; 16], [0; 16]), [1; 16]);
        assert_eq!(or16_gate([1; 16], [1; 16]), [1; 16]);
    }
    #[test]
    fn test_not16_gate() {
        assert_eq!(not16_gate([0; 16]), [1; 16]);
        assert_eq!(not16_gate([1; 16]), [0; 16]);
        assert_eq!(
            not16_gate([1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]),
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
        );
    }
    #[test]
    fn test_mux16_gate() {
        assert_eq!(
            mux16_gate([0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1], [0; 16], 1),
            [0; 16]
        );
        assert_eq!(
            mux16_gate([0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1], [0; 16], 0),
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1]
        );
    }
    #[test]
    fn test_or8_way_gate() {
        assert_eq!(or8way_gate([0; 8]), 0);
        assert_eq!(or8way_gate([0, 0, 0, 0, 0, 0, 0, 1]), 1);
    }
    #[test]
    fn test_dmux4way_gate() {
        assert_eq!(dmux4way_gate(0, [0, 0]), [0, 0, 0, 0]);
        assert_eq!(dmux4way_gate(0, [0, 1]), [0, 0, 0, 0]);
        assert_eq!(dmux4way_gate(0, [1, 0]), [0, 0, 0, 0]);
        assert_eq!(dmux4way_gate(0, [1, 1]), [0, 0, 0, 0]);
        assert_eq!(dmux4way_gate(1, [0, 0]), [1, 0, 0, 0]);
        assert_eq!(dmux4way_gate(1, [0, 1]), [0, 1, 0, 0]);
        assert_eq!(dmux4way_gate(1, [1, 0]), [0, 0, 1, 0]);
        assert_eq!(dmux4way_gate(1, [1, 1]), [0, 0, 0, 1]);
    }
    #[test]
    fn test_dmux8_way() {
        assert_eq!(dmux8way_gate(1, [0, 0, 0]), [1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(dmux8way_gate(1, [0, 0, 1]), [0, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(dmux8way_gate(1, [0, 1, 0]), [0, 0, 1, 0, 0, 0, 0, 0]);
        assert_eq!(dmux8way_gate(1, [0, 1, 1]), [0, 0, 0, 1, 0, 0, 0, 0]);
        assert_eq!(dmux8way_gate(1, [1, 1, 1]), [0, 0, 0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_mux4way16_gate(){
        assert_eq!(mux4way16_gate([0; 16], [0; 16], [0; 16], [0; 16], [0, 0]), [0; 16]);
        assert_eq!(mux4way16_gate([0; 16], [0; 16], [0; 16], [0; 16], [0, 1]), [0; 16]);
    }
}
