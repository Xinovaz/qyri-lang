use crate::parsing::AstNode;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub struct QyriEnvironment {
	generic_types: Vec<String>,
	variables: HashMap<String, VariableData>,
	abstractions: HashMap<String, AbstractionData>,
	code: Vec<QyriInstruction>,
}

#[derive(Debug)]
struct VariableData {
	t: QType,
	size: usize,
	addr: usize,
}

#[derive(Debug)]
struct AbstractionData {
	/* structures */
	field_types: Vec<QType>,
	field_sizes: Vec<usize>,
	/* enumerations */
	size: usize,
	/* shared */
	addr: usize,
	names: Vec<usize>,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum QType {
	_byte(u8),
	_word(u16),
	_long(u32),
	_int(i32),
	_float(f32),
	_double(f64),
	_bool(bool),

	_string(String),

	_null,
	_type(Box<QType>),

	Object(Box<Canonical>),
}

#[derive(Debug)]
pub enum Canonical {
	Term(QType),
	Identifier(String),

	Structure(
		String /* name */,
		Vec<Canonical> /* fields */,
		QyriEnvironment /* implementation */
	),
	Enumeration(
		String /* name */,
		Vec<String> /* variants */,
		QyriEnvironment /* implementation */
	),
	Closure(
		QType /* return type */, 
		QyriEnvironment /* code */
	),
}

#[derive(Debug)]
pub enum QyriInstruction {
	DeclareVariable(String, Box<QyriInstruction>),
	DeclareConstant(String, Box<QyriInstruction>),

	DeclareFunction(String, Box<QyriInstruction>, bool /* no infix */),

	DeclareStructure(
		String /* name */,
		Vec<Box<QyriInstruction>> /* fields */,
		Vec<String> /* generic types */
	),
	DeclareEnumeration(String, Vec<String> /* variants */),

	ImplementAbstract(String, QyriEnvironment),

	CallFunction(String, Vec<Box<QyriInstruction>>),

	PushCanonical(Canonical),
	DebugNop,
}

fn bounded(number: i64, lower: i64, upper: i64) -> bool {
	lower <= number && number <= upper
}

fn reduce_to_byte(n: i64) -> u8 {
	n.to_be_bytes()[7]
}

fn reduce_to_word(n: i64) -> u16 {
	let slice: &[u8] = &(n.to_be_bytes()[6..8]);
	let converted: [u8; 2] = slice.try_into().unwrap();
	u16::from_be_bytes(converted)
}

fn reduce_to_long(n: i64) -> u32 {
	let slice: &[u8] = &(n.to_be_bytes()[4..8]);
	let converted: [u8; 4] = slice.try_into().unwrap();
	u32::from_be_bytes(converted)
}

fn reduce_to_int(n: i64) -> i32 {
	let slice: &[u8] = &(n.to_be_bytes()[4..8]);
	let converted: [u8; 4] = slice.try_into().unwrap();
	let mut num = i32::from_be_bytes(converted);
	if n.to_be_bytes()[0] == 8 {
		num = num * -1;
	}
	num
}

fn reduce_to_float(n: f64) -> f32 {
	let slice: &[u8] = &(n.to_be_bytes()[4..8]);
	let converted: [u8; 4] = slice.try_into().unwrap();
	let mut num = f32::from_be_bytes(converted);
	if n.to_be_bytes()[0] == 8 {
		num = num * -1.0;
	}
	num
}

pub fn walk_ast(tree: Vec<AstNode>) -> Vec<QyriInstruction> {
	/**/
	let mut program = Vec::new();
	for node in tree {
		match node {
			AstNode::Integer { value } => {
				let mut t: QType = QType::_null;
				if bounded(value, u8::MIN.into(), u8::MAX.into()) {
					t = QType::_byte(
						reduce_to_byte(value)
					);
				} else if bounded(value, u16::MIN.into(), u16::MAX.into()) {
					t = QType::_word(
						reduce_to_word(value)
					);
				} else if bounded(value, u32::MIN.into(), u32::MAX.into()) {
					t = QType::_long(
						reduce_to_long(value)
					);
				} else if bounded(value, i32::MIN.into(), i32::MAX.into()) {
					t = QType::_int(
						reduce_to_int(value)
					);
				} else {
					// uh oh! qyri doesn't store values that big! TODO
				}
				program.push(QyriInstruction::PushCanonical(Canonical::Term(t)));
			},
			AstNode::Float { value } => {
				// i'll size down the floats later, for now they're all doubles
				program.push(
					QyriInstruction::PushCanonical(
						Canonical::Term(
							QType::_double(value)
						)
					)
				);
			},
			AstNode::Identifier { name } => {
				program.push(
					QyriInstruction::PushCanonical(
						Canonical::Identifier(name)
					)
				);
			},
			AstNode::LongString { contents } => {
				program.push(
					QyriInstruction::PushCanonical(
						Canonical::Term(
							QType::_string(contents)
						)
					)
				);
			},
			AstNode::ShortString { contents } => {
				program.push(
					QyriInstruction::PushCanonical(
						Canonical::Term(
							QType::_byte(contents)
						)
					)
				);
			},
			AstNode::FunctionCall { name, parameters } => {
				let mut buf: Vec<Box<QyriInstruction>> = Vec::new();
				for p in parameters {
					buf.push(Box::new(walk_ast(vec![*p]).remove(0)));
				}
				let n = match *name {
					AstNode::Identifier { name } => name,
					_ => unreachable!(),
				};
				program.push(
					QyriInstruction::CallFunction(
						n, buf
					)
				);
			},
			_ => program.push(QyriInstruction::DebugNop),
		}
	}
	program
	/*   */
}