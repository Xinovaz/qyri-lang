extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;
use pest::prec_climber::{
    Assoc,
    Operator,
    PrecClimber,
};

#[derive(Parser)]
#[grammar = "qi.pest"]
pub struct QyriParser;

#[derive(Debug)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
}

#[derive(Debug)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug)]
pub enum AstNode {
    // Unit nodes
    Integer {
        value: i64,
    },
    Float {
        value: f64,
    },
    Identifier {
        name: String,
    },
    LongString {
        contents: String,
    },
    ShortString {
        contents: char,
    },
    TypedIdentifier {
        name: Box<AstNode>,
        t: Vec<Box<AstNode>>,
    },
    CanonicalIdentifier {
        name: Box<AstNode>,
        t: Box<AstNode>,
    },


    // Expressions
    FunctionCall {
        name: Box<AstNode>,
        parameters: Vec<Box<AstNode>>,
    },
    VariableAssignment {
        name: Box<AstNode>,
        value: Box<AstNode>,
    },
    ConstantAssignment {
        name: Box<AstNode>,
        value: Box<AstNode>,
    },
    VariableReassignment {
        name: Box<AstNode>,
        value: Box<AstNode>,
    },
    VariableReassignmentAdd {
        name: Box<AstNode>,
        value: Box<AstNode>,
    },
    VariableReassignmentSub {
        name: Box<AstNode>,
        value: Box<AstNode>,
    },
    VariableReassignmentMul {
        name: Box<AstNode>,
        value: Box<AstNode>,
    },
    VariableReassignmentDiv {
        name: Box<AstNode>,
        value: Box<AstNode>,
    },
    ArithmeticExpression {
        lhs: Box<AstNode>,
        op: ArithmeticOperator,
        rhs: Box<AstNode>,
    },
    BitwiseExpression {
        lhs: Box<AstNode>,
        op: BitwiseOperator,
        rhs: Box<AstNode>,
    },

    // Separate scopes
    FnClosure {
        parameters: Vec<Box<AstNode>>,
        t: Option<Box<AstNode>>,
        code: Vec<Box<AstNode>>,
    },
    FunctionDeclaration {
        name: Box<AstNode>,
        computation: Box<AstNode>,
    },
    GenericStructure {
        name: Box<AstNode>,
    },
    GenericEnumeration {
        name: Box<AstNode>,
    },
    StructureDeclaration {
        head: Box<AstNode>,
        fields: Vec<Box<AstNode>>,
    },
    EnumerationDeclaration {
        head: Box<AstNode>,
        variants: Vec<Box<AstNode>>,
    },

    // One-liner modifiers
    GenericTypeList {
        ts: Vec<Box<AstNode>>,
    },
    Decorator {
        parameters: Vec<Box<AstNode>>,
    },

    // Keyword call
    KeywordCall {
        name: Box<AstNode>,
        parameters: Vec<Box<AstNode>>,
    },

    // Return
    Return {
        expr: Box<AstNode>,
    },

    // Control Flow
    IfStatement {
        condition: Box<AstNode>,
        code: Vec<Box<AstNode>>,
    },
    ElseStatement {
        code: Vec<Box<AstNode>>,
    },
    WhileStatement {
        condition: Box<AstNode>,
        code: Vec<Box<AstNode>>,
    },
    ForStatement {
        iteration: Box<AstNode>,
        iterator: Box<AstNode>,
        code: Vec<Box<AstNode>>,
    },

    Other(Rule, String),
    DebugNop,
}


fn main() {
    let source = std::fs::read_to_string("main.qi").expect("Cannot read given Qyri file");
    match parse(&source) {
        Ok(program) => {
            println!("Parsed successfully");
            for ast_node in program {
                println!("{:#?}", ast_node);
            }
            // todo: compile
        },
        Err(_) => {
            println!("Error parsing file");
        },
    };
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = Vec::new();

    for pair in QyriParser::parse(Rule::program, &source)? {
        match pair.as_rule() {
            Rule::expr => {
                ast.push(build_ast_from_expr(pair));
            },
            Rule::separate_scope => {
                ast.push(build_ast_from_separate(pair));
            },
            Rule::call_return | Rule::call_keyword => {
                ast.push(build_ast_from_keyword_call(pair));
            },
            _ => {},
        };
    }

    Ok(ast)
}

pub fn box_parse(source: &str) -> Result<Vec<Box<AstNode>>, Error<Rule>> {
    let mut ast = Vec::new();

    for pair in QyriParser::parse(Rule::program, &source)? {
        let p = match pair.as_rule() {
            Rule::code_loud => pair.into_inner().next().unwrap(),
            _ => pair,
        };
        match p.as_rule() {
            Rule::expr => {
                ast.push(Box::new(build_ast_from_expr(p)));
            },
            Rule::separate_scope => {
                ast.push(Box::new(build_ast_from_separate(p)));
            },
            Rule::call_return | Rule::call_keyword => {
                ast.push(Box::new(build_ast_from_keyword_call(p)));
            },
            _ => {},
        };
    }

    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr | Rule::code_loud => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::closure => {
            let mut pair = pair.into_inner();
            let first = pair.next();
            let second = pair.next();
            let third = pair.next();
            parse_closure(first, second, third)
        },
        Rule::assignment_variable => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_variable_assignment(name, expr)
        },
        Rule::assignment_constant => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_constant_assignment(name, expr)
        },
        Rule::reassignment_variable => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_variable_reassignment(name, expr)
        },
        Rule::reassignment_variable_add => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_variable_reassignment_add(name, expr)
        },
        Rule::reassignment_variable_sub => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_variable_reassignment_sub(name, expr)
        },
        Rule::reassignment_variable_mul => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_variable_reassignment_mul(name, expr)
        },
        Rule::reassignment_variable_div => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_variable_reassignment_div(name, expr)
        },
        Rule::arithmetic_expression => {
            parse_arithmetic_expression(pair.into_inner())
        },
        Rule::bitwise_expression => {
            parse_bitwise_expression(pair.into_inner())
        },
        Rule::logic_expression => {
            parse_logic_expression(pair.into_inner())
        },
        _ => build_ast_from_term(pair),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::term => build_ast_from_term(pair.into_inner().next().unwrap()),
        Rule::typed_identifier => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let t = pair.next();
            let t = match t {
                Some(v) => build_vec_from_formals(v),
                None => Vec::new(),
            };
            if t.is_empty() {
                return AstNode::Identifier {
                    name: name.as_str().to_string(),
                }
            }
            AstNode::TypedIdentifier {
                name: Box::new(AstNode::Identifier { name: name.as_str().to_string() }),
                t,
            }
        },
        Rule::canonical_identifier => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let t = pair.next().unwrap();
            AstNode::CanonicalIdentifier {
                name: Box::new(build_ast_from_term(name)),
                t: Box::new(build_ast_from_term(t)),
            }
        },
        Rule::float => {
            let mut s: String = pair.as_str().to_string();
            s.retain(|c| !r#"_"#.contains(c));

            AstNode::Float {
                value: s.as_str().parse::<f64>().unwrap(),
            }
        },
        Rule::integer => {
            let mut s: String = pair.as_str().to_string();
            s.retain(|c| !r#"_"#.contains(c));

            AstNode::Integer {
                value: s.as_str().parse::<i64>().unwrap(),
            }
        },
        Rule::call_function => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            let parameters = pair.next();
            let parameters = match parameters {
                Some(p) => build_vec_from_formals(p),
                None => Vec::new(),
            };
            AstNode::FunctionCall {
                name: Box::new(build_ast_from_term(name)),
                parameters,
            }
        },
        Rule::string => {
            let mut s = pair.as_str().to_string();
            s.retain(|c| !r#"""#.contains(c));
            AstNode::LongString {
                contents: s,
            }
        },
        Rule::identifier => {
            AstNode::Identifier {
                name: pair.as_str().to_string(),
            }
        },
        _ => AstNode::Other(pair.as_rule(), pair.as_str().to_string()),
    }
}

fn build_ast_from_separate(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::separate_scope => build_ast_from_separate(pair.into_inner().next().unwrap()),
        Rule::declaration_function => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap(); // Name
            let name = build_ast_from_expr(name);
            let expr = pair.next().unwrap(); // Function code
            let expr = build_ast_from_expr(expr);
            parse_declaration_function(name, expr)
        },
        Rule::declaration_generic_structure => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            AstNode::GenericStructure {
                name: Box::new(build_ast_from_term(name)),
            }
        },
        Rule::declaration_structure => {
            let mut pair = pair.into_inner();
            let first = pair.next().unwrap();
            let first = build_ast_from_separate(first);
            let second = pair.next().unwrap();
            AstNode::StructureDeclaration {
                head: Box::new(first),
                fields: build_vec_from_structure_fields(second),
            }
        },
        Rule::declaration_generic_enumeration => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            AstNode::GenericEnumeration {
                name: Box::new(build_ast_from_term(name)),
            }
        },
        Rule::declaration_enumeration => {
            let mut pair = pair.into_inner();
            let first = pair.next().unwrap();
            let first = build_ast_from_separate(first);
            let second = pair.next().unwrap();
            AstNode::EnumerationDeclaration {
                head: Box::new(first),
                variants: build_vec_from_enumeration_variants(second),
            }
        },
        Rule::declaration_type_variables => {
            let mut pair = pair.into_inner();
            AstNode::GenericTypeList {
                ts: build_vec_from_identifiers(pair.next().unwrap()),
            }
        },
        Rule::attach_decorator => {
            let mut pair = pair.into_inner();
            AstNode::Decorator {
                parameters: build_vec_from_formals(pair.next().unwrap()),
            }
        },
        Rule::if_statement => {
            let mut pair = pair.into_inner();
            let condition = pair.next().unwrap();
            let condition = build_ast_from_expr(condition);
            let code = pair.next().unwrap();
            let code = box_parse(code.as_str()).unwrap();
            AstNode::IfStatement {
                condition: Box::new(condition),
                code,
            }
        },
        Rule::else_statement => {
            let mut pair = pair.into_inner();
            let code = pair.next().unwrap();
            let code = box_parse(code.as_str()).unwrap();
            AstNode::ElseStatement {
                code,
            }
        },
        Rule::while_loop => {
            let mut pair = pair.into_inner();
            let condition = pair.next().unwrap();
            let condition = build_ast_from_expr(condition);
            let code = pair.next().unwrap();
            let code = box_parse(code.as_str()).unwrap();
            AstNode::WhileStatement {
                condition: Box::new(condition),
                code,
            }
        },
        Rule::for_loop => {
            let mut pair = pair.into_inner();
            let iteration = pair.next().unwrap();
            let iteration = build_ast_from_expr(iteration);
            let iterator = pair.next().unwrap();
            let iterator = build_ast_from_expr(iterator);
            let code = pair.next().unwrap();
            let code = box_parse(code.as_str()).unwrap();
            AstNode::ForStatement {
                iteration: Box::new(iteration),
                iterator: Box::new(iterator),
                code,
            }
        },
        _ => AstNode::Other(pair.as_rule(), pair.as_str().to_string()),
    }
}

fn build_ast_from_keyword_call(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::call_keyword => {
            let mut pair = pair.into_inner();
            let keyword = pair.next().unwrap();
            let parameters = pair.next();
            parse_keyword_call(keyword, parameters)
        },
        Rule::call_return => {
            let mut pair = pair.into_inner();
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            AstNode::Return {
                expr: Box::new(expr),
            }
        },
        _ => AstNode::DebugNop,
    }
}

fn build_vec_from_informals(informals: pest::iterators::Pair<Rule>) -> Vec<Box<AstNode>> {
    let mut buffer: Vec<Box<AstNode>> = Vec::new();
    for p in QyriParser::parse(Rule::informal_parameters, &informals.as_str()).unwrap() {
        for pair in p.into_inner() {
            buffer.push(Box::new(build_ast_from_expr(pair)));
        }
    }
    buffer
}

fn build_vec_from_formals(formals: pest::iterators::Pair<Rule>) -> Vec<Box<AstNode>> {
    let mut buffer: Vec<Box<AstNode>> = Vec::new();
    for p in QyriParser::parse(Rule::formal_parameters, &formals.as_str()).unwrap() {
        for pair in p.into_inner() {
            buffer.push(Box::new(build_ast_from_expr(pair)));
        }
    }
    buffer
}

fn build_vec_from_structure_fields(fields: pest::iterators::Pair<Rule>) -> Vec<Box<AstNode>> {
    let mut buffer: Vec<Box<AstNode>> = Vec::new();
    for p in QyriParser::parse(Rule::structure_fields, &fields.as_str()).unwrap() {
        for pair in p.into_inner() {
            buffer.push(Box::new(build_ast_from_expr(pair)));
        }
    }
    buffer
}

fn build_vec_from_enumeration_variants(variants: pest::iterators::Pair<Rule>) -> Vec<Box<AstNode>> {
    let mut buffer: Vec<Box<AstNode>> = Vec::new();
    for p in QyriParser::parse(Rule::enumeration_variants, &variants.as_str()).unwrap() {
        for pair in p.into_inner() {
            buffer.push(Box::new(build_ast_from_expr(pair)));
        }
    }
    buffer
}

fn build_vec_from_identifiers(identifiers: pest::iterators::Pair<Rule>) -> Vec<Box<AstNode>> {
    let mut buffer: Vec<Box<AstNode>> = Vec::new();
    for p in QyriParser::parse(Rule::identifier_list, &identifiers.as_str()).unwrap() {
        for pair in p.into_inner() {
            buffer.push(Box::new(build_ast_from_expr(pair)));
        }
    }
    buffer
}

//      -- Parsing --
fn parse_declaration_function(
    name: AstNode, 
    expr: AstNode
    ) -> AstNode
{
    AstNode::FunctionDeclaration {
        name: Box::new(name),
        computation: Box::new(expr),
    }
}

fn parse_closure(
    first: Option<pest::iterators::Pair<Rule>>, 
    second: Option<pest::iterators::Pair<Rule>>, 
    third: Option<pest::iterators::Pair<Rule>>
    ) -> AstNode 
{
    /*
    Reader beware: you're in for a scare.
    */
    match first {
        Some(f) => {
            match f.as_rule() {
                Rule::informal_parameters => {
                    match second {
                        Some(s) => {
                            match s.as_rule() {
                                Rule::typed_identifier => {
                                    match third {
                                        Some(t) => {
                                            match t.as_rule() {
                                                Rule::code_loud => {
                                                    AstNode::FnClosure {
                                                        parameters: build_vec_from_informals(f),
                                                        t: Some(Box::new(build_ast_from_term(s))),
                                                        code: box_parse(t.as_str()).unwrap(),
                                                    }
                                                },
                                                _ => unreachable!(),
                                            }
                                        },
                                        None => unreachable!(),
                                    }
                                },
                                Rule::code_loud => {
                                    AstNode::FnClosure {
                                        parameters: build_vec_from_informals(f),
                                        t: None,
                                        code: box_parse(s.as_str()).unwrap(),
                                    }
                                },
                                _ => unreachable!(),
                            }
                        },
                        None => unreachable!(),
                    }
                },
                Rule::typed_identifier => {
                    match second {
                        Some(s) => {
                            match s.as_rule() {
                                Rule::code_loud => {
                                    AstNode::FnClosure {
                                        parameters: Vec::new(),
                                        t: Some(Box::new(build_ast_from_term(f))),
                                        code: box_parse(s.as_str()).unwrap(),
                                    }
                                },
                                _ => unreachable!(),
                            }
                        },
                        None => unreachable!(),
                    }
                },
                Rule::code_loud => {
                    AstNode::FnClosure {
                        parameters: Vec::new(),
                        t: None,
                        code: box_parse(f.as_str()).unwrap(),
                    }
                },
                _ => unreachable!(),
            }
        },
        None => unreachable!(),
    }
}

fn parse_keyword_call(
    pair: pest::iterators::Pair<Rule>,
    parameters: Option<pest::iterators::Pair<Rule>>
    ) -> AstNode {
    AstNode::KeywordCall {
        name: Box::new(AstNode::Identifier {
            name: pair.as_str().to_string()
        }),
        parameters: match parameters {
            Some(v) => build_vec_from_formals(v),
            None => Vec::new(),
        }
    }
}

fn parse_variable_assignment(
    pair: AstNode, 
    expr: AstNode
    ) -> AstNode {
    AstNode::VariableAssignment {
        name: Box::new(pair),
        value: Box::new(expr),
    }
}

fn parse_constant_assignment(
    pair: AstNode, 
    expr: AstNode
    ) -> AstNode {
    AstNode::ConstantAssignment {
        name: Box::new(pair),
        value: Box::new(expr),
    }
}

fn parse_variable_reassignment(
    pair: AstNode, 
    expr: AstNode
    ) -> AstNode {
    AstNode::VariableReassignment {
        name: Box::new(pair),
        value: Box::new(expr),
    }
}

fn parse_variable_reassignment_add(
    pair: AstNode, 
    expr: AstNode
    ) -> AstNode {
    AstNode::VariableReassignmentAdd {
        name: Box::new(pair),
        value: Box::new(expr),
    }
}

fn parse_variable_reassignment_sub(
    pair: AstNode, 
    expr: AstNode
    ) -> AstNode {
    AstNode::VariableReassignmentSub {
        name: Box::new(pair),
        value: Box::new(expr),
    }
}

fn parse_variable_reassignment_mul(
    pair: AstNode, 
    expr: AstNode
    ) -> AstNode {
    AstNode::VariableReassignmentMul {
        name: Box::new(pair),
        value: Box::new(expr),
    }
}

fn parse_variable_reassignment_div(
    pair: AstNode, 
    expr: AstNode
    ) -> AstNode {
    AstNode::VariableReassignmentDiv {
        name: Box::new(pair),
        value: Box::new(expr),
    }
}

fn parse_arithmetic_expression(pair: pest::iterators::Pairs<Rule>) -> AstNode {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::plus, Assoc::Left)      | 
        Operator::new(Rule::minus, Assoc::Left)     ,
        Operator::new(Rule::times, Assoc::Left)     | 
        Operator::new(Rule::divide, Assoc::Left)    ,
        Operator::new(Rule::modulus, Assoc::Right)  ,
    ]);

    let primary = |pair: pest::iterators::Pair<Rule>| {
        match pair.as_rule() {
            Rule::arithmetic_expression => parse_arithmetic_expression(pair.into_inner()),
            _ => build_ast_from_expr(pair),
        }
    };

    let infix = |lhs: AstNode, op: pest::iterators::Pair<Rule>, rhs: AstNode| {
        match op.as_rule() {
            Rule::plus => AstNode::ArithmeticExpression {
                lhs: Box::new(lhs),
                op: ArithmeticOperator::Add,
                rhs: Box::new(rhs),
            },
            Rule::minus => AstNode::ArithmeticExpression {
                lhs: Box::new(lhs),
                op: ArithmeticOperator::Subtract,
                rhs: Box::new(rhs),
            },
            Rule::times => AstNode::ArithmeticExpression {
                lhs: Box::new(lhs),
                op: ArithmeticOperator::Multiply,
                rhs: Box::new(rhs),
            },
            Rule::divide => AstNode::ArithmeticExpression {
                lhs: Box::new(lhs),
                op: ArithmeticOperator::Divide,
                rhs: Box::new(rhs),
            },
            Rule::modulus => AstNode::ArithmeticExpression {
                lhs: Box::new(lhs),
                op: ArithmeticOperator::Modulus,
                rhs: Box::new(rhs),
            },
            _ => unreachable!(),
        }
    };

    climber.climb(pair, primary, infix)
}

fn parse_bitwise_expression(pair: pest::iterators::Pairs<Rule>) -> AstNode {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::bw_and, Assoc::Left)    | 
        Operator::new(Rule::bw_or, Assoc::Left)     | 
        Operator::new(Rule::bw_xor, Assoc::Left)    ,
    ]);

    let primary = |pair: pest::iterators::Pair<Rule>| {
        match pair.as_rule() {
            Rule::bitwise_expression => parse_bitwise_expression(pair.into_inner()),
            _ => build_ast_from_expr(pair),
        }
    };

    let infix = |lhs: AstNode, op: pest::iterators::Pair<Rule>, rhs: AstNode| {
        match op.as_rule() {
            Rule::bw_and => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::And,
                rhs: Box::new(rhs),
            },
            Rule::bw_or => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::Or,
                rhs: Box::new(rhs),
            },
            Rule::bw_xor => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::Xor,
                rhs: Box::new(rhs),
            },
            _ => unreachable!(),
        }
    };

    climber.climb(pair, primary, infix)
}

fn parse_logic_expression(pair: pest::iterators::Pairs<Rule>) -> AstNode {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::log_gt, Assoc::Left)    |
        Operator::new(Rule::log_lt, Assoc::Left)    |
        Operator::new(Rule::log_ge, Assoc::Left)    |
        Operator::new(Rule::log_le, Assoc::Left)    ,
        Operator::new(Rule::log_and, Assoc::Left)   | 
        Operator::new(Rule::log_or, Assoc::Left)    | 
        Operator::new(Rule::log_xor, Assoc::Left)   ,
        Operator::new(Rule::log_eq, Assoc::Left)    |
        Operator::new(Rule::log_ne, Assoc::Left)    ,
    ]);

    let primary = |pair: pest::iterators::Pair<Rule>| {
        match pair.as_rule() {
            Rule::bitwise_expression => parse_logic_expression(pair.into_inner()),
            _ => build_ast_from_expr(pair),
        }
    };

    let infix = |lhs: AstNode, op: pest::iterators::Pair<Rule>, rhs: AstNode| {
        match op.as_rule() {
            Rule::log_and => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::And,
                rhs: Box::new(rhs),
            },
            Rule::log_or => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::Or,
                rhs: Box::new(rhs),
            },
            Rule::log_xor => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::Xor,
                rhs: Box::new(rhs),
            },
            Rule::log_eq => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::Equal,
                rhs: Box::new(rhs),
            },
            Rule::log_ne => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::NotEqual,
                rhs: Box::new(rhs),
            },
            Rule::log_gt => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::GreaterThan,
                rhs: Box::new(rhs),
            },
            Rule::log_lt => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::LessThan,
                rhs: Box::new(rhs),
            },
            Rule::log_ge => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::GreaterEqual,
                rhs: Box::new(rhs),
            },
            Rule::log_le => AstNode::BitwiseExpression {
                lhs: Box::new(lhs),
                op: BitwiseOperator::LessEqual,
                rhs: Box::new(rhs),
            },
            _ => unreachable!(),
        }
    };

    climber.climb(pair, primary, infix)
}