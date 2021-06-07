extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

fn main() {
	let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

	insts.push(("nop", vec![]));

	run_machine_from_ext(insts);
}
