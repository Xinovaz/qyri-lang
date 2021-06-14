extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

extern crate memory;
use memory::Heap;

fn main() {
	let heap = Heap::new();
	let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

	insts.push(("nop", vec![]));

	let _ = run_machine_from_ext(insts, heap);
}
