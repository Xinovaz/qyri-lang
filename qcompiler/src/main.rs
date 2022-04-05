extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parsing;
mod codegen;
use crate::parsing::basic_parsing;
use crate::codegen::walk_ast;

fn main() {
    let p = basic_parsing(false);
    println!("min: {:?}", f32::MIN);
    println!("max: {:?}", f32::MAX);
    println!("######################");
    for instruction in walk_ast(p) {
        println!("{:#?}", instruction);
    }
}