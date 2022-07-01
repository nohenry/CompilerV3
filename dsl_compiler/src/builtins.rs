use std::collections::HashMap;

use dsl_lexer::{
    ast::{Expression, FunctionCall, Literal, ParseNode},
    default_range, Token, TokenKind,
};
use dsl_symbol::{Symbol, SymbolFlags, SymbolValue};
use rt_format::ParsedFormat;

fn println(args: &Vec<Expression>) -> ParseNode {
    let mut arg_iter = args.iter();
    // let mut
    if let Some(a) = arg_iter.next() {
        if let Expression::Literal(Literal::String(s, _)) = a {
            let mut args = vec![];
            while let Some(f) = arg_iter.next() {
                match f {
                    Expression::Literal(li) => {
                        args.push(li.clone());
                    }
                    _ => (),
                }
            }
            let temp = HashMap::new();

            let args = ParsedFormat::parse::<_, HashMap<String, Literal>>(s, &args, &temp).unwrap();
            let str = args.to_string();
            return ParseNode::Expression(
                Expression::FunctionCall(FunctionCall {
                    arguments: vec![Expression::Literal(Literal::String(str, default_range()))],
                    expression_to_call: Box::new(Expression::Identifier(Token {
                        token_type: TokenKind::Ident("print".into()),
                        range: default_range(),
                    })),
                    generic: None,
                    paren_tokens: default_range(),
                    range: default_range(),
                }),
                default_range(),
            );
        }
    };

    ParseNode::Empty
}

pub fn add_builtins(root: &mut Symbol) {
    root.add_child_flags(
        &"println",
        SymbolValue::Macro(Box::new(println)),
        SymbolFlags::EXPORT,
    );
}
