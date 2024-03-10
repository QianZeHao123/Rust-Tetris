use crate::basic_gate::add::adder;
use crate::basic_gate::basic::not_gate;
use crate::basic_gate::basic::or_gate;
use crate::basic_gate::combinational::and16_gate;
use crate::basic_gate::combinational::mux16_gate;
use crate::basic_gate::combinational::not16_gate;
use crate::basic_gate::combinational::or8way_gate;

pub fn alu(
    x: [u8; 16],
    y: [u8; 16],
    zx: u8,
    nx: u8,
    zy: u8,
    ny: u8,
    f: u8,
    no: u8,
) -> ([u8; 16], u8, u8) {
    let x_zx = mux16_gate(x, [0; 16], zx);
    let x_nx = mux16_gate(x_zx, not16_gate(x_zx), nx);
    let y_zy = mux16_gate(y, [0; 16], zy);
    let y_ny = mux16_gate(y_zy, not16_gate(y_zy), ny);
    let add_xy = adder(x_nx, y_ny);
    let and_xy = and16_gate(x_nx, y_ny);
    let fxy = mux16_gate(and_xy, add_xy, f);
    let not_fxy = mux16_gate(fxy, not16_gate(fxy), no);
    let out = not_fxy;
    let ng = out[15];
    let zr = not_gate(or_gate(
        or8way_gate([
            out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7],
        ]),
        or8way_gate([
            out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[15],
        ]),
    ));
    (out, zr, ng)
}
