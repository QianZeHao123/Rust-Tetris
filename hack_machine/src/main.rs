mod atom_component;
mod basic_gate;
pub use atom_component::dff;
pub use atom_component::nand;
pub use basic_gate::basic;
pub use basic_gate::combinational;
pub use basic_gate::sequential;

fn main() {
    println!("Hack computer simulator");
    // test bit_register
    let mut bit_register = sequential::BitRegister::new();
    println!("bit_register.clock(1, 1, 0) = {}", bit_register.clock(1, 1, 0));
    println!("bit_register.clock(1, 1, 1) = {}", bit_register.clock(1, 1, 1));
    println!("bit_register.clock(0, 0, 0) = {}", bit_register.clock(0, 0, 0));
    println!("bit_register.clock(0, 0, 1) = {}", bit_register.clock(0, 0, 1));
    println!("bit_register.clock(1, 0, 0) = {}", bit_register.clock(1, 0, 0));
    println!("bit_register.clock(1, 0, 1) = {}", bit_register.clock(1, 0, 1));
    println!("bit_register.clock(0, 1, 0) = {}", bit_register.clock(0, 1, 0));
    println!("bit_register.clock(0, 1, 1) = {}", bit_register.clock(0, 1, 1));
}
