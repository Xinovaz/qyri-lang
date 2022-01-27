extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "qi.pest"]
pub struct QyriParser;

fn main() {
    //
}