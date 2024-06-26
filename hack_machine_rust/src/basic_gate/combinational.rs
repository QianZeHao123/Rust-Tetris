use crate::basic_gate::basic::and_gate;
// use crate::basic_gate::basic::dmux_gate;
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

// pub fn or16_gate(input_a: [u8; 16], input_b: [u8; 16]) -> [u8; 16] {
//     let mut result = [0; 16];
//     for i in 0..16 {
//         result[i] = or_gate(input_a[i], input_b[i]);
//     }
//     result
// }

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

// pub fn mux4way16_gate(
//     input_a: [u8; 16],
//     input_b: [u8; 16],
//     input_c: [u8; 16],
//     input_d: [u8; 16],
//     sel: [u8; 2],
// ) -> [u8; 16] {
//     // let mut result = [0; 16];
//     let ab = mux16_gate(input_a, input_b, sel[0]);
//     let cd = mux16_gate(input_c, input_d, sel[0]);
//     let result = mux16_gate(ab, cd, sel[1]);
//     result
// }

// pub fn mux8way16_gate(
//     input_a: [u8; 16],
//     input_b: [u8; 16],
//     input_c: [u8; 16],
//     input_d: [u8; 16],
//     input_e: [u8; 16],
//     input_f: [u8; 16],
//     input_g: [u8; 16],
//     input_h: [u8; 16],
//     sel: [u8; 3],
// ) -> [u8; 16] {
//     let ab = mux16_gate(input_a, input_b, sel[0]);
//     let cd = mux16_gate(input_c, input_d, sel[0]);
//     let ef = mux16_gate(input_e, input_f, sel[0]);
//     let gh = mux16_gate(input_g, input_h, sel[0]);
//     let abcd = mux16_gate(ab, cd, sel[1]);
//     let efgh = mux16_gate(ef, gh, sel[1]);
//     let result = mux16_gate(abcd, efgh, sel[2]);
//     result
// }

// pub fn mux8way16_gate(
//     input_a: [u8; 16],
//     input_b: [u8; 16],
//     input_c: [u8; 16],
//     input_d: [u8; 16],
//     input_e: [u8; 16],
//     input_f: [u8; 16],
//     input_g: [u8; 16],
//     input_h: [u8; 16],
//     sel: [u8; 3],
// ) -> [u8; 16] {
//     // Use mux4way16_gate for the first 4 inputs, with the first 2 selection bits
//     let first_half = mux4way16_gate(
//         input_a,
//         input_b,
//         input_c,
//         input_d,
//         [sel[0], sel[1]], // Pass the first two selection bits
//     );

//     // Use mux4way16_gate for the last 4 inputs, with the first 2 selection bits
//     let second_half = mux4way16_gate(
//         input_e,
//         input_f,
//         input_g,
//         input_h,
//         [sel[0], sel[1]], // Pass the first two selection bits
//     );

//     // Use mux16_gate to choose between the outputs of the two mux4way16_gates,
//     // with the third selection bit
//     mux16_gate(first_half, second_half, sel[2])
// }

// pub fn dmux4way_gate(input: u8, sel: [u8; 2]) -> [u8; 4] {
//     let mut result = [0; 4];
//     let (a, b) = dmux_gate(input, sel[0]);
//     let (c, d) = dmux_gate(a, sel[1]);
//     let (e, f) = dmux_gate(b, sel[1]);
//     result[0] = c;
//     result[1] = d;
//     result[2] = e;
//     result[3] = f;
//     result
// }

// pub fn dmux8way_gate(input: u8, sel: [u8; 3]) -> [u8; 8] {
//     let mut result = [0; 8];
//     let (a, b) = dmux_gate(input, sel[0]);
//     let (c, d) = dmux_gate(a, sel[1]);
//     let (e, f) = dmux_gate(b, sel[1]);
//     let (g, h) = dmux_gate(c, sel[2]);
//     let (i, j) = dmux_gate(d, sel[2]);
//     let (k, l) = dmux_gate(e, sel[2]);
//     let (m, n) = dmux_gate(f, sel[2]);
//     result[0] = g;
//     result[1] = h;
//     result[2] = i;
//     result[3] = j;
//     result[4] = k;
//     result[5] = l;
//     result[6] = m;
//     result[7] = n;
//     result
// }
// pub fn dmux8way_gate(input: u8, sel: [u8; 3]) -> [u8; 8] {
//     // Split the input into two, using the most significant selection bit.
//     let (a, b) = dmux_gate(input, sel[0]);
//     // Use dmux4way_gate to further split each of the two parts,
//     // using the remaining two selection bits.
//     let first_half = dmux4way_gate(a, [sel[1], sel[2]]);
//     let second_half = dmux4way_gate(b, [sel[1], sel[2]]);
//     // Combine the outputs of the two dmux4way_gates into a single 8-element array.
//     [
//         first_half[0],
//         first_half[1],
//         first_half[2],
//         first_half[3],
//         second_half[0],
//         second_half[1],
//         second_half[2],
//         second_half[3],
//     ]
// }

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
    // #[test]
    // fn test_or16_gate() {
    //     assert_eq!(or16_gate([0; 16], [0; 16]), [0; 16]);
    //     assert_eq!(or16_gate([0; 16], [1; 16]), [1; 16]);
    //     assert_eq!(or16_gate([1; 16], [0; 16]), [1; 16]);
    //     assert_eq!(or16_gate([1; 16], [1; 16]), [1; 16]);
    // }
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
    // #[test]
    // fn test_dmux4way_gate() {
    //     assert_eq!(dmux4way_gate(0, [0, 0]), [0, 0, 0, 0]);
    //     assert_eq!(dmux4way_gate(0, [0, 1]), [0, 0, 0, 0]);
    //     assert_eq!(dmux4way_gate(0, [1, 0]), [0, 0, 0, 0]);
    //     assert_eq!(dmux4way_gate(0, [1, 1]), [0, 0, 0, 0]);
    //     assert_eq!(dmux4way_gate(1, [0, 0]), [1, 0, 0, 0]);
    //     assert_eq!(dmux4way_gate(1, [0, 1]), [0, 1, 0, 0]);
    //     assert_eq!(dmux4way_gate(1, [1, 0]), [0, 0, 1, 0]);
    //     assert_eq!(dmux4way_gate(1, [1, 1]), [0, 0, 0, 1]);
    // }
    // #[test]
    // fn test_dmux8_way() {
    //     assert_eq!(dmux8way_gate(1, [0, 0, 0]), [1, 0, 0, 0, 0, 0, 0, 0]);
    //     assert_eq!(dmux8way_gate(1, [0, 0, 1]), [0, 1, 0, 0, 0, 0, 0, 0]);
    //     assert_eq!(dmux8way_gate(1, [0, 1, 0]), [0, 0, 1, 0, 0, 0, 0, 0]);
    //     assert_eq!(dmux8way_gate(1, [0, 1, 1]), [0, 0, 0, 1, 0, 0, 0, 0]);
    //     assert_eq!(dmux8way_gate(1, [1, 1, 1]), [0, 0, 0, 0, 0, 0, 0, 1]);
    // }

    // #[test]
    // fn test_mux4way16_gate() {
    //     assert_eq!(
    //         mux4way16_gate([0; 16], [0; 16], [0; 16], [0; 16], [0, 0]),
    //         [0; 16]
    //     );
    //     assert_eq!(
    //         mux4way16_gate([0; 16], [0; 16], [0; 16], [0; 16], [0, 1]),
    //         [0; 16]
    //     );
    // }

    // #[test]
    // fn test_mux8way16_gate() {
    //     assert_eq!(
    //         mux8way16_gate(
    //             [1; 16],
    //             [0; 16],
    //             [0; 16],
    //             [1; 16],
    //             [0; 16],
    //             [0; 16],
    //             [0; 16],
    //             [0; 16],
    //             [0, 1, 0]
    //         ),
    //         [0; 16]
    //     );
    // }
}
