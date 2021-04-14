extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

fn main() {
	let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();
	// TODO: test ops 12-21
	insts.push(("push", vec![8 as Operand]));
	insts.push(("push", vec![9 as Operand]));
	insts.push(("call", vec![42 as Operand]));
	insts.push(("push", vec![8 as Operand]));
	insts.push(("push", vec![9 as Operand]));
	insts.push(("label", vec![42 as Operand]));
	insts.push(("add", vec![])); 
	insts.push(("return", vec![]));

	run_machine_from_ext(insts);
}
