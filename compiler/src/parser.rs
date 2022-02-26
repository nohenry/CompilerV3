use std::{
    fmt::{self, Display},
    iter::Peekable,
};

use crate::{
    ast::{Expression, Literal, LoopExpression, ParseNode, Type},
    lexer::{KeywordKind, OperatorKind, Token, TokenKind},
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
    let mut statements = vec![];
    let mut current_tags = vec![];
    while let Some(_) = it.peek() {
        let statement = parse_top_level_statement(&mut it)?;
        match statement {
            ParseNode::Tag(_) => {
                current_tags.push(statement);
            }
            _ => {
                if current_tags.len() > 0 {
                    statements.push(ParseNode::TagCollection(
                        current_tags.clone(),
                        Some(Box::new(statement)),
                    ));
                    current_tags.clear();
                } else {
                    statements.push(statement)
                }
            }
        }
    }
    Ok(ParseNode::Block(statements))
}

fn parse_top_level_statement<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    match tokens.peek() {
        Some(t) => match t.token_type {
            TokenKind::OpenBracket => parse_tag(tokens),
            TokenKind::Ident(_) => parse_function(tokens),
            TokenKind::Keyword(k) => match k {
                KeywordKind::Import => parse_import(tokens),
                KeywordKind::Template => parse_template(tokens),
                KeywordKind::Action => parse_action(tokens),
                KeywordKind::Type => {
                    tokens.next();
                    let new_type = parse_type(tokens)?;
                    expect(tokens, TokenKind::Operator(OperatorKind::Assignment))?;
                    let current_type = parse_type(tokens)?;
                    Ok(ParseNode::TypeDecleration(new_type, current_type))
                }
                _ => Err(ParseError::new(&format!("Unexpected keyword {:?}", t))),
            },
            _ => Err(ParseError::new(&format!("Unexpected token {:?}", t))),
        },
        None => Ok(ParseNode::None),
    }
}

fn parse_statement<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    match tokens.peek() {
        Some(t) => match t.token_type {
            TokenKind::Keyword(k) => match k {
                KeywordKind::Let => parse_variable_decleration(tokens),
                KeywordKind::Yield => {
                    tokens.next();
                    Ok(ParseNode::Yield(Box::new(parse_expression(tokens, 0)?)))
                }
                KeywordKind::Return => {
                    tokens.next();
                    Ok(ParseNode::Return(Box::new(parse_expression(tokens, 0)?)))
                }
                _ => Ok(ParseNode::Expression(parse_expression(tokens, 0)?)),
            },
            TokenKind::OpenBrace => parse_block_statement(tokens),
            _ => Ok(ParseNode::Expression(parse_expression(tokens, 0)?)),
        },
        None => Ok(ParseNode::None),
    }
}

fn parse_template<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::Keyword(KeywordKind::Template))?;
    let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
    let generic = if let Some(Token {
        token_type: TokenKind::Operator(OperatorKind::Lt),
        ..
    }) = tokens.peek()
    {
        Some(Box::new(parse_generic(tokens)?))
    } else {
        None
    };
    expect(tokens, TokenKind::OpenBrace)?;
    let mut fields = vec![];

    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.peek()
        {
            break;
        }
        let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
        expect(tokens, TokenKind::Colon)?;
        let field_type = parse_type(tokens)?;
        fields.push((identifier.clone(), field_type));
    }

    expect(tokens, TokenKind::CloseBrace)?;
    Ok(ParseNode::TemplateDecleration(
        identifier.clone(),
        fields,
        generic,
    ))
}

fn parse_action<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::Keyword(KeywordKind::Action))?;
    let templ_type = parse_type(tokens)?;
    let spec = match tokens.peek() {
        Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) => {
            tokens.next();
            Some(parse_type(tokens)?)
        }
        _ => None,
    };
    expect(tokens, TokenKind::OpenBrace)?;
    let mut statements = vec![];

    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.peek()
        {
            break;
        }
        statements.push(parse_action_statement(tokens)?);

        // match tokens.peek() {
        //     Some(t) => match t.token_type {
        //         TokenKind::Comma => tokens.next(),
        //         TokenKind::CloseBrace => {
        //             break;
        //         }
        //         _ => {
        //             return Err(ParseError::new(&format!(
        //                 "Expected comma or closing brace!"
        //             )))
        //         }
        //     },
        //     None => return Err(ParseError::new(&format!("Expected token!"))),
        // };
    }

    expect(tokens, TokenKind::CloseBrace)?;
    Ok(ParseNode::ActionDecleration(
        templ_type,
        spec,
        Box::new(ParseNode::Block(statements)),
    ))
}

fn parse_action_statement<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    match tokens.peek() {
        Some(t) => match t.token_type {
            TokenKind::Ident(_) => parse_function(tokens),
            _ => Err(ParseError::new(&format!(
                "Unexpected token {:?} found in action statement!",
                t
            ))),
        },
        None => Ok(ParseNode::None),
    }
}

fn parse_function_call<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    to_be_called: Expression,
) -> Result<Expression, ParseError> {
    expect(tokens, TokenKind::OpenParen)?;
    let mut args = vec![];
    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseParen,
            ..
        }) = tokens.peek()
        {
            break;
        }
        args.push(parse_expression(tokens, 0)?);
        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.next(),
                TokenKind::CloseParen => {
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
    Ok(Expression::FunctionCall(Box::new(to_be_called), args))
}

fn parse_function<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    let ident_token = expect(tokens, TokenKind::Ident("".to_string()))?;
    let generic = if let Some(Token {
        token_type: TokenKind::Operator(OperatorKind::Lt),
        ..
    }) = tokens.peek()
    {
        Some(Box::new(parse_generic(tokens)?))
    } else {
        None
    };

    expect(tokens, TokenKind::OpenParen)?;
    Ok(ParseNode::FunctionDecleration(
        ident_token.clone(),
        generic,
        parse_function_type(tokens)?,
    ))
}

fn parse_function_type<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<FunctionType, ParseError> {
    let mut params = vec![];
    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseParen,
            ..
        }) = tokens.peek()
        {
            break;
        }
        let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
        expect(tokens, TokenKind::Colon)?;
        let parameter_type = parse_type(tokens)?;
        params.push((identifier.clone(), parameter_type));

        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.next(),
                TokenKind::CloseParen => {
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
    expect(tokens, TokenKind::Operator(OperatorKind::Arrow))?;
    let ret_type = match parse_type(tokens) {
        Ok(t) => t,
        Err(_) => Type::Unit,
    };
    let body = parse_statement(tokens)?;
    Ok((params, ret_type, Box::new(body)))
}

fn parse_function_type_with_first<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    first: &Token,
) -> Result<FunctionType, ParseError> {
    let mut params = vec![];

    expect(tokens, TokenKind::Colon)?;
    let parameter_type = parse_type(tokens)?;
    params.push((first.clone(), parameter_type));
    match tokens.peek() {
        Some(t) => match t.token_type {
            TokenKind::Comma => tokens.next(),
            TokenKind::CloseParen => None,
            _ => {
                return Err(ParseError::new(&format!(
                    "Expected comma or closing parenthesis!"
                )))
            }
        },
        None => return Err(ParseError::new(&format!("Expected token!"))),
    };

    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseParen,
            ..
        }) = tokens.peek()
        {
            break;
        }
        let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
        expect(tokens, TokenKind::Colon)?;
        let parameter_type = parse_type(tokens)?;
        params.push((identifier.clone(), parameter_type));

        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.next(),
                TokenKind::CloseParen => {
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
    expect(tokens, TokenKind::Operator(OperatorKind::Arrow))?;
    let ret_type = match parse_type(tokens) {
        Ok(t) => t,
        Err(_) => Type::Unit,
    };
    let body = parse_statement(tokens)?;
    Ok((params, ret_type, Box::new(body)))
}

fn parse_block_statement<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::OpenBrace)?;
    let mut statements = vec![];
    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.peek()
        {
            break;
        }

        statements.push(parse_statement(tokens)?);

        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::CloseBrace => {
                    break;
                }
                _ => (),
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }
    expect(tokens, TokenKind::CloseBrace)?;
    Ok(ParseNode::Block(statements))
}

fn parse_expression<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
    prev_prec: u8,
) -> Result<Expression, ParseError> {
    match tokens.peek() {
        Some(t) => match t.token_type {
            TokenKind::Keyword(KeywordKind::If) => {
                tokens.next(); // eat keyword
                let condition = parse_expression(tokens, 0)?;
                let body = parse_block_statement(tokens)?;
                let else_clause = if let Some(Token {
                    token_type: TokenKind::Keyword(KeywordKind::Else),
                    ..
                }) = tokens.peek()
                {
                    tokens.next();
                    Some(Box::new(parse_block_statement(tokens)?))
                } else {
                    None
                };

                Ok(Expression::IfExpression(
                    Box::new(condition),
                    Box::new(body),
                    else_clause,
                ))
            }
            TokenKind::Keyword(KeywordKind::Loop) => {
                tokens.next();
                if let Some(Token {
                    token_type: TokenKind::OpenBrace,
                    ..
                }) = tokens.peek()
                {
                    Ok(Expression::LoopExpression(LoopExpression::Infinite(
                        Box::new(parse_block_statement(tokens)?),
                    )))
                } else {
                    Ok(Expression::LoopExpression(LoopExpression::Until(
                        Box::new(parse_expression(tokens, 0)?),
                        Box::new(parse_block_statement(tokens)?),
                    )))
                }
            }
            _ => parse_operator_expression(tokens, prev_prec),
        },
        None => Err(ParseError::new(&String::from(
            "Expected some token in expression!",
        ))),
    }
}

fn parse_operator_expression<'a, T: Iterator<Item = &'a Token>>(
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

    let nleft = left?;
    let nnleft = nleft.clone();
    if let (
        Some(Token {
            token_type: TokenKind::Colon,
            ..
        }),
        Expression::Identifier(t),
    ) = (tokens.peek(), nleft)
    {
        Ok(Expression::Lambda(parse_function_type_with_first(
            tokens, &t,
        )?))
    } else {
        while let Some(Token { token_type, .. }) = tokens.peek() {
            let prec = postfix_precedence(token_type);
            if prec <= prev_prec || prec == 0 {
                break;
            }
            match token_type {
                TokenKind::OpenParen => return parse_function_call(tokens, nnleft),
                TokenKind::OpenBracket => {
                    expect(tokens, TokenKind::OpenBracket)?;
                    let value = parse_expression(tokens, 0)?;
                    expect(tokens, TokenKind::CloseBracket)?;
                    return Ok(Expression::Index(Box::new(nnleft), Box::new(value)));
                }
                _ => return Ok(nnleft),
            }
        }

        Ok(nnleft)
    }
}

fn parse_variable_decleration<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::Keyword(KeywordKind::Let))?;
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
            token_type: TokenKind::Operator(OperatorKind::Assignment),
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

fn parse_tag<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::OpenBracket)?;
    let expression = parse_expression(tokens, 0)?;
    expect(tokens, TokenKind::CloseBracket)?;
    Ok(ParseNode::Tag(expression))
}

fn parse_import<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::Keyword(KeywordKind::Import))?;
    let mut modules = vec![];
    let thing = parse_expression(tokens, 0)?;
    fn add_wild(modules: &mut Vec<Expression>, node: &Expression) {
        match node {
            Expression::BinaryExpression(_, l, r) => {
                add_wild(modules, l.as_ref());
                add_wild(modules, r.as_ref());
            }
            Expression::Identifier(_) => {
                modules.push(node.clone());
            }
            _ => (),
        }
    }
    add_wild(&mut modules, &thing);
    let wildcard = if let Some(Token {
        token_type: TokenKind::Operator(OperatorKind::Wildcard),
        ..
    }) = tokens.peek()
    {
        tokens.next();
        true
    } else {
        false
    };
    Ok(ParseNode::Import(modules, wildcard))
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
                match tokens.peek() {
                    Some(Token {
                        token_type: TokenKind::CloseParen,
                        ..
                    }) => Ok(Expression::Lambda(parse_function_type(tokens)?)),
                    _ => {
                        let expr = parse_expression(tokens, 0)?;
                        match expr {
                            Expression::Lambda(l) => Ok(Expression::Lambda(l)),
                            _ => {
                                expect(tokens, TokenKind::CloseParen)?;
                                Ok(expr)
                            }
                        }
                    }
                }
            }
            _ => parse_literal(tokens),
        },
        None => parse_literal(tokens),
    }
}

fn parse_generic<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<ParseNode, ParseError> {
    expect(tokens, TokenKind::Operator(OperatorKind::Lt))?;
    let mut generic_params = vec![];
    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::Operator(OperatorKind::Gt),
            ..
        }) = tokens.peek()
        {
            break;
        }

        let type_param = expect(tokens, TokenKind::Ident("".to_string()))?;

        let constraints = if let Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) = tokens.peek()
        {
            Some(parse_generic_constraints(tokens)?)
        } else {
            None
        };
        generic_params.push((type_param.clone(), constraints));

        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::Operator(OperatorKind::Gt) => {
                    break;
                }
                _ => (),
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }
    expect(tokens, TokenKind::Operator(OperatorKind::Gt))?;
    Ok(ParseNode::GenericParameters(generic_params))
}

fn parse_generic_constraints<'a, T: Iterator<Item = &'a Token>>(
    tokens: &mut Peekable<T>,
) -> Result<Vec<Type>, ParseError> {
    expect(tokens, TokenKind::Colon)?;
    let mut constraints = vec![];
    while let Some(_) = tokens.peek() {
        if let Some(Token {
            token_type: TokenKind::Operator(OperatorKind::Gt),
            ..
        }) = tokens.peek()
        {
            break;
        }

        let constraint_type = parse_type(tokens)?;
        constraints.push(constraint_type);

        match tokens.peek() {
            Some(t) => match t.token_type {
                TokenKind::Operator(OperatorKind::BitAnd) => {
                    tokens.next();
                }
                TokenKind::Operator(OperatorKind::Gt) | TokenKind::Comma => break,
                _ => (),
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }
    Ok(constraints)
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
                token_type: TokenKind::Ident(_),
                ..
            } => parse_ident(tokens),
            Token {
                token_type: TokenKind::Keyword(k),
                ..
            } => match k {
                KeywordKind::True => {
                    tokens.next();
                    Ok(Expression::Literal(Literal::Boolean(true)))
                }
                KeywordKind::False => {
                    tokens.next();
                    Ok(Expression::Literal(Literal::Boolean(false)))
                }
                _ => Err(ParseError::new(&format!(
                    "Keyword {:?} is not a valid literal!",
                    k
                ))),
            },
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
            Type::NamedType(t) => match t.token_type {
                TokenKind::Ident(_) => Ok(Expression::Identifier(t)),
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
    match tokens.peek() {
        Some(t) => {
            let result = match t.token_type {
                TokenKind::Ident(_) => {
                    let token = (*t).clone();
                    tokens.next();
                    Ok(Type::NamedType(token))
                }
                TokenKind::Keyword(k) => {
                    tokens.next();
                    match k {
                        KeywordKind::Int => Ok(Type::Int(8)),
                        KeywordKind::Uint => Ok(Type::Uint(8)),
                        KeywordKind::Bool => Ok(Type::Bool),
                        KeywordKind::Char => Ok(Type::Char),
                        KeywordKind::Float => Ok(Type::Float),
                        _ => Err(ParseError::new(&format!("{:?} is not a valid type!", k))),
                    }
                }
                TokenKind::OpenBracket => {
                    tokens.next();
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
                    tokens.next();
                    let mut parameters = vec![];
                    while let Some(_) = tokens.peek() {
                        if let Some(Token {
                            token_type: TokenKind::CloseParen,
                            ..
                        }) = tokens.peek()
                        {
                            break;
                        }
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
                        token_type: TokenKind::Operator(OperatorKind::Arrow),
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
                TokenKind::Operator(OperatorKind::BitAnd) => {
                    tokens.next();
                    Ok(Type::ReferenceType(Box::new(parse_type(tokens)?)))
                }
                _ => Err(ParseError::new(&format!("{:?} is not a valid type!", t))),
            };
            if let Some(Token {
                token_type: TokenKind::Operator(OperatorKind::Lt),
                ..
            }) = tokens.peek()
            {
                tokens.next();
                let mut type_arguments = vec![];
                while let Some(_) = tokens.peek() {
                    if let Some(Token {
                        token_type: TokenKind::Operator(OperatorKind::Gt),
                        ..
                    }) = tokens.peek()
                    {
                        break;
                    }

                    let arg_type = parse_type(tokens)?;
                    type_arguments.push(arg_type);

                    match tokens.peek() {
                        Some(t) => match t.token_type {
                            TokenKind::Comma => tokens.next(),
                            TokenKind::Operator(OperatorKind::Gt) => {
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
                expect(tokens, TokenKind::Operator(OperatorKind::Gt))?;
                return Ok(Type::GenericType(Box::new(result?), type_arguments));
            }
            result
        }
        None => Err(ParseError::new(&format!("Expected more tokens for type!"))),
    }
}

fn unary_precedence(operator: OperatorKind) -> u8 {
    match operator {
        OperatorKind::Minus
        | OperatorKind::LogicalNot
        | OperatorKind::BitNot
        | OperatorKind::Mult
        | OperatorKind::BitAnd => 14,
        _ => 0,
    }
}

fn binary_precedence(operator: OperatorKind) -> u8 {
    match operator {
        OperatorKind::Assignment => 2,
        OperatorKind::LogicalOr => 3,
        OperatorKind::LogicalXor => 4,
        OperatorKind::LogicalAnd => 5,
        OperatorKind::BitOr => 6,
        OperatorKind::BitXor => 7,
        OperatorKind::BitAnd => 8,
        OperatorKind::Eq | OperatorKind::NEq => 9,
        OperatorKind::Lt
        | OperatorKind::LtEq
        | OperatorKind::Gt
        | OperatorKind::GtEq
        | OperatorKind::NGt
        | OperatorKind::NLt => 10,
        OperatorKind::BitLeft | OperatorKind::BitRight => 11,
        OperatorKind::Plus | OperatorKind::Minus | OperatorKind::Percent => 12,
        OperatorKind::Mult | OperatorKind::Divide => 13,
        OperatorKind::Spread | OperatorKind::As => 14,
        OperatorKind::Dot => 15,
        _ => 0,
    }
}

fn postfix_precedence(token: &TokenKind) -> u8 {
    match token {
        TokenKind::OpenParen => 15,
        TokenKind::OpenBracket => 15,
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
