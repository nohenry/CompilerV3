use std::{
    fmt::{self, Display},
    iter::Peekable,
};

use crate::{
    ast::{Expression, Literal, ParseNode, Type},
    lexer::{Keyword, Operator, Token, TokenKind},
};

#[derive(Debug)]
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

// #[derive(Debug)]
// pub struct ParseNode {
//     pub children: Vec<ParseNode>,
//     pub entry: GrammarItem,
// }

// impl Display for ParseNode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.output(f, 0, &"".to_string(), false)
//     }
// }

// impl ParseNode {
//     pub fn is_constant(&self) -> bool {
//         match self.entry {
//             GrammarItem::Literal(_) => true,
//             GrammarItem::Identifier(_) => false,
//             GrammarItem::Operator(o) => match o {
//                 Operator::Mult => self.children[0].is_constant() && self.children[1].is_constant(),
//                 _ => false,
//             },
//             _ => false,
//         }
//     }

//     fn output(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//         index: u32,
//         indent: &String,
//         last: bool,
//     ) -> std::fmt::Result {
//         write!(f, "{}", indent)?;
//         if index != 0 {
//             write!(f, "{}", if last { "└──" } else { "├──" })?;
//         }
//         write!(f, "{:?}\n", self.entry)?;
//         let nindent = format!(
//             "{}{}",
//             indent,
//             if index == 0 {
//                 ""
//             } else if last {
//                 "    "
//             } else {
//                 "│   "
//             }
//         );
//         self.children.iter().enumerate().for_each(|(i, v)| {
//             ParseNode::output(v, f, index + 1, &nindent, i == self.children.len() - 1).unwrap();
//         });
//         Ok(())
//     }
// }

pub fn parse_from_tokens(tokens: &Vec<Token>) -> Result<ParseNode, ParseError> {
    let mut it = tokens.iter().peekable();
    let ast = parse_statement(&mut it)?;
    Ok(ast)
}

fn parse_statement<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    println!("decleration");
    match tokens.peek() {
        Some(t) => match t.token_type {
            TokenKind::Keyword(k) => match k {
                Keyword::Let => parse_variable_decleration(tokens),
                _ => Ok(ParseNode::Expression(parse_expression(tokens, 0)?)),
            },
            _ => Ok(ParseNode::Expression(parse_expression(tokens, 0)?)),
        },
        None => Ok(ParseNode::None),
    }
}

fn parse_expression<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    prev_prec: u8,
) -> Result<Expression, ParseError> {
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
                Ok(n) => Ok(Expression::UnaryExpression(o.clone(), Box::new(n))),
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
            left = Ok(Expression::BinaryExpression(
                o.clone(),
                Box::new(left?),
                Box::new(right?),
            ));
        }
    }
    left
}

fn parse_variable_decleration<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::Keyword(Keyword::Let))?;
    let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
    let var_type = match tokens.peek() {
        Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) => {
            tokens.next();
            Some(Box::new(ParseNode::Type(parse_type(tokens)?)))
        }
        _ => None,
    };
    let var_initializer = match tokens.peek() {
        Some(Token {
            token_type: TokenKind::Operator(Operator::Assignment),
            ..
        }) => {
            tokens.next();
            Some(Box::new(ParseNode::Expression(parse_expression(
                tokens, 0,
            )?)))
        }
        _ => None,
    };
    Ok(ParseNode::VariableDecleration(
        identifier.clone(),
        var_type,
        var_initializer,
    ))
}

fn parse_primary<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<Expression, ParseError> {
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
) -> Result<Expression, ParseError> {
    match tokens.peek() {
        Some(t) => match t {
            Token {
                token_type: TokenKind::Literal(a),
                ..
            } => {
                tokens.next();
                Ok(Expression::Literal(a.clone()))
            }
            Token {
                token_type: TokenKind::OpenBracket,
                ..
            } => parse_array_literal(tokens),
            Token {
                token_type: TokenKind::OpenBrace,
                ..
            } => parse_template_initializer(tokens, None),
            Token {
                token_type: TokenKind::Ident(a),
                ..
            } => parse_ident(tokens),
            _ => Err(ParseError::new(&"Unkown literal value!".to_string())),
        },
        None => Err(ParseError::new(&"Unkown literal value!".to_string())),
    }
}

fn parse_ident<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<Expression, ParseError> {
    let possible_type = parse_type(tokens)?;
    if let Some(Token {
        token_type: TokenKind::OpenBrace,
        ..
    }) = tokens.peek()
    {
        parse_template_initializer(tokens, Some(Box::new(possible_type)))
    } else {
        match possible_type {
            Type::NamedType(s) => match s.token_type {
                TokenKind::Ident(s) => Ok(Expression::Identifier(s)),
                _ => Err(ParseError::new(&format!("Unexpected type in expression!"))),
            },
            _ => Err(ParseError::new(&format!("Unexpected type in expression!"))),
        }
    }
}

fn parse_template_initializer<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    struct_type: Option<Box<Type>>,
) -> Result<Expression, ParseError> {
    expect(tokens, TokenKind::OpenBrace)?;
    let mut key_values = vec![];

    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.peek()
        {
            break;
        }
        let key = expect(tokens, TokenKind::Ident("".to_string()))?;
        let key_string = match &key.token_type {
            TokenKind::Ident(s) => s.clone(),
            _ => panic!("Shouldn't be here!"),
        };
        let value = if let Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) = tokens.peek()
        {
            tokens.next();
            Some(parse_expression(tokens, 0)?)
        } else {
            None
        };
        key_values.push((key_string, value));

        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.next(),
                TokenKind::CloseBrace => {
                    break;
                }
                _ => {
                    return Err(ParseError::new(&format!(
                        "Expected comma or closing brace!"
                    )))
                }
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }

    expect(tokens, TokenKind::CloseBrace)?;

    Ok(Expression::Literal(Literal::TemplateInitializer(
        struct_type,
        key_values,
    )))
}

fn parse_array_literal<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<Expression, ParseError> {
    expect(tokens, TokenKind::OpenBracket)?;
    let mut values = vec![];
    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseBracket,
            ..
        }) = tokens.peek()
        {
            break;
        }
        let value = parse_expression(tokens, 0)?;
        values.push(value);
        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.next(),
                TokenKind::CloseBracket => {
                    break;
                }
                _ => {
                    return Err(ParseError::new(&format!(
                        "Expected comma or closing bracket!"
                    )))
                }
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }
    expect(tokens, TokenKind::CloseBracket)?;

    Ok(Expression::Literal(Literal::Array(values)))
}

fn parse_type<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<Type, ParseError> {
    match tokens.next() {
        Some(t) => match &t.token_type {
            TokenKind::Ident(_) => Ok(Type::NamedType((*t).clone())),
            TokenKind::Keyword(k) => match k {
                Keyword::Int => Ok(Type::Int(8)),
                Keyword::Uint => Ok(Type::Uint(8)),
                Keyword::Bool => Ok(Type::Bool),
                Keyword::Char => Ok(Type::Char),
                Keyword::Float => Ok(Type::Float),
                _ => Err(ParseError::new(&format!("{:?} is not a valid type!", k))),
            },
            TokenKind::OpenBracket => {
                let array_type = parse_type(tokens)?;
                let size = if let Some(Token {
                    token_type: TokenKind::Colon,
                    ..
                }) = tokens.peek()
                {
                    tokens.next();
                    let size = expect(tokens, TokenKind::Literal(Literal::Integer(0, 0)))?;
                    let numeric_size = match size {
                        Token {
                            token_type: TokenKind::Literal(Literal::Integer(i, _)),
                            ..
                        } => *i as usize,
                        _ => {
                            return Err(ParseError::new(&format!(
                                "Expected constant integer for array size!"
                            )));
                        }
                    };
                    Some(numeric_size)
                } else {
                    None
                };
                Ok(Type::ArrayType(Box::new(array_type), size))
            }
            TokenKind::OpenParen => {
                let mut parameters = vec![];
                while let Some(_) = tokens.peek() {
                    let parameter_type = parse_type(tokens)?;
                    parameters.push(parameter_type);

                    match tokens.peek() {
                        Some(t) => match t.token_type {
                            TokenKind::Comma => tokens.next(),
                            TokenKind::CloseParen => {
                                tokens.next();
                                break;
                            }
                            _ => {
                                return Err(ParseError::new(&format!(
                                    "Expected comma or closing parenthesis!"
                                )))
                            }
                        },
                        None => return Err(ParseError::new(&format!("Expected token!"))),
                    };
                }
                expect(tokens, TokenKind::CloseParen)?;
                let ret_type = if let Some(Token {
                    token_type: TokenKind::Operator(Operator::Arrow),
                    ..
                }) = tokens.peek()
                {
                    tokens.next();
                    parse_type(tokens)?
                } else {
                    Type::Unit
                };
                Ok(Type::FunctionType(parameters, Box::new(ret_type)))
            }
            TokenKind::Operator(Operator::BitAnd) => {
                Ok(Type::ReferenceType(Box::new(parse_type(tokens)?)))
            }
            _ => Err(ParseError::new(&format!("{:?} is not a valid type!", t))),
        },
        None => Err(ParseError::new(&format!("Expected more tokens for type!"))),
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

fn expect<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    token_type: TokenKind,
) -> Result<&'a Token, ParseError> {
    match tokens.next() {
        Some(t) if std::mem::discriminant(&t.token_type) == std::mem::discriminant(&token_type) => {
            Ok(t)
        }
        Some(t) => Err(ParseError::new(&format!(
            "Expected token {:?}, found token {:?}",
            token_type, t.token_type
        ))),
        None => Err(ParseError::new(&format!(
            "Expected token {:?} ",
            token_type
        ))),
    }
}
