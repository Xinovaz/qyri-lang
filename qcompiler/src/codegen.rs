use crate::parsing::AstNode;
use std::collections::HashMap;
use std::convert::TryInto;
use std::process;

pub fn end_with_error(message: &str) {
	println!("__--==+[  *  ]+==--__\n\n{:#?}\n\n--__==+[  *  ]+==__--", message);

	process::abort();
}

#[derive(Debug, Clone, PartialEq)]
pub struct QyriEnvironment {
	generic_types: Vec<String>,
	variables: HashMap<String, VariableData>,
	abstractions: HashMap<String, AbstractionData>,
	code: Vec<QyriInstruction>,
}

impl QyriEnvironment {
	pub fn new() -> QyriEnvironment {
		QyriEnvironment {
			generic_types: Vec::new(),
			variables: HashMap::new(),
			abstractions: HashMap::new(),
			code: Vec::new(),
		}
	}
	pub fn as_addition_operation(&self) -> QyriEnvironment {
		QyriEnvironment {
			generic_types: self.generic_types.clone(),
			variables: self.variables.clone(),
			abstractions: self.abstractions.clone(),
			code: vec![
				QyriInstruction::RegisterVariables(vec![
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				]),
				QyriInstruction::QLLPushAddition(
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				)
			],
		}
	}
	pub fn as_subtraction_operation(&self) -> QyriEnvironment {
		QyriEnvironment {
			generic_types: self.generic_types.clone(),
			variables: self.variables.clone(),
			abstractions: self.abstractions.clone(),
			code: vec![
				QyriInstruction::RegisterVariables(vec![
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				]),
				QyriInstruction::QLLPushSubtraction(
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				)
			],
		}
	}
	pub fn as_multiplication_operation(&self) -> QyriEnvironment {
		QyriEnvironment {
			generic_types: self.generic_types.clone(),
			variables: self.variables.clone(),
			abstractions: self.abstractions.clone(),
			code: vec![
				QyriInstruction::RegisterVariables(vec![
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				]),
				QyriInstruction::QLLPushMultiplication(
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				)
			],
		}
	}
	pub fn as_division_operation(&self) -> QyriEnvironment {
		QyriEnvironment {
			generic_types: self.generic_types.clone(),
			variables: self.variables.clone(),
			abstractions: self.abstractions.clone(),
			code: vec![
				QyriInstruction::RegisterVariables(vec![
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				]),
				QyriInstruction::QLLPushDivision(
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				)
			],
		}
	}
	pub fn as_modulus_operation(&self) -> QyriEnvironment {
		QyriEnvironment {
			generic_types: self.generic_types.clone(),
			variables: self.variables.clone(),
			abstractions: self.abstractions.clone(),
			code: vec![
				QyriInstruction::RegisterVariables(vec![
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				]),
				QyriInstruction::QLLPushAddition( // TODO: modulus op
					Canonical::Identifier("lhs".to_string()), 
					Canonical::Identifier("rhs".to_string())
				)
			],
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
struct VariableData {
	t: QType,
	size: usize,
	addr: usize,
}

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
		Vec<QType> /* parameters */,
		QType /* return type */, 
		QyriEnvironment /* code */
	),
}

#[derive(Debug, Clone, PartialEq)]
pub enum QyriInstruction {
	DeclareVariable(String, QType, Box<QyriInstruction>),
	DeclareConstant(String, QType, Box<QyriInstruction>),

	DeclareFunction(String, Box<QyriInstruction>, bool /* no infix */),
	RegisterVariables(Vec<Canonical>),

	DeclareStructure(
		String /* name */,
		Vec<Box<QyriInstruction>> /* fields */,
		Vec<String> /* generic types */
	),
	DeclareEnumeration(String, Vec<String> /* variants */),

	ImplementAbstract(String, QyriEnvironment),

	CallFunction(String, Vec<Box<QyriInstruction>>),

	BinaryExpression(
		Box<QyriInstruction> /* Left-hand term */,
		Canonical /* Operation (should be a function closure) */,
		Box<QyriInstruction> /* Right-hand term */,
	),

	PushCanonical(Canonical),
	DebugNop,


	QLLPushAddition(Canonical, Canonical),
	QLLPushSubtraction(Canonical, Canonical),
	QLLPushDivision(Canonical, Canonical),
	QLLPushMultiplication(Canonical, Canonical),
}

fn type_from_string(name: String) -> QType {
	match name.as_str() {
		"byte" => QType::_byte(0),
		"word" => QType::_word(0),
		"long" => QType::_long(0),
		"int" => QType::_int(0),
		"float" => QType::_float(0.0),
		"double" => QType::_double(0.0),
		"bool" => QType::_bool(false),
		"string" => QType::_string("".to_string()),
		"null" => QType::_null,
		"type" => QType::_type(Box::new(QType::_null)),
		_ => QType::Object(Box::new(Canonical::Term(QType::_null))),
	}
}

fn normalize_type(t: QType) -> QType {
	match t {
		QType::_byte(_) => QType::_byte(0),
		QType::_word(_) => QType::_word(0),
		QType::_long(_) => QType::_long(0),
		QType::_int(_) => QType::_int(0),
		QType::_float(_) => QType::_float(0.0),
		QType::_double(_) => QType::_double(0.0),
		QType::_bool(_) => QType::_bool(false),
		QType::_string(_) => QType::_string("".to_string()),
		QType::_null => QType::_null,
		QType::_type(_) => QType::_type(Box::new(QType::_null)),
		_ => QType::Object(Box::new(Canonical::Term(QType::_null))),
	}
}

fn internal_try_into_int(value: i64) -> QType {
	match u8::try_from(value) {
		Ok(result) => QType::_byte(result),
		Err(_) => match u16::try_from(value) {
			Ok(result) => QType::_word(result),
			Err(_) => match u32::try_from(value) {
				Ok(result) => QType::_long(result),
				Err(_) => match i32::try_from(value) {
					Ok(result) => QType::_int(result),
					Err(_) => QType::_null,
				},
			},
		},
	}
}

fn internal_try_into(term: QType) -> QType {
	// TODO look for implemented qyri TryInto
	QType::_null
}

fn internal_largify(term: QType, goal: QType) -> QType {
	// TODO set up for real
	match term {
		QType::_byte(n) => match goal {
			QType::_byte(_) => term,
			QType::_word(n) => QType::_word(n),
			QType::_long(n) => QType::_long(n),
			QType::_int(n) => match i32::try_from(n) {
				Ok(result) => QType::_int(result),
				Err(_) => QType::_null,
			},
			_ => QType::_null,
		},
		QType::_word(n) => match goal {
			QType::_byte(_) => match u8::try_from(n) {
				Ok(result) => {println!("{:?}", result);QType::_byte(result)},
				Err(_) => QType::_null,
			},
			QType::_word(_) => QType::_word(n),
			QType::_long(_) => QType::_long(n.into()),
			QType::_int(_) => match i32::try_from(n) {
				Ok(result) => QType::_int(result),
				Err(_) => QType::_null,
			},
			_ => QType::_null,
		},
		QType::_long(n) => match goal {
			QType::_byte(_) => match u8::try_from(n) {
				Ok(result) => QType::_byte(result),
				Err(_) => QType::_null,
			},
			QType::_word(_) => match u16::try_from(n) {
				Ok(result) => QType::_word(result),
				Err(_) => QType::_null,
			},
			QType::_long(_) => QType::_long(n),
			QType::_int(_) => match i32::try_from(n) {
				Ok(result) => QType::_int(result),
				Err(_) => QType::_null,
			},
			_ => QType::_null,
		},
		QType::_int(n) => match goal {
			QType::_byte(_) => match u8::try_from(n) {
				Ok(result) => QType::_byte(result),
				Err(_) => QType::_null,
			},
			QType::_word(_) => match u16::try_from(n) {
				Ok(result) => QType::_word(result),
				Err(_) => QType::_null,
			},
			QType::_long(_) => match u32::try_from(n) {
				Ok(result) => QType::_long(result),
				Err(_) => QType::_null,
			},
			QType::_int(_) => QType::_int(n),
			_ => QType::_null,
		},
		_ => QType::_null,
	}
}

pub fn walk_ast(tree: Vec<AstNode>, mut environment: &mut QyriEnvironment) -> Vec<QyriInstruction> {
	let operator_addition = Canonical::Closure(
		vec![QType::_int(0), QType::_int(0)],
		QType::_int(0),
		QyriEnvironment::new().as_addition_operation()
	);
	let operator_subtraction = Canonical::Closure(
		vec![QType::_int(0), QType::_int(0)],
		QType::_int(0),
		QyriEnvironment::new().as_subtraction_operation()
	);
	let operator_multiplication = Canonical::Closure(
		vec![QType::_int(0), QType::_int(0)],
		QType::_int(0),
		QyriEnvironment::new().as_multiplication_operation()
	);
	let operator_division = Canonical::Closure(
		vec![QType::_int(0), QType::_int(0)],
		QType::_int(0),
		QyriEnvironment::new().as_division_operation()
	);
	let operator_modulus = Canonical::Closure(
		vec![QType::_int(0), QType::_int(0)],
		QType::_int(0),
		QyriEnvironment::new().as_modulus_operation()
	);

	/**/
	let mut program = Vec::new();
	for node in tree {
		match node {
			AstNode::Integer { value } => {
				let t: QType = internal_try_into_int(value);
				program.push(QyriInstruction::PushCanonical(Canonical::Term(t)));
			},
			AstNode::Float { value } => {
				program.push(
					QyriInstruction::PushCanonical(
						Canonical::Term(
							QType::_double(value)
						)
					)
				);
			},
			AstNode::Identifier { name } => {
				if name == "true".to_string() {
					program.push(
						QyriInstruction::PushCanonical(
							Canonical::Term(
								QType::_bool(true)
							)
						)
					);
				} else if name == "false".to_string() {
					program.push(
						QyriInstruction::PushCanonical(
							Canonical::Term(
								QType::_bool(true)
							)
						)
					);
				} else if name == "null".to_string() {
					program.push(
						QyriInstruction::PushCanonical(
							Canonical::Term(
								QType::_null
							)
						)
					);
				} else if (
					name == "byte".to_string() 	||
					name == "word".to_string()	||
					name == "long".to_string()	||
					name == "int".to_string()	||
					name == "float".to_string()	||
					name == "double".to_string()||
					name == "bool".to_string()	||
					name == "string".to_string()
				) {
					program.push(
						QyriInstruction::PushCanonical(
							Canonical::Term(
								QType::_type(
									Box::new(type_from_string(name))
								)
							)
						)
					);
				} else {
					program.push(
						QyriInstruction::PushCanonical(
							Canonical::Identifier(name)
						)
					);
				}
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
					buf.push(Box::new(walk_ast(vec![*p], environment).remove(0)));
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
			AstNode::VariableAssignment { name, value } => {
				// TODO implement errors
				let n = match &*name {
					AstNode::Identifier { name } => name.clone(),
					AstNode::CanonicalIdentifier { name, t } => {
						match &**name {
							AstNode::Identifier { name } => name.clone(),
							_ => unreachable!(),
						}
					},
					_ => unreachable!(),
				};
				let value = Box::new(walk_ast(vec![*value], environment).remove(0));
				let implicit_type = match *value {
					QyriInstruction::PushCanonical(ref c) => match c.clone() {
						Canonical::Term(t) => t,
						_ => QType::_null,
					},
					_ => QType::_null,
				};
				let explicit_type = match *name {
					AstNode::CanonicalIdentifier { name, t } => {
						match *t {
							AstNode::Identifier { name } => type_from_string(name),
							_ => unreachable!(),
						}
					},
					_ => implicit_type.clone(),
				};
				if &implicit_type != &explicit_type {
					if normalize_type(internal_largify(implicit_type.clone(), explicit_type.clone())) != explicit_type {
						end_with_error("Code Generation: Placeholder type error");
					}
				}
				program.push(
					QyriInstruction::DeclareVariable(
						n, explicit_type, value
					)
				);
			},
			AstNode::ConstantAssignment { name, value } => {
				// TODO implement errors
				let n = match &*name {
					AstNode::Identifier { name } => name.clone(),
					AstNode::CanonicalIdentifier { name, t } => {
						match &**name {
							AstNode::Identifier { name } => name.clone(),
							_ => unreachable!(),
						}
					},
					_ => unreachable!(),
				};
				let value = Box::new(walk_ast(vec![*value], environment).remove(0));
				let implicit_type = match *value {
					QyriInstruction::PushCanonical(ref c) => match c.clone() {
						Canonical::Term(t) => t,
						_ => QType::_null,
					},
					_ => QType::_null,
				};
				let explicit_type = match *name {
					AstNode::CanonicalIdentifier { name, t } => {
						match *t {
							AstNode::Identifier { name } => type_from_string(name),
							_ => unreachable!(),
						}
					},
					_ => implicit_type.clone(),
				};
				if &implicit_type != &explicit_type {
					if normalize_type(internal_largify(implicit_type.clone(), explicit_type.clone())) != explicit_type {
						end_with_error("Code Generation: Placeholder type error");
					}
				}
				program.push(
					QyriInstruction::DeclareConstant(
						n, explicit_type, value
					)
				);
			},
			AstNode::FnClosure { parameters, t, code } => {
				let mut new_params = Vec::new();
				for parameter in parameters {
					match *parameter {
						AstNode::CanonicalIdentifier { name, t } => match *t {
							AstNode::Identifier { name } => new_params.push(type_from_string(name.clone())),
							_ => unreachable!(),
						},
						_ => unreachable!(),
					};
				}
				let new_type = match t {
					Some(b) => match *b {
						AstNode::Identifier { name } => type_from_string(name.clone()),
						_ => unreachable!(),
					},
					None => QType::_null,
				};
				let mut new_code = Vec::new();
				for line in code {
					new_code.push(*line.clone());
				}
				let mut new_env = QyriEnvironment::new();
				let full_new_code = walk_ast(new_code, &mut new_env);
				new_env.code = full_new_code;
				program.push(
					QyriInstruction::PushCanonical(
						Canonical::Closure(new_params, new_type, new_env)
					)
				);
			},
			AstNode::FunctionDeclaration { name, computation } => {
				let infix = match *name.clone() {
					AstNode::Identifier { name } => true,
					AstNode::NoInfixIdentifier { name } => match *name {
						AstNode::Identifier { name } => false,
						_ => unreachable!(),
					},
					_ => unreachable!(),
				};
				let fn_name = match *name.clone() {
					AstNode::Identifier { name } => name,
					AstNode::NoInfixIdentifier { name } => match *name {
						AstNode::Identifier { name } => name,
						_ => unreachable!(),
					},
					_ => unreachable!(),
				};
				let mut closure = walk_ast(vec![*computation], environment);
				program.push(
					QyriInstruction::DeclareFunction(fn_name, Box::new(closure.remove(0)), infix)
				);
			},
			_ => program.push(QyriInstruction::DebugNop),
		}
	}
	program
	/*   */
}