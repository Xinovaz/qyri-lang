use crate::typing::{Operand, AtomType};

use crate::scopes::Scope;

#[derive(Debug, Clone)]
pub struct Function {
	pub arity: u8,
	pub code: Scope,
}