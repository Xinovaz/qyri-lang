extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "qi.pest"]
pub struct QyriParser;

#[derive(Debug)]
pub enum AstNode {
    // Base nodes
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

    // Separate scopes
    FnClosure {
        parameters: Vec<Box<AstNode>>,
        t: Option<Box<AstNode>>,
        code: Vec<Box<AstNode>>,
    },
    FunctionDeclaration {
        name: String,
        computation: Box<AstNode>,
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
        _ => build_ast_from_term(pair),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
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
            AstNode::Float {
                value: pair.as_str().parse::<f64>().unwrap(),
            }
        },
        Rule::integer => {
            AstNode::Integer {
                value: pair.as_str().parse::<i64>().unwrap(),
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
            let expr = pair.next().unwrap(); // Function code
            let expr = build_ast_from_expr(expr);
            parse_declaration_function(name, expr)
        }
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

//      -- Parsing --
fn parse_declaration_function(
    name: pest::iterators::Pair<Rule>, 
    expr: AstNode
    ) -> AstNode
{
    AstNode::FunctionDeclaration {
        name: name.as_str().to_string(),
        computation: Box::new(expr),
    }
}

fn parse_closure(
    first: Option<pest::iterators::Pair<Rule>>, 
    second: Option<pest::iterators::Pair<Rule>>, 
    third: Option<pest::iterators::Pair<Rule>>
    ) -> AstNode 
{
    match first {
        Some(f) => {
            match f.as_rule() {
                Rule::informal_parameters => {
                    // First is informal parameters
                    match second {
                        Some(s) => {
                            match s.as_rule() {
                                Rule::typed_identifier => {
                                    // First is informals, second is type, third is code
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
                                    // First is informals, second is code, no type
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
                                    // First is type, second is code, no parameters
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


