extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_builder, instruction_table};

use stack_vm::{Builder}

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "cansm.pest"]
pub struct CANSMParser;

use std::{collections::HashMap, fs};

fn main() {
    let unparsed_file = fs::read_to_string("qprog.cansm")
    	.expect("Unable to read CANSM file.");

    let file = CANSMParser::parse(Rule::file, &unparsed_file)
    	.expect("File parsed unsuccessfully.")
    	.next().unwrap();

    // Variables are stored in a HashMap as u32
    let mut ca_env: HashMap<&str, u32> = HashMap::new();
    let mut builder: Builder<Operand> = Builder::new(&instruction_table);

    for line in file.into_inner() {
    	match line.as_rule() {
    		Rule::assgn => {
    			let mut inner_rules = line.into_inner();

    			let name: &str = inner_rules.next()
    				.unwrap().as_str();
    			let value: &str = inner_rules.next()
    				.unwrap().as_str();

    			ca_env.insert(name, value);
    		},
    		Rule::label => {
    			let mut inner_rules = line.into_inner();
    			let labeltext: &str = inner_rules.next()
    				.unwrap().as_str();

    			builder.label(labeltext);
    		},
    		Rule::statement => {
    			let mut inner_rules = line.into_inner();

    			let kw: &str = inner_rules.next()
    				.unwrap().as_str();

    			let argstr: &str = inner_rules.next()
    				.unwrap().as_str();

    			let strargs = argstr.split(" ");
    			let strargv = strargs.collect::<Vec<&str>>();

    			//TODO: Iterate through strargv and match it against the var env
    		}
    	}
    }
}
