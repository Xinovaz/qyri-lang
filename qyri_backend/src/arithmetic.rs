extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

extern crate memory;
use memory::typing::Type;

pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
}

pub fn compute(insts: &mut Vec<(&str, Vec<Operand>)>, l: Type, o: Operator, r: Type) {
	let lhs = l.qi_to_operand();
	let rhs = r.qi_to_operand();

	insts.push(("push", vec![lhs as Operand]));
	insts.push(("push", vec![rhs as Operand]));

	match o {
		Operator::Add => insts.push(("add", vec![])),
		Operator::Subtract => insts.push(("sub", vec![])),
		Operator::Multiply => insts.push(("mul", vec![])),
		Operator::Divide => insts.push(("div", vec![])),
		_ => unreachable!()
	}
}