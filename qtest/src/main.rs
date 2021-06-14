extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

use qyri_backend::memory::Heap;

fn main() {
	let memory = Heap::new();
	let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

	insts.push(("nop", vec![]));

	let _ = run_machine_from_ext(insts, memory);
}
