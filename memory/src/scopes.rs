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
			code: Vec::new(),
			collection: Vec::new(),
		}
	}

	pub fn code(&self) -> Vec<(&str, Vec<Operand>)> {
		let mut converted: Vec<(&str, Vec<Operand>)> = Vec::new();
		for instruction in &self.code {
			converted.push((instruction.0.as_str(), instruction.1.clone()));
		}
		converted
	}
}