pub mod typing {
	pub enum types {
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
}

pub mod functions {
	pub struct Function {
		name: &str,
		arity: u8,
	}
}