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

fn read(machine: &mut Machine<Operand>, _args: &[usize]) { // pushes 1 u32
	let mut buffer = String::new();
	io::stdin()
		.read_line(&mut buffer)
		.expect("failed to read from stdin");
	match buffer.trim().parse::<u32>() {
		Ok(num) => machine.operand_push(num as Operand),
		Err(e) => panic!("failed to read operand: {}", e),
	}
}

fn long_read(machine: &mut Machine<Operand>, _args: &[usize]) { // pushes str as u32s
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
	println!("{}", out.to_string());
}

fn prt(machine: &mut Machine<Operand>, args: &[usize]) {
	let mut out: Vec<u8> = vec![];
	for _ in 0..*machine.get_data(args[0]) {
		let ch: u8 = op_to_word(machine.operand_pop().clone());
		out.push(ch);
	}
	out.reverse();
	let f = match str::from_utf8(&out[..]) {
		Ok(v) => v,
		Err(e) => panic!("invalid UTF-8 series: {}", e),
	};
	println!("{}", f);
}

// Control flow

fn call(machine: &mut Machine<Operand>, args: &[usize]) {
	let scope_name = machine.get_data(args[0]).clone();
	let lbl = op_to_word(scope_name) as char;
	machine.call(lbl.to_string().as_str());
}

fn ret(machine: &mut Machine<Operand>, _args: &[usize]) {
	machine.ret();
}

fn jump(machine: &mut Machine<Operand>, args: &[usize]) {
	let scope_name = machine.get_data(args[0]).clone();
	let lbl = op_to_word(scope_name) as char;
	machine.jump(lbl.to_string().as_str());
}

fn jumpz(machine: &mut Machine<Operand>, args: &[usize]) {
	let scope_name = machine.get_data(args[0]).clone();
	let lbl = op_to_word(scope_name) as char;
	let top = machine.operand_pop().clone();
	if top == 0 {
		machine.jump(lbl.to_string().as_str());
	}
}

fn jumpnz(machine: &mut Machine<Operand>, args: &[usize]) {
	let scope_name = machine.get_data(args[0]).clone();
	let lbl = op_to_word(scope_name) as char;
	let top = machine.operand_pop().clone();
	if top != 0 {
		machine.jump(lbl.to_string().as_str());
	}
}

// Bitwise ops

fn and(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs & rhs);
}

fn or(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs | rhs);
}

fn xor(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs ^ rhs);
}

fn l_shift(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs << rhs);
}

fn r_shift(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs >> rhs);
}

fn z_shift(machine: &mut Machine<Operand>, _args: &[usize]) {
	let rhs = machine.operand_pop().clone();
	let lhs = machine.operand_pop().clone();
	machine.operand_push(lhs >>> rhs);
}

fn not(machine: &mut Machine<Operand>, _args: &[usize]) {
	let top = machine.operand_pop().clone();
	machine.operand_push(!top);
}

// Advanced Control Flow

fn pushpc(machine: &mut Machine<Operand>, _args: &[usize]) {
	machine.operand_push(machine.ip as Operand);
}

fn poppc(machine: &mut Machine<Operand>, _args: &[usize]) {
	let arg = machine.operand_pop().clone();
	machine.ip = arg as usize;
}

fn nop(machine: &mut Machine<Operand>, _args: &[usize]) {
	let _ = ();
}

// Stack manipulation

fn dup(machine: &mut Machine<Operand>, _args: &[usize]) {
	let top = machine.operand_pop().clone();
	machine.operand_push(top);
	machine.operand_push(top);
}

fn over(machine: &mut Machine<Operand>, _args: &[usize]) {
	let top = machine.operand_pop().clone();
	let next = machine.operand_pop().clone();
	machine.operand_push(next);
	machine.operand_push(top);
	machine.operand_push(next);
}

fn dnext(machine: &mut Machine<Operand>, _args: &[usize]) {
	let top = machine.operand_pop().clone();
	let _ = machine.operand_pop().clone();
	machine.operand_push(top);
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
	instruction_table.insert(Instruction::new(10, "call", 1, call));
	instruction_table.insert(Instruction::new(11, "return", 0, ret));
	instruction_table.insert(Instruction::new(12, "and", 0, and));
	instruction_table.insert(Instruction::new(13, "or", 0, or));
	instruction_table.insert(Instruction::new(14, "xor", 0, xor));
	instruction_table.insert(Instruction::new(15, "lsh", 0, l_shift));
	instruction_table.insert(Instruction::new(16, "rsh", 0, r_shift));
	instruction_table.insert(Instruction::new(17, "zsh", 0, z_shift));
	instruction_table.insert(Instruction::new(18, "not", 0, not));
	instruction_table.insert(Instruction::new(19, "pushpc", 0, pushpc));
	instruction_table.insert(Instruction::new(20, "poppc", 0, poppc));
	instruction_table.insert(Instruction::new(21, "nop", 0, nop));
	instruction_table.insert(Instruction::new(22, "dup", 0, dup));
	instruction_table.insert(Instruction::new(23, "over", 0, over));
	instruction_table.insert(Instruction::new(24, "dnext", 0, dnext));
	instruction_table.insert(Instruction::new(25, "jmp", 1, jump));
	instruction_table.insert(Instruction::new(26, "jz", 1, jumpz));
	instruction_table.insert(Instruction::new(26, "jnz", 1, jumpnz));

	let mut builder: Builder<Operand> = Builder::new(&instruction_table);

	for (instruction, args) in inst {
		if instruction == "label" {
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