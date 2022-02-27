#![feature(linked_list_cursors)]

use std::{collections::LinkedList, fs};

use crate::evaluator::Evaluator;

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod symbol;
mod trie;
mod value;

fn main() {
    let contents = fs::read_to_string("test/test.dsl").expect("Unable to read file!");
    let tokens = lexer::lex(&contents).unwrap();
    println!("{:#?}", tokens);
    let mut ltokens = LinkedList::new();
    for tok in &tokens {
        ltokens.push_back(tok);
    }

    let ast = parser::parse_from_tokens(&ltokens).unwrap();
    println!("{:#?}", ast);

    Evaluator::evaluate_from_ast(&ast);
}
