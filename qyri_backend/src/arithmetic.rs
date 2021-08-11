extern crate memory;
use memory::exceptions::DivideByZeroException;
use memory::typing::{Operand, Type};

pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	AddF,
	SubtractF,
	MultiplyF,
	DivideF,
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
				println!("{}", DivideByZeroException);
			} else {
				insts.push(("div", vec![]));
			}
		},
		Operator::AddF => insts.push(("addf", vec![])),
		Operator::SubtractF => insts.push(("subf", vec![])),
		Operator::MultiplyF => insts.push(("mulf", vec![])), // haha mulf
		Operator::DivideF => {
			if rhs == 0 {
				println!("{}", DivideByZeroException);
			} else {
				insts.push(("divf", vec![]));
			}
		}
	}
}