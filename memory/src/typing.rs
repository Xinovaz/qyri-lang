use crate::identifiers::Identifier;
use crate::functions::Function;
use crate::scopes::Scope;

pub type Operand = i32;

#[derive(Debug, Clone)]
pub enum AtomType {
	Int,
	Bool,
	Null,
	Str,
	Float,
	Double,
	Byte,
	Word,
	Long,
	Type,
}

#[derive(Debug, Clone)]
pub enum Type {
	Int(i32),
	Bool(bool),
	Null,
	Str(String),
	Float(i16),
	Double(i32),
	Byte(u8),
	Word(u16),
	Long(u32),
	Type,
}

#[derive(Debug, Clone)]
pub enum Abstract {		/* In the future, all abstractions */ // I want to
	Type(Type),			/* may derive from struct and enum */ // make it clear:
	Struct(Vec<(Identifier, Type)>),						  // "may"
	Enum(Vec<Identifier>),
	Function(Function),
	Scope(Scope),
}

impl Type {
	pub fn qi_to_operand(&self) -> Operand {
		match self {
			Type::Int(i) => *i as Operand,
			Type::Bool(bl) => {
				if *bl {
					i32::MAX as Operand
				} else {
					0 as Operand
				}
			},
			Type::Null => 0 as Operand,
			Type::Str(s) => s.as_str().chars().next().unwrap() as Operand,
			Type::Float(f) => *f as Operand,
			Type::Double(d) => *d as Operand,
			Type::Byte(b) => *b as Operand,
			Type::Word(w) => *w as Operand,
			Type::Long(l) => *l as Operand,
			Type::Type => i32::MAX as Operand,
			_ => unreachable!(),
		}
	}
}

impl Abstract {
	pub fn qi_to_operand(&self) -> Operand {
		match self {
			Abstract::Type(T) => T.qi_to_operand(),
			Abstract::Struct(_) => 0 as Operand,
			Abstract::Enum(_) => 0 as Operand,
			Abstract::Function(_) => 0 as Operand,
			Abstract::Scope(_) => 0 as Operand,
		}
	}
}

pub fn int_raw_to_qi(atom: &str) -> Type {
	Type::Int(atom.parse::<i32>().unwrap())
}

pub fn bool_raw_to_qi(atom: &str) -> Type {
	Type::Bool(atom.parse::<bool>().unwrap())
}

pub fn str_raw_to_qi(atom: &str) -> Type {
	Type::Str(String::from(atom))
}

pub fn float_raw_to_qi(atom: &str) -> Type {
	Type::Float(atom.parse::<i16>().unwrap())
}

pub fn double_raw_to_qi(atom: &str) -> Type {
	Type::Double(atom.parse::<i32>().unwrap())
}

pub fn byte_raw_to_qi(atom: &str) -> Type {
	Type::Byte(atom.parse::<u8>().unwrap())
}

pub fn word_raw_to_qi(atom: &str) -> Type {
	Type::Word(atom.parse::<u16>().unwrap())
}

pub fn long_raw_to_qi(atom: &str) -> Type {
	Type::Long(atom.parse::<u32>().unwrap())
}