// pub use crate::basic_gate;
// pub use crate::basic_gate::add;
// pub use crate::basic_gate::basic;
// import mux
use crate::basic_gate::combinational::mux16_gate;
pub fn test_alu() {
    println!("Hello");
}

pub fn alu(x: [u8; 16], y: [u8; 16],) -> [u8; 16] {
    let result = mux16_gate(x, y, 0);
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_alu() {
        assert_eq!(alu([0; 16], [0; 16]), [0; 16]);
        assert_eq!(alu([0; 16], [1; 16]), [0; 16]);
        assert_eq!(alu([1; 16], [0; 16]), [1; 16]);
        assert_eq!(alu([1; 16], [1; 16]), [1; 16]);
    }
}