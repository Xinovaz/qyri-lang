extern crate qyri_vm;
use qyri_vm::{Operand, run_machine_from_ext};

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QyriParser;

