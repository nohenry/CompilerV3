#![feature(linked_list_cursors)]
use std::collections::{linked_list::Cursor, LinkedList};

use dsl_lexer::ast::{
    ActionDecleration, ArrayInitializer, ArrayType, BinaryExpression, Expression, ExpressionIndex,
    FunctionCall, FunctionDecleration, FunctionSignature, FunctionType, GenericParameters,
    GenericType, IfExpression, ImportDecleration, Literal, Loop, LoopExpression, ParseNode,
    ReferenceType, SpecBody, SpecDecleration, TemplateDecleration, TemplateInitializer, Type,
    TypeDecleration, TypeSymbol, UnaryExpression, VariableDecleration,
};
use dsl_lexer::{default_range, Keyword, KeywordKind, Operator, OperatorKind, Token, TokenKind};

use dsl_errors::ParseError;

pub fn parse_from_tokens(tokens: &LinkedList<&Token>) -> Result<ParseNode, ParseError> {
    let mut it = tokens.cursor_front();
    let mut statements = vec![];
    let mut current_tags = vec![];

    let start = if let Some(t) = it.current() {
        t.range
    } else {
        default_range()
    };

    while let Some(_) = it.current() {
        let statement = parse_top_level_statement(&mut it)?;
        match statement {
            ParseNode::Tag(_, _) => {
                current_tags.push(statement);
            }
            _ => {
                if current_tags.len() > 0 {
                    statements.push(ParseNode::TagCollection(
                        current_tags.clone(),
                        Box::new(statement),
                        (
                            current_tags[0].get_range().0,
                            current_tags[current_tags.len() - 1].get_range().1,
                        ),
                    ));
                    current_tags.clear();
                } else {
                    statements.push(statement)
                }
            }
        }
    }

    let end = if let Some(t) = it.current() {
        t.range
    } else if let Some(t) = it.back() {
        t.range
    } else {
        default_range()
    };
    Ok(ParseNode::Block(statements, (start.0, end.1)))
}

fn parse_top_level_statement(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    match tokens.current() {
        Some(t) => match t.token_type {
            TokenKind::OpenBracket => parse_tag(tokens),
            TokenKind::Keyword(k) => match k.keyword {
                KeywordKind::Import => {
                    let res = parse_import(tokens);
                    res
                }
                KeywordKind::Type => {
                    tokens.move_next();
                    let new_type = expect(tokens, TokenKind::Ident(String::from("")))?;
                    let eq = expect(tokens, Operator::create_expect(OperatorKind::Assignment))?;
                    let current_type = parse_type(tokens)?;

                    let end = current_type.get_range().1;

                    let td = TypeDecleration {
                        type_keyword: t.range,
                        token: new_type.clone(),
                        old_type: current_type,
                        assignment: eq.range,
                        range: (t.range.0, end),
                    };

                    Ok(ParseNode::TypeDecleration(td))
                }
                KeywordKind::Template => parse_template(tokens),
                KeywordKind::Action => parse_action(tokens),
                KeywordKind::Spec => parse_spec(tokens),
                _ => Err(ParseError::new(&format!("Unexpected keyword {:?}", t))),
            },
            TokenKind::Ident(_) => Ok(parse_function(tokens)?),
            _ => Err(ParseError::new(&format!("Unexpected token {:?}", t))),
        },
        None => Ok(ParseNode::None),
    }
}

fn parse_statement(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    match tokens.current() {
        Some(t) => match t.token_type {
            TokenKind::Keyword(k) => match k.keyword {
                KeywordKind::Let => parse_variable_decleration(tokens),
                KeywordKind::Yield => {
                    let tok = tokens.current().unwrap();
                    tokens.move_next();
                    Ok(ParseNode::Yield(
                        Box::new(parse_expression(tokens, 0)?),
                        tok.range,
                    ))
                }
                KeywordKind::Return => {
                    let tok = tokens.current().unwrap();
                    tokens.move_next();
                    Ok(ParseNode::Return(
                        Box::new(parse_expression(tokens, 0)?),
                        tok.range,
                    ))
                }
                _ => {
                    let expr = parse_expression(tokens, 0)?;
                    // expect(tokens, TokenKind::Semi)?;
                    let rng = expr.get_range();
                    Ok(ParseNode::Expression(expr, rng))
                }
            },
            TokenKind::OpenBrace => parse_block_statement(tokens),
            _ => {
                let expr = parse_expression(tokens, 0)?;
                // expect(tokens, TokenKind::Semi)?;
                let rng = expr.get_range();
                Ok(ParseNode::Expression(expr, rng))
            }
        },
        None => Ok(ParseNode::None),
    }
}

fn parse_template(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let kw = expect(tokens, Keyword::create_expect(KeywordKind::Template))?;
    let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
    let generic = if let Some(Token {
        token_type:
            TokenKind::Operator(Operator {
                operator: OperatorKind::Lt,
                ..
            }),
        ..
    }) = tokens.current()
    {
        Some(Box::new(parse_generic(tokens)?))
    } else {
        None
    };

    let ob = expect(tokens, TokenKind::OpenBrace)?;
    let mut fields = vec![];

    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.current()
        {
            break;
        }

        let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
        expect(tokens, TokenKind::Colon)?;
        let field_type = parse_type(tokens)?;

        let ts = TypeSymbol {
            symbol_type: field_type,
            symbol: identifier.clone(),
        };
        fields.push(ts);
    }

    let cb = expect(tokens, TokenKind::CloseBrace)?;
    let sd = TemplateDecleration {
        struct_keyword: kw.clone().range,
        token: identifier.clone(),
        fields,
        generic,
        range: (kw.range.0, cb.range.1),
    };
    Ok(ParseNode::TemplateDecleration(sd))
}

fn parse_action(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let keyword = expect(tokens, Keyword::create_expect(KeywordKind::Action))?;
    let generic = if let Some(Token {
        token_type:
            TokenKind::Operator(Operator {
                operator: OperatorKind::Lt,
                ..
            }),
        ..
    }) = tokens.current()
    {
        Some(Box::new(parse_generic(tokens)?))
    } else {
        None
    };

    let template_type = parse_type(tokens)?;
    let spec = match tokens.current() {
        Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) => {
            tokens.move_next();
            Some(parse_type(tokens)?)
        }
        _ => None,
    };
    let left = expect(tokens, TokenKind::OpenBrace)?;
    let mut statements = vec![];

    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.current()
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

    let right = expect(tokens, TokenKind::CloseBrace)?;
    Ok(ParseNode::ActionDecleration(ActionDecleration {
        action_keyword: keyword.range,
        template_type,
        generic,
        specification: spec,
        body: Box::new(ParseNode::Block(statements, (left.range.0, right.range.1))),
        range: (keyword.range.0, right.range.1),
    }))
}

fn parse_action_statement(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    match tokens.current() {
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

fn parse_spec(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let keyword = expect(tokens, Keyword::create_expect(KeywordKind::Spec))?;
    let generic = if let Some(Token {
        token_type:
            TokenKind::Operator(Operator {
                operator: OperatorKind::Lt,
                ..
            }),
        ..
    }) = tokens.current()
    {
        Some(Box::new(parse_generic(tokens)?))
    } else {
        None
    };

    let identifier = expect(tokens, TokenKind::Ident(String::from("")))?;
    let left = expect(tokens, TokenKind::OpenBrace)?;
    let mut statements = vec![];

    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.current()
        {
            break;
        }
        statements.push(parse_spec_statement(tokens)?);
    }

    let right = expect(tokens, TokenKind::CloseBrace)?;
    Ok(ParseNode::SpecDecleration(SpecDecleration {
        spec_keyword: keyword.range,
        identifier: identifier.clone(),
        generic,
        body: statements,
        range: (keyword.range.0, right.range.1),
    }))
}

fn parse_spec_statement(tokens: &mut Cursor<&Token>) -> Result<SpecBody, ParseError> {
    match tokens.current() {
        Some(t) => match t.token_type {
            TokenKind::Ident(_) => {
                tokens.move_next();
                Ok(SpecBody::Function(
                    (*t).clone(),
                    parse_function_type(tokens, None)?,
                ))
            }
            _ => Err(ParseError::new(&format!(
                "Unexpected token {:?} found in action statement!",
                t
            ))),
        },
        None => Err(ParseError::new(&format!("Unkown field in spec statement!"))),
    }
}

fn parse_function_call(
    tokens: &mut Cursor<&Token>,
    to_be_called: Expression,
) -> Result<Expression, ParseError> {
    let op = expect(tokens, TokenKind::OpenParen)?;
    let mut args = vec![];
    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseParen,
            ..
        }) = tokens.current()
        {
            break;
        }
        args.push(parse_expression(tokens, 0)?);
        match tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.move_next(),
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
    let cp = expect(tokens, TokenKind::CloseParen)?;
    let start = to_be_called.get_range().0;
    let fc = FunctionCall {
        expression_to_call: Box::new(to_be_called),
        arguments: args,
        paren_tokens: (op.range.0, cp.range.1),
        range: (start, cp.range.1),
    };
    Ok(Expression::FunctionCall(fc))
}

// fn parse_var_or_func(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
//     let ttype = parse_type(tokens)?;
//     let ident_token = expect(tokens, TokenKind::Ident("".to_string()))?;

//     if let Some(t) = tokens.current() {
//         match t.token_type {
//             TokenKind::OpenParen => {
//                 return parse_function(tokens, ttype, ident_token);
//             }
//             _ => return parse_variable_decleration(tokens, ttype, ident_token),
//         }
//     }
//     Err(ParseError::new(&String::from(
//         "Could not parse funciton or variable",
//     )))
// }

fn parse_function(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let ident_token = expect(tokens, TokenKind::Ident("".to_string()))?;
    let generic = if let Some(Token {
        token_type:
            TokenKind::Operator(Operator {
                operator: OperatorKind::Lt,
                ..
            }),
        ..
    }) = tokens.peek_next()
    {
        Some(Box::new(parse_generic(tokens)?))
    } else {
        None
    };

    let fn_type = parse_function_type(tokens, None)?;
    let body = parse_statement(tokens)?;

    let end = body.get_range().1;

    let fd = FunctionDecleration {
        identifier: ident_token.clone(),
        function_type: fn_type,
        body: Box::new(body),
        generic,
        range: (ident_token.range.0, end),
    };

    Ok(ParseNode::FunctionDecleration(fd))
}

fn parse_function_type(
    tokens: &mut Cursor<&Token>,
    first: Option<(&Token, &Token)>,
) -> Result<FunctionSignature, ParseError> {
    let mut params = vec![];

    let op = match first {
        Some((op, prm)) => {
            expect(tokens, TokenKind::Colon)?;
            let parameter_type = parse_type(tokens)?;
            let ts = TypeSymbol {
                symbol_type: parameter_type,
                symbol: prm.clone(),
            };
            params.push(ts);

            match tokens.peek_next() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => tokens.move_next(),
                    TokenKind::CloseParen => (),
                    _ => {
                        return Err(ParseError::new(&format!(
                            "Expected comma or closing parenthesis!"
                        )))
                    }
                },
                None => return Err(ParseError::new(&format!("Expected token!"))),
            };
            op
        }
        None => expect(tokens, TokenKind::OpenParen)?,
    };

    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseParen,
            ..
        }) = tokens.current()
        {
            break;
        }
        let parameter_type = parse_type(tokens)?;
        let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;
        let ts = TypeSymbol {
            symbol_type: parameter_type,
            symbol: identifier.clone(),
        };
        params.push(ts);

        match tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.move_next(),
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

    let cp = expect(tokens, TokenKind::CloseParen)?;
    let _arrow = expect(tokens, Operator::create_expect(OperatorKind::Arrow))?;

    let ret_type = match parse_type(tokens) {
        Ok(t) => t,
        Err(_) => Type::Unit,
    };

    let end = ret_type.get_range().1;
    Ok(FunctionSignature {
        parameters: params,
        return_type: Box::new(ret_type),
        parens: (op.range.0, cp.range.1),
        range: (op.range.0, end),
    })
}

fn parse_block_statement(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let op = expect(tokens, TokenKind::OpenBrace)?;
    let mut statements = vec![];
    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.current()
        {
            break;
        }

        statements.push(parse_statement(tokens)?);

        match tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::CloseBrace => {
                    break;
                }
                _ => (),
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }
    let cp = expect(tokens, TokenKind::CloseBrace)?;
    Ok(ParseNode::Block(statements, (op.range.0, cp.range.1)))
}

fn parse_operator_expression(
    tokens: &mut Cursor<&Token>,
    prev_prec: u8,
) -> Result<Expression, ParseError> {
    let mut left = if let Some(Token {
        token_type: TokenKind::Operator(o),
        ..
    }) = tokens.current()
    {
        let uprec = unary_precedence(*o);
        if uprec != 0 && uprec >= prev_prec {
            tokens.move_next();
            let right = parse_expression(tokens, uprec);
            match right {
                Ok(n) => {
                    let end = n.get_range().1;
                    let ue = UnaryExpression {
                        expression: Box::new(n),
                        operator: o.operator,
                        range: (o.range.0, end),
                    };
                    Ok(Expression::UnaryExpression(ue))
                }
                Err(_) => right,
            }
        } else {
            parse_expression(tokens, 0)
        }
    } else {
        parse_primary(tokens)
    };

    while let Some(t) = tokens.current() {
        match t {
            Token {
                token_type: TokenKind::Operator(o),
                ..
            } => {
                let prec = binary_precedence(*o);
                if prec <= prev_prec || prec == 0 {
                    break;
                }
                tokens.move_next();

                let lleft = left?;
                let right = parse_expression(tokens, prec)?;
                let start = lleft.get_range().0;
                let end = right.get_range().1;
                let be = BinaryExpression {
                    left: Box::new(lleft),
                    operator: o.operator,
                    right: Box::new(right),
                    range: (start, end),
                };
                left = Ok(Expression::BinaryExpression(be));
            }
            token => {
                let token_type = &token.token_type;
                let prec = postfix_precedence(token_type);
                if prec <= prev_prec || prec == 0 {
                    break;
                }

                let lleft = left?;
                match token_type {
                    TokenKind::OpenParen => return parse_function_call(tokens, lleft),
                    TokenKind::OpenBracket => {
                        let ob = expect(tokens, TokenKind::OpenBracket)?;
                        let value = parse_expression(tokens, 0)?;
                        let cb = expect(tokens, TokenKind::CloseBracket)?;
                        let idx = ExpressionIndex {
                            index_expression: Box::new(lleft),
                            index_value: Box::new(value),
                            square_range: (ob.range.0, cb.range.1),
                        };

                        left = Ok(Expression::Index(idx));
                    }
                    _ => return Ok(lleft),
                }
            }
        }
    }

    let nleft = left?;

    if let (
        Some(Token {
            token_type: TokenKind::Colon,
            ..
        }),
        Expression::Identifier(t),
    ) = (tokens.peek_next(), &nleft)
    {
        Ok(Expression::Lambda(
            parse_function_type(tokens, Some((tokens.current().unwrap(), &t)))?,
            Box::new(parse_statement(tokens)?),
        ))
    } else {
        Ok(nleft)
    }
}

fn parse_expression(tokens: &mut Cursor<&Token>, prev_prec: u8) -> Result<Expression, ParseError> {
    match tokens.current() {
        Some(t) => match t.token_type {
            TokenKind::Keyword(Keyword {
                keyword: KeywordKind::If,
                ..
            }) => {
                tokens.move_next(); // eat keyword
                let condition = parse_expression(tokens, 0)?;
                let body = parse_block_statement(tokens)?;
                let else_clause = if let Some(Token {
                    token_type:
                        TokenKind::Keyword(Keyword {
                            keyword: KeywordKind::Else,
                            ..
                        }),
                    ..
                }) = tokens.peek_next()
                {
                    let tok = tokens.current().unwrap();
                    tokens.move_next();
                    Some((tok.range, Box::new(parse_block_statement(tokens)?)))
                } else {
                    None
                };

                let end = else_clause.as_ref().map_or(body.get_range().1, |f| f.0 .1);
                Ok(Expression::IfExpression(IfExpression {
                    if_token: t.range,
                    condition: Box::new(condition),
                    body: Box::new(body),
                    else_clause,
                    range: (t.range.0, end),
                }))
            }
            TokenKind::Keyword(Keyword {
                keyword: KeywordKind::Loop,
                ..
            }) => {
                tokens.move_next();
                if let Some(Token {
                    token_type: TokenKind::OpenBrace,
                    ..
                }) = tokens.peek_next()
                {
                    let body = parse_block_statement(tokens)?;
                    let range = (t.range.0, body.get_range().1);
                    Ok(Expression::LoopExpression(LoopExpression {
                        keyword: t.range,
                        loop_type: Loop::Infinite(Box::new(body)),
                        range,
                    }))
                } else {
                    let body = parse_block_statement(tokens)?;
                    let range = (t.range.0, body.get_range().1);
                    Ok(Expression::LoopExpression(LoopExpression {
                        keyword: t.range,
                        loop_type: Loop::Until(
                            Box::new(parse_expression(tokens, 0)?),
                            Box::new(body),
                        ),

                        range,
                    }))
                }
            }
            _ => parse_operator_expression(tokens, prev_prec),
        },
        None => Err(ParseError::new(&String::from(
            "Expected some token in expression!",
        ))),
    }
}

fn parse_variable_decleration(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let keyword = expect(tokens, Keyword::create_expect(KeywordKind::Let))?;
    let identifier = expect(tokens, TokenKind::Ident("".to_string()))?;

    let var_type = match tokens.peek_next() {
        Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) => {
            tokens.move_next();
            let _type = parse_type(tokens)?;
            Some(Box::new(_type))
        }
        _ => None,
    };

    let var_initializer = match tokens.current() {
        Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Assignment,
                    ..
                }),
            ..
        }) => {
            let tok = tokens.current().unwrap();
            tokens.move_next();
            Some((Box::new(parse_expression(tokens, 0)?), tok.range))
        }
        _ => None,
    };

    // expect(tokens, TokenKind::Semi)?;

    let end = match &var_initializer {
        Some(s) => s.1,
        None => identifier.range,
    };
    let start = keyword.range.0;
    let vd = VariableDecleration {
        variable_type: var_type,
        possible_initializer: var_initializer,
        identifier: identifier.clone(),
        range: (start, end.1),
    };
    Ok(ParseNode::VariableDecleration(vd))
}

fn parse_tag(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let left = expect(tokens, TokenKind::OpenBracket)?;
    let expression = parse_expression(tokens, 0)?;
    let right = expect(tokens, TokenKind::CloseBracket)?;
    Ok(ParseNode::Tag(expression, (left.range.0, right.range.1)))
}

fn parse_import(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let keyword = expect(tokens, Keyword::create_expect(KeywordKind::Import))?;
    let mut modules = vec![];
    let thing = parse_expression(tokens, 0)?;
    fn add_wild(modules: &mut Vec<Expression>, node: &Expression) {
        match node {
            Expression::BinaryExpression(BinaryExpression { left, right, .. }) => {
                add_wild(modules, left.as_ref());
                add_wild(modules, right.as_ref());
            }
            Expression::Identifier(_) => {
                modules.push(node.clone());
            }
            _ => (),
        }
    }
    add_wild(&mut modules, &thing);
    let end = match modules.last() {
        Some(m) => m.get_range().1,
        None => keyword.range.1,
    };
    let id = ImportDecleration {
        import_keyword: keyword.range,
        path: modules,
        range: (keyword.range.0, end),
    };

    Ok(ParseNode::Import(id))
}

fn parse_primary(tokens: &mut Cursor<&Token>) -> Result<Expression, ParseError> {
    match tokens.current() {
        Some(t) => match t {
            Token {
                token_type: TokenKind::OpenParen,
                ..
            } => {
                tokens.move_next();
                match tokens.peek_next() {
                    Some(Token {
                        token_type: TokenKind::CloseParen,
                        ..
                    }) => {
                        tokens.move_prev();
                        Ok(Expression::Lambda(
                            parse_function_type(tokens, None)?,
                            Box::new(parse_statement(tokens)?),
                        ))
                    }
                    _ => {
                        let expr = parse_expression(tokens, 0)?;
                        match expr {
                            Expression::Lambda(l, b) => Ok(Expression::Lambda(l, b)),
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

fn parse_generic(tokens: &mut Cursor<&Token>) -> Result<ParseNode, ParseError> {
    let start = expect(tokens, Operator::create_expect(OperatorKind::Lt))?;
    // let gt = Operator::create_expect(OperatorKind::Gt);
    let mut generic_params = vec![];
    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Gt,
                    ..
                }),
            ..
        }) = tokens.current()
        {
            break;
        }

        let type_param = expect(tokens, TokenKind::Ident("".to_string()))?;

        let constraints = if let Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) = tokens.current()
        {
            Some(parse_generic_constraints(tokens)?)
        } else {
            None
        };
        generic_params.push((type_param.clone(), constraints));

        match tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Gt,
                    ..
                }) => {
                    break;
                }
                _ => (),
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }
    let end = expect(tokens, Operator::create_expect(OperatorKind::Gt))?;
    Ok(ParseNode::GenericParameters(GenericParameters {
        parameters: generic_params,
        range: (start.range.0, end.range.1),
    }))
}

fn parse_generic_constraints(tokens: &mut Cursor<&Token>) -> Result<Vec<Type>, ParseError> {
    expect(tokens, TokenKind::Colon)?;
    let mut constraints = vec![];
    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Gt,
                    ..
                }),
            ..
        }) = tokens.current()
        {
            break;
        }

        let constraint_type = parse_type(tokens)?;
        constraints.push(constraint_type);

        match tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Operator(Operator {
                    operator: OperatorKind::BitAnd,
                    ..
                }) => {
                    tokens.move_next();
                }
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Gt,
                    ..
                })
                | TokenKind::Comma => break,
                _ => (),
            },
            None => return Err(ParseError::new(&format!("Expected token!"))),
        };
    }
    Ok(constraints)
}

fn parse_literal(tokens: &mut Cursor<&Token>) -> Result<Expression, ParseError> {
    match tokens.current() {
        Some(t) => match t {
            Token {
                token_type: TokenKind::Literal(a),
                ..
            } => {
                tokens.move_next();
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
            } => match k.keyword {
                KeywordKind::True => {
                    tokens.move_next();
                    Ok(Expression::Literal(Literal::Boolean(true, t.range)))
                }
                KeywordKind::False => {
                    tokens.move_next();
                    Ok(Expression::Literal(Literal::Boolean(false, t.range)))
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

fn parse_ident(tokens: &mut Cursor<&Token>) -> Result<Expression, ParseError> {
    let possible_type = parse_type(tokens)?;

    if let Some(Token {
        token_type: TokenKind::OpenBrace,
        ..
    }) = tokens.current()
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

fn parse_template_initializer(
    tokens: &mut Cursor<&Token>,
    named_type: Option<Box<Type>>,
) -> Result<Expression, ParseError> {
    let ob = expect(tokens, TokenKind::OpenBrace)?;
    let mut key_values = vec![];

    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseBrace,
            ..
        }) = tokens.current()
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
        }) = tokens.current()
        {
            tokens.move_next();
            Some(parse_expression(tokens, 0)?)
        } else {
            None
        };
        key_values.push((key_string, value));

        match tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.move_next(),
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

    let cb = expect(tokens, TokenKind::CloseBrace)?;
    let start = named_type.as_ref().map_or(ob.range.0, |f| f.get_range().0);

    let si = TemplateInitializer {
        named_type,
        initializer_values: key_values,
        range: (start, cb.range.1),
    };
    Ok(Expression::Literal(Literal::StructInitializer(si)))
}

fn parse_array_literal(tokens: &mut Cursor<&Token>) -> Result<Expression, ParseError> {
    let ob = expect(tokens, TokenKind::OpenBracket)?;
    let mut values = vec![];
    while let Some(_) = tokens.current() {
        if let Some(Token {
            token_type: TokenKind::CloseBracket,
            ..
        }) = tokens.current()
        {
            break;
        }
        let value = parse_expression(tokens, 0)?;
        values.push(value);
        match tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Comma => tokens.move_next(),
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
    let cb = expect(tokens, TokenKind::CloseBracket)?;
    let ai = ArrayInitializer {
        elements: values,
        range: (ob.range.0, cb.range.1),
    };

    Ok(Expression::Literal(Literal::Array(ai)))
}

fn parse_type(tokens: &mut Cursor<&Token>) -> Result<Type, ParseError> {
    match tokens.current() {
        Some(t) => {
            let result = match t.token_type {
                TokenKind::Ident(_) => {
                    let token = (*t).clone();
                    tokens.move_next();
                    Ok(Type::NamedType(token))
                }
                TokenKind::Keyword(k) => {
                    tokens.move_next();
                    match k.keyword {
                        KeywordKind::Int => Ok(Type::Int(8, t.range)),
                        KeywordKind::Int8 => Ok(Type::Int(8, t.range)),
                        KeywordKind::Int16 => Ok(Type::Int(16, t.range)),
                        KeywordKind::Int32 => Ok(Type::Int(32, t.range)),
                        KeywordKind::Int64 => Ok(Type::Int(64, t.range)),
                        KeywordKind::Int128 => Ok(Type::Int(128, t.range)),
                        KeywordKind::Uint => Ok(Type::Uint(8, t.range)),
                        KeywordKind::Uint8 => Ok(Type::Uint(8, t.range)),
                        KeywordKind::Uint16 => Ok(Type::Uint(16, t.range)),
                        KeywordKind::Uint32 => Ok(Type::Uint(32, t.range)),
                        KeywordKind::Uint64 => Ok(Type::Uint(64, t.range)),
                        KeywordKind::Uint128 => Ok(Type::Uint(128, t.range)),
                        KeywordKind::Bool => Ok(Type::Bool(t.range)),
                        KeywordKind::Char => Ok(Type::Char(t.range)),
                        KeywordKind::Float => Ok(Type::Float(32, t.range)),
                        KeywordKind::Float32 => Ok(Type::Float(32, t.range)),
                        KeywordKind::Float64 => Ok(Type::Float(64, t.range)),
                        _ => Err(ParseError::new(&format!("{:?} is not a valid type!", k))),
                    }
                }
                TokenKind::OpenBracket => {
                    let ob = tokens.current().unwrap();
                    tokens.move_next();
                    let array_type = parse_type(tokens)?;
                    let size = if let Some(Token {
                        token_type: TokenKind::Colon,
                        ..
                    }) = tokens.peek_next()
                    {
                        let tok = tokens.current().unwrap();
                        tokens.move_next();
                        let size = expect(
                            tokens,
                            TokenKind::Literal(Literal::Integer(0, 0, default_range())),
                        )?;
                        let numeric_size = match size {
                            Token {
                                token_type: TokenKind::Literal(Literal::Integer(i, _, _)),
                                ..
                            } => *i as usize,
                            _ => {
                                return Err(ParseError::new(&format!(
                                    "Expected constant integer for array size!"
                                )));
                            }
                        };
                        Some((tok.range, numeric_size))
                    } else {
                        None
                    };
                    let cb = expect(tokens, TokenKind::CloseBracket)?;
                    Ok(Type::ArrayType(ArrayType {
                        base_type: Box::new(array_type),
                        size,
                        range: (ob.range.0, cb.range.1),
                    }))
                }
                TokenKind::OpenParen => {
                    let op = tokens.current().unwrap();
                    tokens.move_next();
                    let mut parameters = vec![];
                    while let Some(_) = tokens.peek_next() {
                        if let Some(Token {
                            token_type: TokenKind::CloseParen,
                            ..
                        }) = tokens.peek_next()
                        {
                            break;
                        }
                        let parameter_type = parse_type(tokens)?;
                        parameters.push(parameter_type);

                        match tokens.peek_next() {
                            Some(t) => match t.token_type {
                                TokenKind::Comma => tokens.move_next(),
                                TokenKind::CloseParen => {
                                    tokens.move_next();
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
                    let cp = expect(tokens, TokenKind::CloseParen)?;
                    let ret_type = if let Some(Token {
                        token_type:
                            TokenKind::Operator(Operator {
                                operator: OperatorKind::Arrow,
                                ..
                            }),
                        ..
                    }) = tokens.peek_next()
                    {
                        tokens.move_next();
                        parse_type(tokens)?
                    } else {
                        Type::Unit
                    };
                    let end = ret_type.get_range().1;

                    Ok(Type::FunctionType(FunctionType {
                        parameters,
                        return_type: Box::new(ret_type),
                        parens: (op.range.0, cp.range.1),
                        range: (op.range.0, end),
                    }))
                }
                TokenKind::Operator(Operator {
                    operator: OperatorKind::BitAnd,
                    ..
                }) => {
                    let tok = tokens.current().unwrap();
                    tokens.move_next();
                    let ttype = parse_type(tokens)?;
                    let end = ttype.get_range().1;

                    Ok(Type::ReferenceType(ReferenceType {
                        base_type: Box::new(ttype),
                        reference: tok.range,
                        range: (tok.range.0, end),
                    }))
                }
                _ => Err(ParseError::new(&format!("{:?} is not a valid type!", t))),
            };
            if let Some(Token {
                token_type:
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::Lt,
                        ..
                    }),
                ..
            }) = tokens.current()
            {
                let lt = tokens.current().unwrap();
                tokens.move_next();
                let mut type_arguments = vec![];
                while let Some(_) = tokens.current() {
                    if let Some(Token {
                        token_type:
                            TokenKind::Operator(Operator {
                                operator: OperatorKind::Gt,
                                ..
                            }),
                        ..
                    }) = tokens.current()
                    {
                        break;
                    }

                    let arg_type = parse_type(tokens)?;
                    type_arguments.push(arg_type);

                    match tokens.current() {
                        Some(t) => match t.token_type {
                            TokenKind::Comma => tokens.move_next(),
                            TokenKind::Operator(Operator {
                                operator: OperatorKind::Gt,
                                ..
                            }) => {
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
                let gt = expect(tokens, Operator::create_expect(OperatorKind::Gt))?;
                return Ok(Type::GenericType(GenericType {
                    base_type: Box::new(result?),
                    arguments: type_arguments,
                    range: (lt.range.0, gt.range.1),
                }));
            }

            result
        }
        None => Err(ParseError::new(&format!("Expected more tokens for type!"))),
    }
}

fn unary_precedence(operator: Operator) -> u8 {
    match operator.operator {
        OperatorKind::Minus
        | OperatorKind::LogicalNot
        | OperatorKind::BitNot
        | OperatorKind::Mult
        | OperatorKind::BitAnd => 14,
        _ => 0,
    }
}

fn binary_precedence(operator: Operator) -> u8 {
    match operator.operator {
        OperatorKind::Assignment
        | OperatorKind::BitAndEqual
        | OperatorKind::BitLeftEqual
        | OperatorKind::BitNotEqual
        | OperatorKind::BitOrEqual
        | OperatorKind::BitRightEqual
        | OperatorKind::BitXorEqual
        | OperatorKind::DivideEqual
        | OperatorKind::MinusEqual
        | OperatorKind::MultEqual
        | OperatorKind::PercentEqual
        | OperatorKind::PlusEqual => 2,
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

fn expect<'a>(
    tokens: &mut Cursor<&'a Token>,
    token_type: TokenKind,
) -> Result<&'a Token, ParseError> {
    match tokens.current() {
        Some(t) if std::mem::discriminant(&t.token_type) == std::mem::discriminant(&token_type) => {
            tokens.move_next();
            Ok(t)
        }
        Some(t) => {
            tokens.move_next();
            Err(ParseError::new(&format!(
                "Expected token {:?}, found token {:?}",
                token_type, t.token_type
            )))
        }
        None => {
            tokens.move_next();
            Err(ParseError::new(&format!(
                "Expected token {:?} ",
                token_type
            )))
        }
    }
}
