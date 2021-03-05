extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

fn main() {
	let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

	insts.push(("push", vec![3 as Operand]));
	insts.push(("push", vec![4 as Operand]));
	insts.push(("mul", vec![]));

	let top = run_machine_from_ext(insts);
}