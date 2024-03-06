// bit_register
// // test and16_gate
// let input_a = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
// let input_b = [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// let result = combinational::and16_gate(input_a, input_b);
// println!("and16_gate(a, b) = {:?}", result);
// // test dff
// let mut dff = dff::DFlipFlop::new();
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// println!("dff.clock(0, 1) = {}", dff.clock(0, 1));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// println!("dff.clock(0, 1) = {}", dff.clock(0, 1));

// // test mux_gate
// println!("mux_gate(0, 1, 0) = {}", basic::mux_gate(0, 1, 0));
// println!("mux_gate(0, 1, 1) = {}", basic::mux_gate(0, 1, 1));
// println!("mux_gate(1, 0, 0) = {}", basic::mux_gate(1, 0, 0));
// println!("mux_gate(1, 0, 1) = {}", basic::mux_gate(1, 0, 1));

// // test dmux_gate
// println!("dmux_gate(0, 0) = {:?}", basic::dmux_gate(0, 0));
// println!("dmux_gate(0, 1) = {:?}", basic::dmux_gate(0, 1));
// println!("dmux_gate(1, 0) = {:?}", basic::dmux_gate(1, 0));
// println!("dmux_gate(1, 1) = {:?}", basic::dmux_gate(1, 1));

// // test dff
// println!("----Test dff------------------------------------------------");
// let mut dff = dff::DFlipFlop::new();
// // -------------------------------------------------------------------
// println!("dff.clock(1, 0) = {}", dff.clock(1, 0));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// // -------------------------------------------------------------------
// println!("dff.clock(1, 0) = {}", dff.clock(1, 0));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// // -------------------------------------------------------------------
// println!("dff.clock(0, 0) = {}", dff.clock(0, 0));
// println!("dff.clock(0, 1) = {}", dff.clock(0, 1));
// // -------------------------------------------------------------------
// println!("dff.clock(1, 0) = {}", dff.clock(1, 0));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// // -------------------------------------------------------------------
// println!("dff.clock(1, 0) = {}", dff.clock(1, 0));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// // -------------------------------------------------------------------
// println!("dff.clock(1, 0) = {}", dff.clock(1, 0));
// println!("dff.clock(1, 1) = {}", dff.clock(1, 1));
// // -------------------------------------------------------------------
// println!("dff.clock(0, 0) = {}", dff.clock(0, 0));
// println!("dff.clock(0, 1) = {}", dff.clock(0, 1));
