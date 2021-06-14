extern crate memory;
use memory::typing::{Type, Abstract};

pub struct Function {
	returns: Abstract,
	arity: u8,
}