extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

extern crate memory;
use memory::typing::Type;

pub enum Operator {
	And,
	Or,
	Xor,
	LShift,
	RShift,
	Not,
}

pub fn not(mut insts: Vec<(&str, Vec<Operand>)>, v: Type) {
	let value = v.qi_to_operand();

	insts.push(("push", vec![value as Operand]));
	insts.push(("not", vec![]));
}

pub fn compute(insts: &mut Vec<(&str, Vec<Operand>)>, l: Type, o: Operator, r: Type) {
	let lhs = l.qi_to_operand();
	let rhs = r.qi_to_operand();

	insts.push(("push", vec![lhs as Operand]));
	insts.push(("push", vec![rhs as Operand]));

	match o {
		Operator::And => insts.push(("and", vec![])),
		Operator::Or => insts.push(("or", vec![])),
		Operator::Xor => insts.push(("xor", vec![])),
		Operator::LShift => insts.push(("lsh", vec![])),
		Operator::RShift => insts.push(("rsh", vec![])),
		_ => unreachable!()
	}
}