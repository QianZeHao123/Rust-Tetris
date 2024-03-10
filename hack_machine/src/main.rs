mod atom_component;
mod basic_gate;
mod component;

use component::alu::alu;

fn main() {
    println!("Hack computer simulator");
    let x = [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let y = [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let zx = 0;
    let nx = 0;
    let zy = 1;
    let ny = 1;
    let f = 1;
    let no = 1;
    let result = alu(x, y, zx, nx, zy, ny, f, no);
    println!("result = {:?}", result);
}
