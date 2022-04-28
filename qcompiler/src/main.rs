extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parsing;
mod codegen;
use crate::parsing::basic_parsing;
use crate::codegen::{walk_ast, QyriEnvironment};

fn main() {
    let p = basic_parsing(false);
    println!("######################");
    let mut environment: QyriEnvironment = QyriEnvironment::new();
    for instruction in walk_ast(p, &mut environment) {
        println!("{:#?}", instruction);
    }
}