//! The instruction builder.
//!
//! Use this module to build code for your machine to execute.
//!
//! ## Examples
//!
//! ```
//! use stack_vm::{Instruction, InstructionTable, Builder, Machine};
//!
//! fn push(machine: &mut Machine<f64>, args: &[usize]) {
//!     let arg = machine.get_data(args[0]).clone();
//!     machine.operand_push(arg)
//! }
//!
//! let mut instruction_table: InstructionTable<f64> = InstructionTable::new();
//! instruction_table.insert(Instruction::new(0, "push", 1, push));
//!
//! let mut builder: Builder<f64> = Builder::new(&instruction_table);
//! builder.push("push", vec![1.23]);
//! ```

use crate::instruction_table::InstructionTable;
use crate::table::Table;
use crate::write_once_table::WriteOnceTable;
use std::fmt;

/// The builder struct.
///
/// Contains:
/// * an `InstructionTable`.
/// * a list of instructions that have been pushed into this builder.
/// * a `Table` of labels used for jumping.
/// * a list of `T` to be stored in the builder's data section.
pub struct Builder<'a, T: 'a + fmt::Debug + PartialEq> {
    pub instruction_table: &'a InstructionTable<T>,
    pub instructions: Vec<usize>,
    pub labels: WriteOnceTable<usize>,
    pub data: Vec<T>,
}

impl<'a, T: fmt::Debug + PartialEq> Builder<'a, T> {
    /// Create a new `Builder` from an `InstructionTable`.
    pub fn new(instruction_table: &'a InstructionTable<T>) -> Builder<T> {
        let mut labels = WriteOnceTable::new();
        labels.insert("main", 0);
        Builder {
            instruction_table: &instruction_table,
            instructions: vec![],
            labels,
            data: vec![],
        }
    }

    /// Push an instruction into the code.
    ///
    /// * `name` should match that of an instruction in the `InstructionTable`.
    /// * `args` a vector of operands to be pushed into the builder's data
    ///   section.
    pub fn push(&mut self, name: &str, args: Vec<T>) {
        let instr = self
            .instruction_table
            .by_name(name)
            .unwrap_or_else(|| panic!("Unable to find instruction with name {:?}", name));

        if args.len() != instr.arity {
            panic!(
                "Instruction {} has arity of {}, but you provided {} arguments.",
                instr.name,
                instr.arity,
                args.len()
            )
        }

        self.instructions.push(instr.op_code);
        self.instructions.push(instr.arity);
        for arg in args {
            let pos = self.push_data(arg);
            self.instructions.push(pos);
        }
    }

    /// Insert a label at this point in the code.
    ///
    /// Labels are used as targets for jumps.  When you call this method a
    /// label is stored which points to the position of the next instruction.
    pub fn label(&mut self, name: &str) {
        let idx = self.instructions.len();
        self.labels.insert(name, idx);
    }

    /// Return the length of the instructions vector.
    ///
    /// i.e. the number of instructions pushed so far.
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Return whether the Builder contains any instructions.
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    fn push_data(&mut self, data: T) -> usize {
        let pos = self.data.iter().position(|d| d == &data);
        match pos {
            Some(pos) => pos,
            None => {
                self.data.push(data);
                self.data.len() - 1
            }
        }
    }
}

impl<'a, T: 'a + fmt::Debug + PartialEq> fmt::Debug for Builder<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.data.len() {
            result.push_str(&format!("@{} = {:?}\n", i, self.data[i]));
        }

        let mut ip = 0;
        let len = self.instructions.len();
        loop {
            for label in self.labels.keys() {
                let idx = *self.labels.get(&label).unwrap();
                if idx == ip {
                    result.push_str(&format!("\n.{}:\n", label));
                }
            }

            if ip == len {
                break;
            }

            let op_code = self.instructions[ip];
            ip += 1;
            let arity = self.instructions[ip];

            let instr = self
                .instruction_table
                .by_op_code(op_code)
                .unwrap_or_else(|| panic!("Unable to find instruction with op code {}", op_code));

            result.push_str(&format!("\t{}", &instr.name));

            for _j in 0..arity {
                ip += 1;
                let const_idx = self.instructions[ip];
                result.push_str(&format!(" @{}", const_idx));
            }
            result.push('\n');

            ip += 1;
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instruction::Instruction;
    use crate::instruction_table::InstructionTable;
    use crate::machine::Machine;

    fn noop(_machine: &mut Machine<usize>, _args: &[usize]) {}

    fn example_instruction_table() -> InstructionTable<usize> {
        let mut it = InstructionTable::new();
        it.insert(Instruction::new(0, "noop", 0, noop));
        it.insert(Instruction::new(1, "push", 1, noop));
        it.insert(Instruction::new(2, "pop", 0, noop));
        it
    }

    #[test]
    fn new() {
        let it = example_instruction_table();
        let builder: Builder<usize> = Builder::new(&it);
        assert!(builder.instructions.is_empty());
    }

    #[test]
    fn push() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push("noop", vec![]);
        assert!(!builder.instructions.is_empty());
    }

    #[test]
    #[should_panic(expected = "has arity of")]
    fn push_with_incorrect_arity() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push("noop", vec![1]);
    }

    #[test]
    fn label() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push("noop", vec![]);
        builder.label("wow");
        assert_eq!(*builder.labels.get("wow").unwrap(), 2);
    }

    #[test]
    fn data_is_deduped() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push("push", vec![123]);
        builder.push("push", vec![123]);
        builder.push("push", vec![123]);
        assert_eq!(builder.data.len(), 1);
    }

    #[test]
    fn debug_format() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push("noop", vec![]);
        builder.push("push", vec![123]);
        builder.push("push", vec![456]);
        builder.label("some_function");
        builder.push("pop", vec![]);

        let actual = format!("{:?}", builder);
        let expected = "@0 = 123
@1 = 456

.main:
\tnoop
\tpush @0
\tpush @1

.some_function:
\tpop
";
        assert_eq!(actual, expected);
    }
}
