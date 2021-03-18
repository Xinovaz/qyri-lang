use std::io::prelude::*;
use std::io::{self, Read};
use std::convert::TryFrom;
use std::str;
use stack_vm::{Instruction,
			InstructionTable, 
			Machine,
			Builder,
			WriteManyTable,
			Code,
}; // Generic stack-based VM. Thanks, Jimsy!
pub type Operand = u32; // Operands are u32


fn op_to_word(op: Operand) -> u8 {
	u8::try_from(op).ok().unwrap()
}


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

// Arithmetic

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

// Standard I/O

fn read(machine: &mut Machine<Operand>, _args: &[usize]) { // pushes 1 i32
	let mut buffer = String::new();
	io::stdin()
		.read_line(&mut buffer)
		.expect("failed to read from stdin");
	match buffer.trim().parse::<u32>() {
		Ok(num) => machine.operand_push(num as Operand),
		Err(e) => panic!("failed to read operand: {}", e),
	}
}

fn long_read(machine: &mut Machine<Operand>, _args: &[usize]) { // pushes str as i32s
	let mut buffer = String::new();
	io::stdin()
		.read_line(&mut buffer)
		.expect("failed to read from stdin");
	let bytes = buffer.as_bytes();
	for byte in bytes {
		machine.operand_push(*byte as Operand);
	}
}

fn write(machine: &mut Machine<Operand>, _args: &[usize]) {
	let out = machine.operand_pop().clone();
	print!("{}", out.to_string());
	io::stdout().flush().expect("failed to flush stdout");
}

fn prt(machine: &mut Machine<Operand>, args: &[usize]) {
	let mut out: Vec<u8> = vec![];
	for i in 0..*machine.get_data(args[0]) + 2 {
		let ch: u8 = op_to_word(machine.operand_pop().clone());
		out.push(ch);
	}
	out.reverse();
	let f = match str::from_utf8(&out[..]) {
		Ok(v) => v,
		Err(e) => panic!("invalid UTF-8 series: {}", e),
	};
	println!("{}", f);
	io::stdout().flush().expect("failed to flush stdout")
}

// Control flow

fn call(machine: &mut Machine<Operand>, args: &[usize]) {
	let label = machine.get_data(args[0]).clone();
	let func = op_to_word(label) as char;
	machine.call(func.to_string().as_str());
}

fn ret(machine: &mut Machine<Operand>, _args: &[usize]) {
	machine.ret();
}

// }

pub fn run_machine_from_ext<'a>(inst: Vec<(&str, Vec<Operand>)>) {
	let mut instruction_table = InstructionTable::new();

	instruction_table.insert(Instruction::new(0, "push", 1, push));
	instruction_table.insert(Instruction::new(1, "pop", 0, pop));
	instruction_table.insert(Instruction::new(2, "add", 0, add));
	instruction_table.insert(Instruction::new(3, "sub", 0, sub));
	instruction_table.insert(Instruction::new(4, "mul", 0, mul));
	instruction_table.insert(Instruction::new(5, "div", 0, div));
	instruction_table.insert(Instruction::new(6, "read", 0, read));
	instruction_table.insert(Instruction::new(7, "write", 0, write));
	instruction_table.insert(Instruction::new(8, "lrd", 0, long_read));
	instruction_table.insert(Instruction::new(9, "print", 1, prt));

	let mut builder: Builder<Operand> = Builder::new(&instruction_table);

	for (instruction, args) in inst {
		if (instruction == "label") {
			let lbl = op_to_word(args[0]) as char;
			builder.label(lbl.to_string().as_str());
		} else {
			builder.push(instruction, args);
		}
	}

	let constants: WriteManyTable<Operand> = WriteManyTable::new();
	let mut machine = Machine::new(Code::from(builder), &constants, &instruction_table);
	machine.run();
}