use std::{
    fmt::{self, Display},
    iter::Peekable,
};

use crate::lexer::{Keyword, Literal, Operator, Token, TokenKind};

pub struct ParseError {
    error: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl ParseError {
    fn new(error: &String) -> Self {
        ParseError {
            error: error.clone(),
        }
    }
}

#[derive(Debug)]
pub enum GrammarItem {
    Literal(Literal),
    Identifier(String),
    Operator(Operator),
    List,
    VariableDecleration {
        identifier: String,
        variable_type: Option<GrammarItem>,
        initializer: Option<GrammarItem>,
    }
}

#[derive(Debug)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

impl Display for ParseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.output(f, 0, &"".to_string(), false)
    }
}

impl ParseNode {
    pub fn is_constant(&self) -> bool {
        match self.entry {
            GrammarItem::Literal(_) => true,
            GrammarItem::Identifier(_) => false,
            GrammarItem::Operator(o) => match o {
                Operator::Mult => self.children[0].is_constant() && self.children[1].is_constant(),
                _ => false,
            },
            _ => false,
        }
    }

    fn output(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        index: u32,
        indent: &String,
        last: bool,
    ) -> std::fmt::Result {
        write!(f, "{}", indent)?;
        if index != 0 {
            write!(f, "{}", if last { "└──" } else { "├──" })?;
        }
        write!(f, "{:?}\n", self.entry)?;
        let nindent = format!(
            "{}{}",
            indent,
            if index == 0 {
                ""
            } else if last {
                "    "
            } else {
                "│   "
            }
        );
        self.children.iter().enumerate().for_each(|(i, v)| {
            ParseNode::output(v, f, index + 1, &nindent, i == self.children.len() - 1).unwrap();
        });
        Ok(())
    }
}

pub fn parse_from_tokens(tokens: &Vec<Token>) -> Result<ParseNode, ParseError> {
    let mut it = tokens.iter().peekable();
    let ast = parse_expression(&mut it, 0)?;
    println!("{:?}", ast);
    Ok(ast)
}

fn parse_statement<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    match tokens.peek() {
        Some(t) => match t.token_type {
            TokenKind::Keyword(k) => match k {
                Keyword::Let => {

                } 
            }
        }
    }
}

fn parse_expression<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    prev_prec: u8,
) -> Result<ParseNode, ParseError> {
    let mut left = if let Some(Token {
        token_type: TokenKind::Operator(o),
        ..
    }) = tokens.peek()
    {
        let uprec = unary_precedence(*o);
        if uprec != 0 && uprec >= prev_prec {
            tokens.next();
            let right = parse_expression(tokens, uprec);
            match right {
                Ok(n) => Ok(ParseNode {
                    children: vec![n],
                    entry: GrammarItem::Operator(*o),
                }),
                Err(_) => right,
            }
        } else {
            parse_expression(tokens, 0)
        }
    } else {
        parse_primary(tokens)
    };

    if tokens.peek().is_some() {
        while let Some(Token {
            token_type: TokenKind::Operator(o),
            ..
        }) = tokens.peek()
        {
            let prec = binary_precedence(*o);
            if prec <= prev_prec || prec == 0 {
                break;
            }
            tokens.next();

            let right = parse_expression(tokens, prec);
            left = Ok(ParseNode {
                children: vec![left?, right?],
                entry: GrammarItem::Operator(*o),
            });
        }
    }
    left
}

fn parse_variable_decleration<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {

}

fn parse_primary<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    match tokens.peek() {
        Some(t) => match t {
            Token {
                token_type: TokenKind::OpenParen,
                ..
            } => {
                tokens.next();
                let expr = parse_expression(tokens, 0);
                tokens.next();
                expr
            }
            _ => parse_literal(tokens),
        },
        None => parse_literal(tokens),
    }
}

fn parse_literal<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    match tokens.next() {
        Some(t) => match t {
            Token {
                token_type: TokenKind::Literal(a),
                ..
            } => Ok(ParseNode {
                children: Vec::new(),
                entry: GrammarItem::Literal(a.clone()),
            }),
            Token {
                token_type: TokenKind::Ident(a),
                ..
            } => parse_ident(tokens, a),
            _ => Err(ParseError::new(&"Unkown literal value!".to_string())),
        },
        None => Err(ParseError::new(&"Unkown literal value!".to_string())),
    }
}

fn parse_ident<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    str: &String,
) -> Result<ParseNode, ParseError> {
    Ok(ParseNode {
        children: Vec::new(),
        entry: GrammarItem::Identifier(String::from(str)),
    })
}

fn parse_paren_list<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    if let Some(Token {
        token_type: TokenKind::OpenParen,
        ..
    }) = tokens.peek()
    {
        let mut node = ParseNode {
            children: vec![],
            entry: GrammarItem::List,
        };
        tokens.next();
        while {
            if let Some(n) = tokens.peek() {
                match n {
                    Token {
                        token_type: TokenKind::OpenParen,
                        ..
                    } => {
                        tokens.next();
                        false
                    }
                    _ => {
                        node.children.push(parse_expression(tokens, 0)?);
                        match tokens.next() {
                            Some(t) => match t.token_type() {
                                TokenKind::Comma => (),
                                TokenKind::OpenParen => return Ok(node),
                                _ => return Err(ParseError::new(&String::from("Expected comma!"))),
                            },
                            _ => return Err(ParseError::new(&String::from("Expected comma!"))),
                        };
                        true
                    }
                }
            } else {
                false
            }
        } {}
        return Ok(node);
    } else {
        Err(ParseError::new(&String::from("Expected left parenthesis!")))
    }
}

fn unary_precedence(operator: Operator) -> u8 {
    match operator {
        Operator::Minus
        | Operator::LogicalNot
        | Operator::BitNot
        | Operator::Mult
        | Operator::BitAnd => 14,
        _ => 0,
    }
}

fn binary_precedence(operator: Operator) -> u8 {
    match operator {
        Operator::LogicalOr => 3,
        Operator::BitXor => 4,
        Operator::LogicalAnd => 5,
        Operator::BitOr => 6,
        Operator::BitXor => 7,
        Operator::BitAnd => 8,
        Operator::Eq | Operator::NEq => 9,
        Operator::Lt
        | Operator::LtEq
        | Operator::Gt
        | Operator::GtEq
        | Operator::NGt
        | Operator::NLt => 10,
        Operator::BitLeft | Operator::BitRight => 11,
        Operator::Plus | Operator::Minus | Operator::Percent => 12,
        Operator::Mult | Operator::Divide => 13,
        Operator::Spread | Operator::As => 14,
        Operator::Dot => 15,
        _ => 0,
    }
}
