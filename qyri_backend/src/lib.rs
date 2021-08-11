pub mod arithmetic;
pub mod bitwise;
pub mod strings;

#[cfg(test)]
mod tests {
	extern crate qyri_vm;
	use qyri_vm::run_machine_from_ext;

	extern crate memory;
	use memory::Heap;
	use memory::typing::{Type, Abstract, Operand, AtomType};
	use memory::identifiers::Identifier;
	use memory::functions::Function;
	use memory::scopes::Scope;

	use crate::arithmetic;
	use crate::bitwise;
	

	#[test]
	fn two_plus_two_equals_four() {
		let mut memory = Heap::new();
		let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

		arithmetic::compute(&mut insts, 
			Type::Int(2), 
			arithmetic::Operator::Add, 
			Type::Int(2)); // 2 + 2

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
		let x_alloc =  memory.allocate();
		let x = memory.bind(String::from("x"), x_alloc);
		insts.push(("ld", vec![x.address as Operand]));

		let top = run_machine_from_ext(insts, memory);
		assert_eq!(top, 5);
	}

	#[test]
	fn divide_by_zero_error() {
		let mut memory = Heap::new();
		let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

		arithmetic::compute(&mut insts, 
			Type::Int(2), 
			arithmetic::Operator::Divide, 
			Type::Int(0)); // 2 + 2

		let top = run_machine_from_ext(insts, memory);
		assert_eq!(top, 4);
	}

	/* #[test]
	fn init_simple_scope() {
		let mut memory = Heap::new();
		let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();

		/* Quick Qyri-izing:
		{			// creates a blank scope in the memory
			2 + 2;	// loads 2 and 2 into the stack and adds them for 4
		}
		5;
		*/

		let scope_e = Abstract::Scope(
			Scope::new(
				AtomType::Null
			)
		);

		match scope_e {
			Abstract::Scope(s) => let scope = s;
		}

		memory.store(memory.allocate(), scope);
		arithmetic::compute(&mut scope.code, 
			Type::Int(2), 
			arithmetic::Operator::Add, 
			Type::Int(2)); // 2 + 2 in the scope

		let scope_top = run_machine_from_ext(scope.code(), memory);
		assert_eq!(scope_top, 4); // This is the result of the scope operation

		insts.push(("push", vec![5 as Operand]));

		let top = run_machine_from_ext(insts, memory);
		assert_eq!(top, 5); // This is the singular 5 that's been pushed in
	}

	#[test]
	fn create_add_function_and_add_two_and_three() {
		let mut memory = Heap::new();
		let mut insts: Vec<(&str, Vec<Operand>)> = Vec::new();
		/* Quick Qyri-izing:
		fn add = (x, y) $ int {
			x + y;
		}
		add(2, 3);
		*/

		let function_e = Abstract::Function(
			Function {
				arity: 2,
				code: Scope::new(AtomType::Int),
			}
		);	// creates the function

		match function_e {
			Abstract::Function(f) => let function = f;
		}

		function.code.code.push(("add", vec![])); // function definition
		memory.store(memory.allocate(), function); // stores the function
		let function_i = memory.bind(String::from("add"), memory.last_allocated()); // gives it the name
		// load the numbers
		insts.push(("push", vec![2 as Operand]));
		insts.push(("push", vec![3 as Operand]));
		
		match memory.load(function_i.address) {
			Abstract::Function(f) => let func = f;
		}
		// load the function definition
		for instruction in func.code.code() {
			insts.push(instruction);
		}

		let top = run_machine_from_ext(insts, memory);
		assert_eq!(top, 5);
	}*/
}