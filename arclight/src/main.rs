mod arc_vm;
use arc_vm::*;

use std::fs;
use std::io::{
    Stdin, Stdout,
    stdin, stdout,
};


extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "qll.pest"]
pub struct QLLParser;

#[derive(Debug, PartialEq, Clone)]
pub enum StackInstruction {
    PushI,
    Push,
    Pop,
    Store,
    Load,
    Jump,
    JumpZero,
    JumpNotZero,
    PushPC,
    PopPC,
    Dup,
    DrTop,
    Over,
    DrNext,
    Add,
    Subtract,
    Multiply,
    Negative,
    ShiftLeft,
    ShiftRight,
    BWAnd,
    BWOr,
    BWXor,
    BWNot,
    Equal,
    NotEqual,
    GreaterEq,
    LessEq,
    GreaterThan,
    LessThan,
    LAnd,
    LOr,
    LNot,
    In,
    Out,
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordCall {
    Allocate,
    Store,
    Back,
    Next,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveType {
    Signed1,
    Signed2,
    Signed4,
    Signed8,
    Unsigned1,
    Unsigned2,
    Unsigned4,
    Unsigned8,
    Float2,
    Float4,
    Bool,
}


#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Call {                      // Calls to memory
        keyword: KeywordCall,
        primitive: PrimitiveType,
        value: Operand,
    },

    Instruction {               // Stack instructions
        inst: StackInstruction,
        parameter: Option<Operand>,
    },

    Label {
        name: String,
    },

    Goto {
        label: String,
    },

    GotoCall {
        label: String,
    },

    Return,

    SetWorkingAddress {
        addr: usize,
    },



    Nop(Rule),
}


fn main() {
    let machine: ArcVM<Stdin, Stdout> = ArcVM::default(stdin(), stdout());

    //todo: make this program a command-line program
    let path = std::env::args().nth(1).expect("no file to run given");

    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let stmt = parse(&unparsed_file).expect("unsuccessful parse");
    execute_program(machine, stmt);
}

pub fn parse(source: &str) -> Result<Vec<Statement>, Error<Rule>> {
    let mut stmt = Vec::new();

    let pairs = QLLParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                stmt.push(build_ast_from_expr(pair));
            },
            _ => {},
        }
    }

    Ok(stmt)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::call_keyword => {
            let mut pair = pair.into_inner();
            let keyword = pair.next().unwrap(); // Keyword
            let primitive = pair.next().unwrap(); // Type
            let value = pair.next(); // Option<Pair<Rule>> of the value
            parse_call_keyword(keyword, primitive, value)
        },
        Rule::label => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap(); // Label name
            parse_label(name)
        },
        Rule::call_stack => {
            let mut pair = pair.into_inner();
            let inst = pair.next().unwrap(); // The instruction
            parse_stack_instruction(inst, None)
        },
        Rule::call_stack_arg => {
            let mut pair = pair.into_inner();
            let inst = pair.next().unwrap(); // The instruction
            let arg = pair.next().unwrap(); // The argument
            parse_stack_instruction(inst, Some(arg))
        },
        Rule::call_stack_log => {
            let mut pair = pair.into_inner();
            let inst = pair.next().unwrap(); // The instruction
            parse_stack_instruction_logic(inst, None)
        },
        Rule::goto => {
            let mut pair = pair.into_inner();
            let label = pair.next().unwrap(); // Label name to go to
            parse_goto(label)
        },
        Rule::call => {
            let mut pair = pair.into_inner();
            let label = pair.next().unwrap(); // Label name to go to
            parse_goto_call(label)
        },
        Rule::ret => {
            Statement::Return
        },
        Rule::address => {
            let mut pair = pair.into_inner();
            let addr = pair.next().unwrap(); // Address
            parse_setwaddr(addr)
        }
        _ => { Statement::Nop(pair.as_rule()) }
    }
}

fn parse_call_keyword(
    keyword: pest::iterators::Pair<Rule>, 
    primitive: pest::iterators::Pair<Rule>,
    value: Option<pest::iterators::Pair<Rule>>,
    ) -> Statement {
    Statement::Call {
        keyword: match keyword.as_str().to_lowercase().as_str() {
            "alloc" => KeywordCall::Allocate,
            "store" => KeywordCall::Store,
            "back" => KeywordCall::Back,
            "next" => KeywordCall::Next,
            _ => panic!("Unexpected keyword: {}", keyword.as_str()),
        },
        primitive: build_prim_from_type(primitive),
        value: match value {
            Some(v) => v.as_str().parse::<Operand>().unwrap(),
            None => 0 as Operand,
        }
    }
}

fn parse_label(name: pest::iterators::Pair<Rule>) -> Statement {
    Statement::Label {
        name: name.as_str().to_string(),
    }
}

fn parse_stack_instruction(
    pair: pest::iterators::Pair<Rule>, 
    arg: Option<pest::iterators::Pair<Rule>>,
    ) -> Statement {
    Statement::Instruction {
        inst: build_inst_from_rule(pair),
        parameter: match arg {
            Some(a) => {
                match a.as_rule() {
                    Rule::number => {
                        Some(a.as_str().parse::<Operand>().unwrap())
                    },
                    _ => panic!("Unexpected parameter: {}", a.as_str()),
                }
            },
            None => None,
        }
    }
}

fn parse_stack_instruction_logic(
    pair: pest::iterators::Pair<Rule>, 
    arg: Option<pest::iterators::Pair<Rule>>,
    ) -> Statement {
    Statement::Instruction {
        inst: match pair.as_str().to_lowercase().as_str() {
            "and" => StackInstruction::LAnd,
            "or" => StackInstruction::LOr,
            "not" => StackInstruction::LNot,
            _ => panic!("Unexpected stack instruction: {}", pair.as_str()),
        },
        parameter: match arg {
            Some(a) => {
                match a.as_rule() {
                    Rule::number => {
                        Some(a.as_str().parse::<Operand>().unwrap())
                    },
                    _ => panic!("Unexpected parameter: {}", a.as_str()),
                }
            },
            None => None,
        }
    }
}

fn parse_goto(pair: pest::iterators::Pair<Rule>) -> Statement {
    Statement::Goto {
        label: pair.as_str().to_string()
    }
}

fn parse_goto_call(pair: pest::iterators::Pair<Rule>) -> Statement {
    Statement::GotoCall {
        label: pair.as_str().to_string()
    }
}

fn parse_setwaddr(pair: pest::iterators::Pair<Rule>) -> Statement {
    Statement::SetWorkingAddress {
        addr: pair.as_str().parse::<usize>().unwrap(),
    }
}

//      -- Variable assignment utilities --
fn remove_predicate_from_name(name: String) -> String {
    let mut s = name;
    s.retain(|c| !r#"%$"#.contains(c));
    s
}

//      -- Other tools --
fn build_prim_from_type(primitive: pest::iterators::Pair<Rule>) -> PrimitiveType {
    match primitive.as_str() {
        "int" => PrimitiveType::Signed4,
        "bool" => PrimitiveType::Bool,
        "float" => PrimitiveType::Float2,
        "double" => PrimitiveType::Float4,
        "byte" => PrimitiveType::Unsigned1,
        "word" => PrimitiveType::Unsigned2,
        "long" => PrimitiveType::Unsigned4,
        _ => panic!("Unexpected primitive type: {}", primitive.as_str()),
    }
}

fn build_inst_from_rule(inst: pest::iterators::Pair<Rule>) -> StackInstruction {
    match remove_predicate_from_name(inst.as_str().to_string())
        .to_lowercase().as_str() {
        "pushi" => StackInstruction::PushI,
        "push" => StackInstruction::Push,
        "pop" => StackInstruction::Pop,
        "store" => StackInstruction::Store,
        "load" => StackInstruction::Load,
        "jump" => StackInstruction::Jump,
        "jmpz" => StackInstruction::JumpZero,
        "jmpnz" => StackInstruction::JumpNotZero,
        "pushpc" => StackInstruction::PushPC,
        "poppc" => StackInstruction::PopPC,
        "dup" => StackInstruction::Dup,
        "drop" => StackInstruction::DrTop,
        "over" => StackInstruction::Over,
        "dropnext" => StackInstruction::DrNext,
        "add" => StackInstruction::Add,
        "sub" => StackInstruction::Subtract,
        "mul" => StackInstruction::Multiply,
        "neg" => StackInstruction::Negative,
        "shl" => StackInstruction::ShiftLeft,
        "shr" => StackInstruction::ShiftRight,
        "and" => StackInstruction::BWAnd,
        "or" => StackInstruction::BWOr,
        "xor" => StackInstruction::BWXor,
        "not" => StackInstruction:: BWNot,
        "equal" => StackInstruction::Equal,
        "notequal" => StackInstruction::NotEqual,
        "greq" => StackInstruction::GreaterEq,
        "leeq" => StackInstruction::LessEq,
        "grtr" => StackInstruction::GreaterThan,
        "less" => StackInstruction::LessThan,
        "in" => StackInstruction::In,
        "out" => StackInstruction::Out,
        _ => panic!("Unexpected stack instruction: {}", inst.as_str()),
    }
}

//      -- Execution tools --
fn counter_to_index(c: &Counter) -> usize {
    let idx_bytes: [u8; 4] = c.to_be_bytes();
    let mut idx_full_bytes: [u8; 8] = [0x00; 8];

    let mut i: usize = 0;

    while i < 4 {
        idx_full_bytes[i + 4] = idx_bytes[i];
        i += 1;
    }

    usize::from_be_bytes(idx_full_bytes)
}

fn get_bytes_from_primitive(primitive: &PrimitiveType) -> usize {
    match primitive {
        PrimitiveType::Signed1 => 1,
        PrimitiveType::Signed2 => 2,
        PrimitiveType::Signed4 => 4,
        PrimitiveType::Signed8 => 8,
        PrimitiveType::Unsigned1 => 1,
        PrimitiveType::Unsigned2 => 2,
        PrimitiveType::Unsigned4 => 4,
        PrimitiveType::Unsigned8 => 8,
        PrimitiveType::Float2 => 2,
        PrimitiveType::Float4 => 4,
        PrimitiveType::Bool => 1,
    }
}

//      -- Executing code --

fn execute_program(mut machine: ArcVM<Stdin, Stdout>, program: Vec<Statement>) {
    machine.program_counter = match program.iter().position(
        |r| r == &(Statement::Label { name: "main".to_string() })
    ) {
        Some(idx) => idx as Counter,
        None => panic!("Program has no main function"),
    };

    
    while &machine.program_counter < &(program.iter().len() as Counter) {
        machine = execute_statement(counter_to_index(&machine.program_counter), machine, &program);
        
        machine.program_counter += 1;
    }
}

fn execute_statement(idx: usize, mut machine: ArcVM<Stdin, Stdout>, program: &Vec<Statement>) -> ArcVM<Stdin, Stdout> {
    match program.get(idx).unwrap() {
        Statement::Call { keyword, primitive, value } => {
            machine = memory_call(machine, keyword.to_owned(), primitive.to_owned(), value.to_owned());
        },
        Statement::Instruction { inst, parameter } => {
            machine = stack_instruction(machine, inst.to_owned(), parameter.to_owned());
        },
        Statement::Label { name: _ } => (),
        Statement::Goto { label } => {
            machine.program_counter = match program.iter().position(
                |r| r == &(Statement::Label { name: label.clone() })
            ) {
                Some(idx) => idx as Counter,
                None => panic!("Label {} does not exist in this program", label),
            };
        },
        Statement::GotoCall { label } => {
            machine.core.return_stack.push(machine.program_counter);
            machine.program_counter = match program.iter().position(
                |r| r == &(Statement::Label { name: label.clone() })
            ) {
                Some(idx) => idx as Counter,
                None => panic!("Label {} does not exist in this program", label),
            };
        },
        Statement::Return => {
            machine.program_counter = match machine.core.return_stack.pop() {
                Some(d) => d,
                None => program.len().try_into().unwrap(),
            };
        },
        Statement::SetWorkingAddress { addr } => {
            machine.program_memory.address_register = *addr;
        },
        Statement::Nop(_) => (),
    };

    machine
}

fn memory_call(mut machine: ArcVM<Stdin, Stdout>, keyword: KeywordCall, primitive: PrimitiveType, value: Operand) -> ArcVM<Stdin, Stdout> {
   match keyword {
        KeywordCall::Allocate => {
            while !machine.program_memory.malloc(
                machine.program_memory.address_register,
                get_bytes_from_primitive(&primitive),
            ) {
                machine.program_memory.address_register += get_bytes_from_primitive(&primitive);
            }
        },
        KeywordCall::Store => {
            machine.program_memory.mstore(
                machine.program_memory.address_register, 
                value
            );
        },
        KeywordCall::Back => {
            if machine.program_memory.heap.len() >= get_bytes_from_primitive(&primitive) {
                machine.program_memory.address_register -= get_bytes_from_primitive(&primitive);
            } else {
                ();
            }
        },
        KeywordCall::Next => {
            if machine.program_memory.heap.capacity() >= (machine.program_memory.heap.len() + get_bytes_from_primitive(&primitive)) {
                machine.program_memory.address_register += get_bytes_from_primitive(&primitive);
            } else {
                ();
            }
        },
   };

   machine
}

fn stack_instruction(mut machine: ArcVM<Stdin, Stdout>, inst: StackInstruction, parameter: Option<Operand>) -> ArcVM<Stdin, Stdout> {
    match inst {
        StackInstruction::PushI => {
            match parameter {
                Some(o) => {
                    match machine.pushi(o) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    };
                },
                None => {
                    match machine.pushi(0) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    }
                },
            };
        },
        StackInstruction::Push => {
            match parameter {
                Some(o) => {
                    match machine.push(Memory::operand_to_address(o)) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    };
                },
                None => {
                    match machine.push(machine.program_memory.address_register) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    }
                },
            };
        },
        StackInstruction::Pop => {
            match parameter {
                Some(o) => {
                    match machine.pop(Memory::operand_to_address(o)) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    }
                },
                None => {
                    match machine.pop(machine.program_memory.address_register) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    }
                },
            };
        },
        StackInstruction::Store => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.store() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Load => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.load() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Jump => {
            match parameter {
                Some(o) => {
                    match machine.jump(o) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    }
                },
                None => {
                    match machine.jump(0) {
                        Ok(_) => (),
                        Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                    }
                },
            };
        },
        StackInstruction::JumpZero => {
            match parameter {
                Some(o) => {
                    if machine.core.data_stack.get(
                        machine.core.data_stack.len() - 1
                    ) == Some(&0) {
                        match machine.jump(o) {
                            Ok(_) => (),
                            Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                        };
                    }
                },
                None => {
                    if machine.core.data_stack.get(
                        machine.core.data_stack.len() - 1
                    ) == Some(&0) {
                        match machine.jump(0) {
                            Ok(_) => (),
                            Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                        };
                    }
                },
            };
        },
        StackInstruction::JumpNotZero => {
            match parameter {
                Some(o) => {
                    if machine.core.data_stack.get(
                        machine.core.data_stack.len()
                    ) != Some(&0) {
                        match machine.jump(o) {
                            Ok(_) => (),
                            Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                        };
                    }
                },
                None => {
                    if machine.core.data_stack.get(
                        machine.core.data_stack.len()
                    ) != Some(&0) {
                        match machine.jump(0) {
                            Ok(_) => (),
                            Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                        };
                    }
                },
            };
        },
        StackInstruction::PushPC => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.pushpc() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::PopPC => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.poppc() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Dup => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.dup() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::DrTop => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.drop() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Over => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.over() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::DrNext => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.dnext() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Add => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.add() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Subtract => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.sub() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Multiply => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.mul() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Negative => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.neg() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::ShiftLeft => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.shl() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::ShiftRight => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.shr() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::BWAnd => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.bw_and() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::BWOr => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.bw_or() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::BWXor => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.bw_xor() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::BWNot => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.bw_not() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Equal => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.eq() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::NotEqual => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.ne() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::GreaterEq => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.ge() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::LessEq => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.le() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::GreaterThan => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.gt() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::LessThan => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.lt() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::LAnd => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.and() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::LOr => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.or() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::LNot => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.not() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::In => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.ain() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
        StackInstruction::Out => {
            match parameter {
                Some(_) => {
                    panic!("Error at {}: instruction expects no parameters", machine.program_counter + 1);
                },
                None => match machine.aout() {
                    Ok(_) => (),
                    Err(e) => panic!("Error at {}: {:?}", machine.program_counter + 1, e),
                },
            };
        },
    };

    machine
}
