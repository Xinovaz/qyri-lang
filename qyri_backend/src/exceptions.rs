use crate::strings::to_instructions;

extern crate memory;
use memory::typing::{Operand, Type};

pub struct Exception(String); // TODO make constant exceptions work or smthn

impl Exception {
	pub fn message_console(&self, 
		insts: &mut Vec<(&str, Vec<Operand>)>, 
		message: String) 
	{
		to_instructions(insts, Type::Str(message));
	}

	pub fn message_dialogue(&self, 
		insts: &mut Vec<(&str, Vec<Operand>)>, 
		inserts: Vec<String>,
	) {
		let Exception(message) = self;
		for insert in inserts {
			message.as_str().replace("{}", insert.as_str());
		}
		self.message_console(insts, message.to_string());
	}
}

/* Builtin Exceptions */

pub const DivideByZeroException: Exception = Exception (
	"cannot divide {} by 0".to_string()
);