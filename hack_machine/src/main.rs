mod atom_component;
mod basic_gate;
pub use atom_component::nand;
pub use basic_gate::basic;
fn main() {
    // test mux_gate
    println!("mux_gate(0, 1, 0) = {}", basic::mux_gate(0, 1, 0));
    println!("mux_gate(0, 1, 1) = {}", basic::mux_gate(0, 1, 1));
    println!("mux_gate(1, 0, 0) = {}", basic::mux_gate(1, 0, 0));
    println!("mux_gate(1, 0, 1) = {}", basic::mux_gate(1, 0, 1));
    // test dmux_gate
    println!("dmux_gate(0, 0) = {:?}", basic::dmux_gate(0, 0));
    println!("dmux_gate(0, 1) = {:?}", basic::dmux_gate(0, 1));
    println!("dmux_gate(1, 0) = {:?}", basic::dmux_gate(1, 0));
    println!("dmux_gate(1, 1) = {:?}", basic::dmux_gate(1, 1));
}
