pub mod identifiers;
pub mod typing;
pub mod functions;
pub mod scopes;

use crate::identifiers::Identifier;
use crate::typing::{Type, Abstract};

pub struct Heap {
	memory: Vec<Abstract>,
	bindings: Vec<Identifier>,
}

impl Heap {
	pub fn new() -> Heap {
		Heap {
			memory: vec![Abstract::Type(Type::Null); 0xFF as usize],
			bindings: vec![],
		}
	}

	pub fn allocate(&self) -> u32 {
		let mcopy = &self.memory;
		let first_empty_address = mcopy.into_iter()
					.position(|x| match x {Abstract::Type(Type::Null) => true, 
											_ => false});
		match first_empty_address {
			None => 0 as u32,
			Some(address) => address as u32,
		}
	}

	pub fn last_allocated(&self, c: Abstract) -> u32 {
		if self.allocate() == 0 {
			0xFE
		} else {
			self.allocate() - 1
		}
		
	}

	pub fn load(&self, addr: i32) -> Abstract {
		self.memory[addr as usize].clone()
	}

	pub fn store(&mut self, addr: u32, value: Abstract) {
		self.memory[addr as usize] = value;
	}

	pub fn bind(&mut self, name: String, addr: u32) -> Identifier {
		let i = Identifier {
			name: name,
			address: addr,
		};
		let e = i.clone();
		self.bindings.push(e);
		return i;
	}
}