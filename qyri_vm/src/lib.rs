use stack_vm::{Instruction,
			InstructionTable, 
			Machine,
			Builder,
			WriteManyTable,
			Code
}; // Generic stack-based VM. Thanks, Jimsy!
pub type Operand = i32; // Operands are i32


/*
The following functions are QyriVM Instructions.
These are the Rust definitions of all CandleAssembly statements.

{
*/

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
	let arg = machine.get_data(args[0]).clone();
	machine.operand_push(arg);
}

fn pop(machine: &mut Machine<Operand>, _args: &[usize]) {
	machine.operand_pop();
}

fn add(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs + rhs);
}

fn sub(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs - rhs);
}

fn mul(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs * rhs);
}

fn div(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs / rhs);
}

// }

pub fn run_machine_from_builder(builder: Builder<Operand>) {
	let mut instruction_table = InstructionTable::new();
	instruction_table.insert(Instruction::new(0, "push", 1, push));
	instruction_table.insert(Instruction::new(1, "pop", 0, pop));
	instruction_table.insert(Instruction::new(2, "add", 0, add));
	instruction_table.insert(Instruction::new(3, "sub", 0, sub));
	instruction_table.insert(Instruction::new(4, "mul", 0, mul));
	instruction_table.insert(Instruction::new(5, "div", 0, div));

	let constants: WriteManyTable<Operand> = WriteManyTable::new();

	let mut machine = Machine::new(Code::from(builder), &constants, &instruction_table);
	machine.run();
}