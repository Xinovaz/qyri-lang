pub mod typing {
	pub enum Type {
		Int(i32),
		Bool(bool),
		Null,
		Str(&str),
		Float(f32),
		Double(f64),
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
		Type::Float(atom.parse::<f32>())
	}

	pub fn double_raw_to_qi(atom: &str) -> Type {
		Type::Double(atom.parse::<f64>())
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

pub mod identifiers {
	pub type identifier = &str;
}

pub mod functions {
	pub struct Function {
		name: identifier,
		arity: u8,
	}
}