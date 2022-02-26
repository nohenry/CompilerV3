use std::fs;

use crate::evaluator::Evaluator;

mod ast;
mod lexer;
mod parser;
mod trie;
mod evaluator;
mod symbol;
mod value;

fn main() {
    let contents = fs::read_to_string("test/test.dsl").expect("Unable to read file!");
    let tokens = lexer::lex(&contents).unwrap();
    println!("{:#?}", tokens);
    let ast = parser::parse_from_tokens(&tokens).unwrap();
    println!("{:#?}", ast);

    Evaluator::evaluate_from_ast(&ast);
}
