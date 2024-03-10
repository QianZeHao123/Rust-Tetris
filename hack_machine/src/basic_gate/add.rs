// implement a 16-bit adder from half adder and full adder

use crate::basic_gate::basic::and_gate;
use crate::basic_gate::basic::or_gate;
use crate::basic_gate::basic::xor_gate;

pub fn half_adder(a: u8, b: u8) -> (u8, u8) {
    let sum = xor_gate(a, b);
    let carry = and_gate(a, b);
    (sum, carry)
}

pub fn full_adder(a: u8, b: u8, carry: u8) -> (u8, u8) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, carry);
    let carry_out = or_gate(carry1, carry2);
    (sum2, carry_out)
}

pub fn adder(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let mut result = [0; 16];
    let mut carry = 0;
    for i in 0..16 {
        let (sum, carry_out) = full_adder(a[i], b[i], carry);
        result[i] = sum;
        carry = carry_out;
    }
    result
}

pub fn inc(input: [u8; 16]) -> [u8; 16] {
    let one = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let result = adder(input, one);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(0, 0), (0, 0));
        assert_eq!(half_adder(0, 1), (1, 0));
        assert_eq!(half_adder(1, 0), (1, 0));
        assert_eq!(half_adder(1, 1), (0, 1));
    }
    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(0, 0, 0), (0, 0));
        assert_eq!(full_adder(0, 0, 1), (1, 0));
        assert_eq!(full_adder(0, 1, 0), (1, 0));
        assert_eq!(full_adder(0, 1, 1), (0, 1));
        assert_eq!(full_adder(1, 0, 0), (1, 0));
        assert_eq!(full_adder(1, 0, 1), (0, 1));
        assert_eq!(full_adder(1, 1, 0), (0, 1));
        assert_eq!(full_adder(1, 1, 1), (1, 1));
    }
    #[test]
    fn test_adder() {
        assert_eq!(adder([0; 16], [0; 16]), [0; 16]);
        assert_eq!(adder([0; 16], [1; 16]), [1; 16]);
        assert_eq!(adder([1; 16], [0; 16]), [1; 16]);
        assert_eq!(
            adder([1; 16], [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            [0; 16]
        );
    }
    #[test]
    fn test_inc() {
        assert_eq!(
            inc([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(inc([1; 16]), [0; 16]);
    }
}
