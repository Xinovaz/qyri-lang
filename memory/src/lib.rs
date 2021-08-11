pub mod identifiers;
pub mod typing;
pub mod functions;
pub mod scopes;
pub mod exceptions;

use crate::identifiers::Identifier;
use crate::typing::{Type, Abstract};
use crate::exceptions::*;

pub struct Heap {
	memory: Vec<Abstract>,
	bindings: Vec<Identifier>,
	last_allocated_address: u32,
}

impl Heap {
	pub fn new() -> Heap {
		let mut heap = Heap {
			memory: vec![Abstract::Type(Type::Null); 0xFF as usize],
			bindings: Vec::new(),
			last_allocated_address: 0xFF,
		};

		/* Exceptions */
		heap.store(0xBD, Abstract::Exception(String::from(AttributeException)));
		heap.store(0xBC, Abstract::Exception(String::from(ExternalCodeException)));
		heap.store(0xBB, Abstract::Exception(String::from(RecursionException)));
		heap.store(0xBA, Abstract::Exception(String::from(MapException)));
		heap.store(0xB9, Abstract::Exception(String::from(StackOverflowException)));
		heap.store(0xB8, Abstract::Exception(String::from(IdentifierException)));
		heap.store(0xB7, Abstract::Exception(String::from(SegmentationFaultException)));
		heap.store(0xB6, Abstract::Exception(String::from(SyntaxException)));
		heap.store(0xB5, Abstract::Exception(String::from(TypeException)));
		heap.store(0xB4, Abstract::Exception(String::from(DivideByZeroException)));


		heap
	}

	pub fn allocate(&mut self) -> u32 {
		let mcopy = &self.memory;
		let first_empty_address = mcopy.into_iter()
					.position(|x| match x {Abstract::Type(Type::Null) => true, 
											_ => false});
		match first_empty_address {
			None => self.last_allocated_address = 0 as u32,
			Some(address) => self.last_allocated_address = address as u32,
		}
		self.last_allocated_address
	}

	pub fn last_allocated(&self, c: Abstract) -> u32 {
		self.last_allocated_address
		
	}

	pub fn load(&self, addr: u32) -> Abstract {
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

/* IMPORTANT */

/*
Default memory allocations are stored here:

	Exceptions:
		AttributeException: 		0xBD
		ExternalCodeException: 		0xBC
		RecursionException: 		0xBB
		MapException: 				0xBA
		StackOverflowException: 	0xB9
		IdentifierException: 		0xB8
		SegmentationFaultException: 0xB7
		SyntaxException: 			0xB6
		TypeException: 				0xB5
		DivideByZeroException: 		0xB4

*/