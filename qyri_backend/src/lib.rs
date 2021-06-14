pub mod arithmetic;
pub mod bitwise;
pub mod functions;


#[cfg(test)]
mod tests {
	extern crate qyri_vm;
	use qyri_vm::{Operand, run_machine_from_ext};

	extern crate memory;
	use memory::Heap;
	use memory::typing::{Type, Abstract};
	use memory::identifiers::Identifier;

	use crate::arithmetic;
	use crate::bitwise;
	use crate::functions;

	#[test]
	fn two_plus_two_equals_four() {
		let mut memory = Heap::new();
		let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

		arithmetic::compute(&mut insts, 
			Type::Int(2), 
			arithmetic::Operator::Add, 
			Type::Int(2)); // x = 2 + 2

		let top = run_machine_from_ext(insts, memory);
		assert_eq!(top, 4);
	}

	#[test]
	fn eighteen_or_six_equals_twenty_two() {
		let mut memory = Heap::new();
		let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

		bitwise::compute(&mut insts, 
			Type::Int(18), 
			bitwise::Operator::Or, 
			Type::Int(6));

		let top = run_machine_from_ext(insts, memory);
		assert_eq!(top, 22);
	}

	#[test]
	fn store_five_into_variable_x() {
		let mut memory = Heap::new();
		let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

		/* Quick Qyri-izing:
		var x = 5;	// This stores int 5, and binds x to its location
		x;			// This loads x into the stack
		*/

		insts.push(("push", vec![5 as Operand]));
		insts.push(("st", vec![]));
		/* The above calls Heap::allocate() immediately prior to the below.
		This is incredibly important. If a variable is not bound immediately after it
		has been allocated, it's possible for something else to allocate that space
		in the memory, which will cause huge issues. */
		let x = memory.bind(String::from("x"), memory.allocate());
		insts.push(("ld", vec![x.address as Operand]));

		let top = run_machine_from_ext(insts, memory);
		assert_eq!(top, 5);
	}
}