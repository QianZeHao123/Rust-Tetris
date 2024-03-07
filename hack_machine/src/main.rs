mod atom_component;
mod basic_gate;
pub use atom_component::dff;
pub use atom_component::nand;
pub use basic_gate::basic;
pub use basic_gate::combinational;
pub use basic_gate::sequential;

fn main() {
    println!("Hack computer simulator");
}
