mod atom_component;
mod basic_gate;
mod component;
// pub use atom_component::dff;
// pub use atom_component::nand;
// pub use basic_gate::basic;
// pub use basic_gate::combinational;
// pub use basic_gate::sequential;
// pub use basic_gate::add;
// pub use component::alu::test_alu;
// pub use component;

fn main() {
    println!("Hack computer simulator");
    component::alu::test_alu();
    // alu
    let result = component::alu::alu([1; 16], [1; 16]);
    println!("{:?}", result);
}
