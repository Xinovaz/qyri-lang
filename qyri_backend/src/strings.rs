extern crate memory;
use memory::typing::{Type, Operand};

pub fn to_instructions(insts: &mut Vec<(&str, Vec<Operand>)>, string: Type) {
	let to_convert = match string {
		Type::Str(s) => s.as_str(),
		_ => "",
	};
	let char_vec: Vec<char> = to_convert.chars().collect();
	for ch in char_vec {
		insts.push(("push", vec![ch as Operand]));
	}
}