extern crate memory;
use memory::typing::{Operand, Type};
use memory::typing::builtins::Exceptions::{message_dialogue, DivideByZeroException};

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
		Operator::Divide => {
			if rhs == 0 {
				message_dialogue(DivideByZeroException, insts, vec![lhs.to_string()]);
			} else {
				insts.push(("div", vec![]));
			}
		},
	}
}