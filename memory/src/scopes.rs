use crate::typing::{Operand, AtomType, Abstract};

#[derive(Debug, Clone)]
pub struct Scope {
	pub returns: AtomType,
	pub code: Vec<(String, Vec<Operand>)>,
	pub collection: Vec<Abstract>,
}

impl Scope {
	pub fn new(returns: AtomType) -> Scope {
		Scope {
			returns,
			code: vec![],
			collection: vec![],
		}
	}
}