use std::fs;

mod lexer;
mod parser;
mod ast;
mod trie;

fn main() {
    let contents = fs::read_to_string("test/test.dsl").expect("Unable to read file!");
    let tokens = lexer::lex(&contents).unwrap();
    println!("{:#?}", tokens);
    let ast = parser::parse_from_tokens(&tokens).unwrap();
    println!("{:#?}", ast);
}
