extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
	type_name::<T>()
}

pub mod typing {
	pub enum Type {
		Int(i32),
		Bool(bool),
		Null,
		Str(&str),
		Float(u16),
		Double(u32),
		Byte(u8),
		Word(u16),
		Long(u32),
		Type,
	}

	pub fn int_raw_to_qi(atom: &str) -> Type {
		Type::Int(atom.parse::<i32>())
	}

	pub fn bool_raw_to_qi(atom: &str) -> Type {
		Type::Bool(atom.parse::<bool>())
	}

	pub fn str_raw_to_qi(atom: &str) -> Type {
		Type::Str(atom)
	}

	pub fn float_raw_to_qi(atom: &str) -> Type {
		Type::Float(atom.parse::<u16>())
	}

	pub fn double_raw_to_qi(atom: &str) -> Type {
		Type::Double(atom.parse::<u32>())
	}

	pub fn byte_raw_to_qi(atom: &str) -> Type {
		Type::Byte(atom.parse::<u8>())
	}

	pub fn word_raw_to_qi(atom: &str) -> Type {
		Type::Word(atom.parse::<u16>())
	}

	pub fn long_raw_to_qi(atom: &str) -> Type {
		Type::Long(atom.parse::<u32>())
	}
}

pub mod arithmetic {
	pub enum Operator {
		Add,
		Subtract,
		Multiply,
		Divide,
	}

	pub fn compute(mut insts: Vec<(&str, Vec<Operand>)>, l: Type, o: Operator, r: Type) {
		let lhs = match l {
			Type::Int(i) => i,
			Type::Float(f) => f,
			Type::Double(d) => d,
			Type::Byte(b) => b,
			Type::Word(w) => w,
			Type::Long(l) => l,
			_ => todo!() // TypeError
		};

		let rhs = match r {
			Type::Int(i) => i,
			Type::Float(f) => f,
			Type::Double(d) => d,
			Type::Byte(b) => b,
			Type::Word(w) => w,
			Type::Long(l) => l,
			_ => todo!() // TypeError
		};

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
}

pub mod bitwise {
	pub enum Operator {
		And,
		Or,
		Xor,
		LShift,
		RShift,
		Not,
	}

	pub fn not(mut insts: Vec<(&str, Vec<Operand>)>, v: Type) {
		let value = match v {
			Type::Int(i) => i,
			Type::Float(f) => f,
			Type::Double(d) => d,
			Type::Byte(b) => b,
			Type::Word(w) => w,
			Type::Long(l) => l,
			_ => todo!()
		};

		insts.push(("push", vec![value as Operand]));
		insts.push(("not", vec![]));
	}

	pub fn compute(mut insts: Vec<(&str, Vec<Operand>)>, l: Type, o: Operator, r: Type) {
		let lhs = match l {
			Type::Int(i) => i,
			Type::Float(f) => f,
			Type::Double(d) => d,
			Type::Byte(b) => b,
			Type::Word(w) => w,
			Type::Long(l) => l,
			_ => todo!() // TypeError
		};

		let rhs = match r {
			Type::Int(i) => i,
			Type::Float(f) => f,
			Type::Double(d) => d,
			Type::Byte(b) => b,
			Type::Word(w) => w,
			Type::Long(l) => l,
			_ => todo!() // TypeError
		};

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
}

pub mod identifiers {
	pub type identifier = &str;
}

pub mod functions {
	pub struct Function {
		name: identifier,
		arity: u8,
	}
}

pub fn compile() {
	let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

	// compiler logic goes here

	run_machine_from_ext(insts);
}