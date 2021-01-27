extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use pest::Parser;

#[derive(Parser)]
#[grammar = "qyri.pest"]
pub struct QyriParser;


#[allow(non_snake_case)]
let Env = HashMap::new();

/*

Dear reader of this code:
I know not how you got here, as of now I have not even
finished evaluating the grammar of the Qyri code.

However, you will need something to get through this code,
because it blows. So I present to you a poem.

"
Out of the night that covers me,
 Black as the pit from pole to pole,
I thank whatever gods may be
 For my unconquerable soul.

In the fell clutch of circumstance
 I have not winced nor cried aloud.
Under the bludgeonings of chance
 My head is bloody, but unbowed.

Beyond this place of wrath and tears
 Looms but the Horror of the shade,
And yet the menace of the years
 Finds and shall find me unafraid.

It matters not how strait the gate,
 How charged with punishments the scroll,
I am the master of my fate,
 I am the captain of my soul.
"

-- W.E. Henley
*/


pub enum ArithmeticOperation {
	ADD, SUB, 
	MUL, DIV,
}

pub enum BitwiseOperation {
	AND, OR, XOR,
	LSHIFT, RSHIFT,
	NOT,
}

pub enum LogicalOperation {
	AND, OR, XOR, NOT
}

pub enum ConditionalOperation {
	EQUALS, NOT_EQUALS,
	LESS_THAN, GREATER_THAN,
	L_T_E, G_T_E,
}


pub enum AstNode {
	Keyword {
		identifier: Box<AstNode>::Ident,
		params: Box<AstNode>,
	},						// parrot 'focus.qi';
	Call(Box<AstNode>) {
		identifier: Box<AstNode>::Ident,
		params: Box<AstNode>,
	},						// println('Hello, world!');
	ArithOp {
		lhs: Box<AstNode>,
		op: ArithmeticOperation,
		rhs: Box<AstNode>,
	},						// 2 + 2
	BwOp {
		lhs: Box<AstNode>,
		op: BitwiseOperation,
		rhs: Box<AstNode>,
	},						// 4 & 6
	LogOp {
		lhs: Box<AstNode>,
		op: LogicalOperation,
		rhs: Box<AstNode>,
	},						// True && False
	ConOp {
		lhs: Box<AstNode>,
		op: ConditionalOperation,
		rhs: Box<AstNode>,
	},						// 'Hello, world!' != 6
	Assg {
		lhs: String,
		rhs: String,
	},						// var x = 5;
	Integer(isize),			// 23
	Ident(String),			// to_string
	Str(String),			// "Hello, foo!"
	Array(Vec<AstNode>),	// [5, 6, "Damn, bro", datetime()]
	Float(f64),				// 12.345

	Block {
		code: Vec<AstNode>,
		env: HashMap<String, Box<AstNode>>,
	},	/* 
	{
		var list = ["Hello", "world!"];
		var list = join(list, ", ");
		println(list);
	} 
	*/
}


pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
	let mut ast = vec![]; // Abstract Syntax Tree begins here
	let pairs = QyriParser::parse(Rule::program, source)?;
	for pair in pairs {
		match pair.as_rule() {
			Rule::expr => {
				ast.push(Box::new(build_ast_from_expr(pair)));
			},
			_ => {},
		}
	}

	Ok(ast)
}


// This function IS Qyri. Do it right.
pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
	match pair.as_rule() {
		Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
		Rule::assgmtExpr => {
			let mut pair = pair.into_inner();
			let _ = pair.next().unwrap(); // var
			let name = pair.next().unwrap(); // identifier
			let _ = pair.next().unwrap(); // =
			let expr = pair.next().unwrap(); // value to bind to
			let expr = build_ast_from_expr(expr); // AST
			parse_assigment(name, expr); // assigns the value
		},
		Rule::kwExpr => {
			let mut pair = pair.into_inner();
			let keyword = pair.next().unwrap(); // keyword
			let expr = pair.next().unwrap(); // value to run
			let expr = build_ast_from_expr(expr); // AST
			parse_keyword(keyword, expr); // calls the keyword
		},
		Rule::fncallExpr => {
			let mut pair = pair.into_inner();
			let name = pair.next().unwrap(); // function
			let _ = pair.next().unwrap(); // open paren
			let params_p1 = pair.next().unwrap(); // params1
			let params_p2 = pair.next().unwrap(); // params2
			let _ = pair.next().unwrap(); // closed paren
			parse_function_call(name, params_p1, params_p2); // calls function
		},
		Rule::arithExpr => {
			let mut pair = pair.into_inner();
			let lhs = pair.next().unwrap(); // first number
			let op = pair.next().unwrap(); // operator
			let op = parse_arith_op(op); // operator as AST
			let rhs = pair.next().unwrap(); // second number
			parse_arithmetic(lhs, op, rhs); // do the math
		},
		Rule::logicExpr => {
			let mut pair = pair.into_inner();
			let lhs = pair.next().unwrap(); // first
			let op = pair.next().unwrap(); // operator
			let op = parse_log_op(op);
			let rhs = pair.next().unwrap(); // second
			parse_logic_expr(lhs, op, rhs); // do the think
		},
		Rule::bwExpr => {
			let mut pair = pair.into_inner();
			let lhs = pair.next().unwrap(); // first
			let op = pair.next().unwrap(); // operator
			let op = parse_bw_op(op);
			let rhs = pair.next().unwrap(); // second
			parse_bitw_expr(lhs, op, rhs); // do the binary
		},
		Rule::conExpr => {
			let mut pair = pair.into_inner();
			let lhs = pair.next().unwrap(); // first
			let op = pair.next().unwrap(); // operator
			let op = parse_con_op(op);
			let rhs = pair.next().unwrap(); // second
			parse_condit_expr(lhs, op, rhs); // compare
		},
		Rule::block => {
			let mut pair = pair.into_inner();
			let _ = pair.next().unwrap(); // open brace
			let code = pair.next().unwrap(); // the code
			let _ = pair.next().unwrap(); // closed brace
			parse_code_block(code); // do the code
		}, 
		Rule::string => {
			let mut pair = pair.into_inner();
			let _ = pair.next().unwrap(); // first quote
			let text = pair.next().unwrap(); // string contents
			let _ = pair.next().unwrap(); // end quote
			parse_string(text); // the string itself
		}, // TODO continue here
	}
}

/*
		Functions needed:
parse_assignment 	[ ]
parse_keyword		[ ]
parse_function_call	[ ]
parse_arithmetic	[ ]
parse_logic_expr	[ ]
parse_bitw_expr		[ ]
parse_condit_expr	[ ]
parse_code_block	[ ]
parse_string		[ ]

parse_arith_op		[ ]
parse_log_op		[ ]
parse_bw_op			[ ]
parse_con_op		[ ]
*/

fn main() {
	let unparsed_file = std::fs::read_to_string("test.qi") //temp name
		.expect("cannot read qyri file");
}