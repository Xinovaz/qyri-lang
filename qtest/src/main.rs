extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

fn main() {
	let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

	insts.push(("push", vec![101 as Operand]));
	insts.push(("push", vec![110 as Operand]));
	insts.push(("push", vec![116 as Operand]));
	insts.push(("push", vec![101 as Operand]));
	insts.push(("push", vec![114 as Operand]));
	insts.push(("push", vec![32 as Operand]));
	insts.push(("print", vec![6 as Operand]));
	insts.push(("read", vec![]));
	insts.push(("read", vec![]));
	insts.push(("add", vec![]));
	insts.push(("write", vec![]));

	run_machine_from_ext(insts);
}
