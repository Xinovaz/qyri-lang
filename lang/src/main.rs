extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "qyri.pest"]
pub struct QyriParser;

pub enum ArithmeticOperation {
	Add, Subtract, 
	Multiply, Divide,
}

pub enum BitwiseOperation {
	AND, OR, XOR,
	LSHIFT, RSHIFT,
	NOT,
}

pub enum LogicalOperation {
	AND, OR, XOR, NOT
}

pub enum ConditionalOperation {
	EQUALS, NOT_EQUALS,
	LESS_THAN, GREATER_THAN,
	L_T_E, G_T_E,
}

fn main() {
	let unparsed_file = std::fs::read_to_string("test.qi") //temp name
		.expect("cannot read qyri file");
}