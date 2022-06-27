#![feature(linked_list_cursors)]
use std::collections::{linked_list::Cursor, LinkedList};

use colored::{ColoredString, Colorize};
use dsl_lexer::ast::{
    ActionDecleration, ArrayInitializer, ArrayType, BinaryExpression, Expression, FunctionCall,
    FunctionDecleration, FunctionSignature, FunctionType, GenericParameters, GenericType,
    IfExpression, ImportDecleration, IndexExpression, Literal, Loop, LoopExpression, ParseNode,
    ReferenceType, SpecBody, SpecDecleration, TemplateDecleration, TemplateInitializer, Type,
    TypeDecleration, TypeSymbol, UnaryExpression, VariableDecleration,
};
use dsl_lexer::{
    default_range, Keyword, KeywordKind, Operator, OperatorKind, Range, Token, TokenKind,
};

use dsl_errors::{check, ParseError};

pub struct Parser<'a> {
    tokens: Cursor<'a, &'a Token>,
    errors: Vec<ParseError>,
    ast: ParseNode,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a LinkedList<&'a Token>) -> Parser<'a> {
        Parser {
            errors: Vec::new(),
            ast: ParseNode::Empty,
            tokens: tokens.cursor_front(),
        }
    }

    pub fn get_ast(&self) -> &ParseNode {
        &self.ast
    }

    pub fn get_errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    pub fn add_error(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    pub fn print_errors(&self) {
        for err in self.errors.iter() {
            println!("{}: {}", ColoredString::from("Error").bright_red(), err);
        }
    }

    pub fn parse_from_tokens(tokens: &'a LinkedList<&'a Token>) -> Parser<'a> {
        let mut parser = Parser::new(tokens);

        let mut statements = vec![];
        let mut current_tags = vec![];

        let start = if let Some(t) = parser.tokens.current() {
            t.range
        } else {
            default_range()
        };

        while let Some(_) = parser.tokens.current() {
            let statement = parser.parse_top_level_statement();
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

        let end = if let Some(t) = parser.tokens.current() {
            t.range
        } else if let Some(t) = parser.tokens.back() {
            t.range
        } else {
            default_range()
        };

        parser.ast = ParseNode::Expression(
            Expression::Block(statements, (start.0, end.1)),
            (start.0, end.1),
        );

        check!(parser)
    }

    fn parse_top_level_statement(&mut self) -> ParseNode {
        match self.tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::OpenBracket => self.parse_tag(),
                TokenKind::Keyword(k) => match k.keyword {
                    KeywordKind::Import => {
                        let res = self.parse_import();
                        res
                    }
                    KeywordKind::Type => {
                        self.tokens.move_next();
                        let new_type = check!(
                            self.errors,
                            self.expect(TokenKind::Ident(String::from("")), line!()),
                            ParseNode
                        );
                        let eq = check!(
                            self.errors,
                            self.expect(Operator::create_expect(OperatorKind::Assignment), line!()),
                            ParseNode
                        );
                        let current_type = self.parse_type();

                        let end = current_type.get_range().1;

                        let td = TypeDecleration {
                            type_keyword: t.range,
                            token: new_type.clone(),
                            old_type: current_type,
                            assignment: eq.range,
                            range: (t.range.0, end),
                        };

                        check!(ParseNode::TypeDecleration(td))
                    }
                    KeywordKind::Template => self.parse_template(),
                    KeywordKind::Action => self.parse_action(),
                    KeywordKind::Spec => self.parse_spec(),
                    _ => Err(ParseError::new(&format!("Unexpected keyword {:?}", t))),
                },
                TokenKind::Ident(_) => check!(self.parse_function()),
                _ => Err(ParseError::new(&format!("Unexpected token {:?}", t))),
            },
            None => check!(ParseNode::Empty),
        }
    }

    fn parse_statement(&mut self) -> ParseNode {
        match self.tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Keyword(k) => match k.keyword {
                    KeywordKind::Let => self.parse_variable_decleration(false),
                    KeywordKind::Const => self.parse_variable_decleration(true),
                    KeywordKind::Yield => {
                        let tok = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        check!(ParseNode::Yield(
                            Box::new(self.parse_expression(0)),
                            tok.range,
                        ))
                    }
                    KeywordKind::Return => {
                        let tok = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        check!(ParseNode::Return(
                            Box::new(self.parse_expression(0)),
                            tok.range,
                        ))
                    }
                    _ => {
                        let expr = self.parse_expression(0);
                        // check!(self.errors, self.expect( TokenKind::Semi, line!()), ParseNode);
                        let rng = expr.get_range();
                        check!(ParseNode::Expression(expr, rng))
                    }
                },
                TokenKind::OpenBrace => self.parse_block_statement(),
                _ => {
                    let expr = self.parse_expression(0);
                    // check!(self.errors, self.expect( TokenKind::Semi, line!()), ParseNode);
                    let rng = expr.get_range();
                    check!(ParseNode::Expression(expr, rng))
                }
            },
            None => check!(ParseNode::Empty),
        }
    }

    fn parse_template(&mut self) -> ParseNode {
        let kw = check!(
            self.errors,
            self.expect(Keyword::create_expect(KeywordKind::Template), line!()),
            ParseNode
        );
        let identifier = check!(
            self.errors,
            self.expect(TokenKind::Ident("".to_string()), line!()),
            ParseNode
        );
        let generic = if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Lt,
                    ..
                }),
            ..
        }) = self.tokens.current()
        {
            Some(Box::new(self.parse_generic()))
        } else {
            None
        };

        let ob = check!(
            self.errors,
            self.expect(TokenKind::OpenBrace, line!()),
            ParseNode
        );
        let mut fields = vec![];

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }

            let identifier = check!(
                self.errors,
                self.expect(TokenKind::Ident("".to_string()), line!()),
                ParseNode
            );
            check!(
                self.errors,
                self.expect(TokenKind::Colon, line!()),
                ParseNode
            );
            let field_type = self.parse_type();

            let ts = TypeSymbol {
                symbol_type: field_type,
                symbol: identifier.clone(),
            };
            fields.push(ts);
        }

        let cb = check!(
            self.errors,
            self.expect(TokenKind::CloseBrace, line!()),
            ParseNode
        );
        let sd = TemplateDecleration {
            struct_keyword: kw.clone().range,
            token: identifier.clone(),
            fields,
            generic,
            range: (kw.range.0, cb.range.1),
        };
        check!(ParseNode::TemplateDecleration(sd))
    }

    fn parse_action(&mut self) -> ParseNode {
        let keyword = check!(
            self.errors,
            self.expect(Keyword::create_expect(KeywordKind::Action), line!()),
            ParseNode
        );
        let generic = if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Lt,
                    ..
                }),
            ..
        }) = self.tokens.current()
        {
            Some(Box::new(self.parse_generic()))
        } else {
            None
        };

        let template_type = self.parse_type();
        let spec = match self.tokens.current() {
            Some(Token {
                token_type: TokenKind::Colon,
                ..
            }) => {
                self.tokens.move_next();
                Some(self.parse_type())
            }
            _ => None,
        };
        let left = check!(
            self.errors,
            self.expect(TokenKind::OpenBrace, line!()),
            ParseNode
        );
        let mut statements = vec![];

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            statements.push(self.parse_action_statement());

            // match self.tokens.peek() {
            //     Some(t) => match t.token_type {
            //         TokenKind::Comma => self.tokens.next(),
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

        let right = check!(
            self.errors,
            self.expect(TokenKind::CloseBrace, line!()),
            ParseNode
        );
        check!(ParseNode::ActionDecleration(ActionDecleration {
            action_keyword: keyword.range,
            template_type,
            generic,
            specification: spec,
            body: Box::new(ParseNode::Expression(
                Expression::Block(statements, (left.range.0, right.range.1)),
                (left.range.0, right.range.1),
            )),
            range: (keyword.range.0, right.range.1),
        }))
    }

    fn parse_action_statement(&mut self) -> ParseNode {
        match self.tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Ident(_) => self.parse_function(),
                _ => {
                    self.add_error(ParseError::new(&format!(
                        "Unexpected token {:?} found in action statement!",
                        t
                    )));
                    return ParseNode::Empty;
                }
            },
            None => check!(ParseNode::Empty),
        }
    }

    fn parse_spec(&mut self) -> ParseNode {
        let keyword = check!(
            self.errors,
            self.expect(Keyword::create_expect(KeywordKind::Spec), line!()),
            ParseNode
        );
        let generic = if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Lt,
                    ..
                }),
            ..
        }) = self.tokens.current()
        {
            Some(Box::new(self.parse_generic()))
        } else {
            None
        };

        let identifier = check!(
            self.errors,
            self.expect(TokenKind::Ident(String::from("")), line!()),
            ParseNode
        );
        let left = check!(
            self.errors,
            self.expect(TokenKind::OpenBrace, line!()),
            ParseNode
        );
        let mut statements = vec![];

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            statements.push(self.parse_spec_statement());
        }

        let right = check!(
            self.errors,
            self.expect(TokenKind::CloseBrace, line!()),
            ParseNode
        );
        check!(ParseNode::SpecDecleration(SpecDecleration {
            spec_keyword: keyword.range,
            identifier: identifier.clone(),
            generic,
            body: statements,
            range: (keyword.range.0, right.range.1),
        }))
    }

    fn parse_spec_statement(&mut self) -> SpecBody {
        match self.tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Ident(_) => {
                    self.tokens.move_next();
                    check!(SpecBody::Function(
                        (*t).clone(),
                        self.parse_function_type(None),
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

    fn parse_function_call(&mut self, to_be_called: Expression) -> Expression {
        let op = check!(
            self.errors,
            self.expect(TokenKind::OpenParen, line!()),
            Expression
        );
        let mut args = vec![];
        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseParen,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            args.push(self.parse_expression(0));
            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => self.tokens.move_next(),
                    TokenKind::CloseParen => {
                        break;
                    }
                    _ => {
                        self.add_error(ParseError::new(&format!(
                            "Expected comma or closing parenthesis!"
                        )));
                        return Expression::Empty;
                    }
                },
                None => {
                    self.add_error(ParseError::new(&format!("Expected token!")));
                    return Expression::Empty;
                }
            };
        }
        let cp = check!(
            self.errors,
            self.expect(TokenKind::CloseParen, line!()),
            Expression
        );
        let start = to_be_called.get_range().0;
        let (to_be_called, generic) = if let Expression::Generic(ident, args, _) = to_be_called {
            (*ident, Some(args))
        } else {
            (to_be_called, None)
        };
        let fc = FunctionCall {
            expression_to_call: Box::new(to_be_called),
            arguments: args,
            paren_tokens: (op.range.0, cp.range.1),
            generic,
            range: (start, cp.range.1),
        };
        Expression::FunctionCall(fc)
    }

    fn parse_function(&mut self) -> ParseNode {
        let ident_token = check!(
            self.errors,
            self.expect(TokenKind::Ident("".to_string()), line!()),
            ParseNode
        );
        let generic = if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Lt,
                    ..
                }),
            ..
        }) = self.tokens.current()
        {
            Some(Box::new(self.parse_generic()))
        } else {
            None
        };

        let fn_type = self.parse_function_type(None);
        let body = self.parse_statement();

        let end = body.get_range().1;

        let fd = FunctionDecleration {
            identifier: ident_token.clone(),
            function_type: fn_type,
            body: Box::new(body),
            generic,
            range: (ident_token.range.0, end),
        };

        check!(ParseNode::FunctionDecleration(fd))
    }

    fn parse_function_type(
        &mut self,
        first: Option<(&Token, &Token)>,
    ) -> Result<FunctionSignature, ParseError> {
        let mut params = vec![];

        let op = match first {
            Some((op, prm)) => {
                if Keyword::create_expect(KeywordKind::SELF) == prm.token_type {
                    params.push(TypeSymbol {
                        symbol_type: Type::SELF,
                        symbol: prm.clone(),
                    })
                } else if Keyword::create_expect(KeywordKind::Const) == prm.token_type {
                    let _ = self.expect(Keyword::create_expect(KeywordKind::SELF), line!())?;

                    params.push(TypeSymbol {
                        symbol_type: Type::ConstSelf,
                        symbol: prm.clone(),
                    })
                } else {
                    check!(
                        self.errors,
                        self.expect(TokenKind::Colon, line!()),
                        ParseNode
                    );
                    let parameter_type = self.parse_type();
                    let ts = TypeSymbol {
                        symbol_type: parameter_type,
                        symbol: prm.clone(),
                    };
                    params.push(ts);

                    match self.tokens.current() {
                        Some(t) => match t.token_type {
                            TokenKind::Comma => self.tokens.move_next(),
                            TokenKind::CloseParen => (),
                            _ => {
                                return Err(ParseError::new(&format!(
                                    "Expected comma or closing parenthesis!"
                                )))
                            }
                        },
                        None => return Err(ParseError::new(&format!("Expected token!"))),
                    };
                }
                op
            }
            None => check!(
                self.errors,
                self.expect(TokenKind::OpenParen, line!()),
                ParseNode
            ),
        };

        if let Some(
            tok @ Token {
                token_type:
                    TokenKind::Keyword(Keyword {
                        keyword: KeywordKind::SELF,
                        ..
                    }),
                ..
            },
        ) = self.tokens.current()
        {
            self.tokens.move_next();
            params.push(TypeSymbol {
                symbol_type: Type::SELF,
                symbol: (*tok).clone(),
            })
        } else if let Some(
            tok @ Token {
                token_type:
                    TokenKind::Keyword(Keyword {
                        keyword: KeywordKind::Const,
                        ..
                    }),
                ..
            },
        ) = self.tokens.current()
        {
            self.tokens.move_next();
            let otok = check!(
                self.errors,
                self.expect(Keyword::create_expect(KeywordKind::SELF), line!()),
                ParseNode
            );
            params.push(TypeSymbol {
                symbol_type: Type::ConstSelf,
                symbol: otok.clone(),
            })
        }

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseParen,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            let identifier = check!(
                self.errors,
                self.expect(TokenKind::Ident("".to_string()), line!()),
                ParseNode
            );
            check!(
                self.errors,
                self.expect(TokenKind::Colon, line!()),
                ParseNode
            );
            let parameter_type = self.parse_type();

            let ts = TypeSymbol {
                symbol_type: parameter_type,
                symbol: identifier.clone(),
            };
            params.push(ts);

            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => self.tokens.move_next(),
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

        let cp = check!(
            self.errors,
            self.expect(TokenKind::CloseParen, line!()),
            ParseNode
        );

        let ret_type = if let Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) = self.tokens.current()
        {
            self.tokens.move_next();
            self.parse_type()
        } else {
            Type::Unit
        };

        let _arrow = check!(
            self.errors,
            self.expect(Operator::create_expect(OperatorKind::Arrow), line!()),
            ParseNode
        );

        let end = ret_type.get_range().1;
        check!(FunctionSignature {
            parameters: params,
            return_type: Box::new(ret_type),
            parens: (op.range.0, cp.range.1),
            range: (op.range.0, end),
        })
    }

    fn parse_block_statement_expr(&mut self) -> Expression {
        let op = check!(
            self.errors,
            self.expect(TokenKind::OpenBrace, line!()),
            ParseNode
        );
        let mut statements = vec![];
        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }

            statements.push(self.parse_statement());

            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::CloseBrace => {
                        break;
                    }
                    _ => (),
                },
                None => return Err(ParseError::new(&format!("Expected token!"))),
            };
        }
        let cp = check!(
            self.errors,
            self.expect(TokenKind::CloseBrace, line!()),
            ParseNode
        );
        check!(Expression::Block(statements, (op.range.0, cp.range.1)))
    }

    fn parse_block_statement(&mut self) -> ParseNode {
        let expr = self.parse_block_statement_expr();
        let rng = expr.get_range();
        check!(ParseNode::Expression(expr, rng))
    }

    fn parse_operator_expression(&mut self, prev_prec: u8) -> Expression {
        let mut left = if let Some(Token {
            token_type: TokenKind::Operator(o),
            ..
        }) = self.tokens.current()
        {
            let uprec = Parser::unary_precedence(*o);
            if uprec != 0 && uprec >= prev_prec {
                self.tokens.move_next();
                let right = self.parse_expression(uprec);

                let end = right.get_range().1;
                Expression::UnaryExpression(UnaryExpression {
                    expression: Box::new(right),
                    operator: o.operator,
                    range: (o.range.0, end),
                })
            } else {
                self.parse_expression(0)
            }
        } else {
            self.parse_primary()
        };

        while let Some(t) = self.tokens.current() {
            match t {
                Token {
                    token_type: TokenKind::Operator(o),
                    ..
                } => {
                    let prec = Parser::binary_precedence(*o);
                    if prec <= prev_prec || prec == 0 {
                        break;
                    }
                    self.tokens.move_next();

                    let right = self.parse_expression(prec);
                    let right = if let (OperatorKind::Dot, Expression::Generic(tok, prms, rng)) =
                        (o.operator, &right)
                    {
                        let right = &*tok;
                        let start = left.get_range().0;
                        let end = right.get_range().1;
                        let be = BinaryExpression {
                            left: Box::new(left),
                            operator: o.operator,
                            right: right.clone(),
                            range: (start, end),
                        };
                        left = check!(Expression::Generic(
                            Box::new(Expression::BinaryExpression(be)),
                            prms.clone(),
                            rng.clone(),
                        ));

                        return self.parse_function_call(left);
                    } else {
                        right
                    };
                    let start = left.get_range().0;
                    let end = right.get_range().1;
                    let be = BinaryExpression {
                        left: Box::new(left),
                        operator: o.operator,
                        right: Box::new(right),
                        range: (start, end),
                    };
                    left = check!(Expression::BinaryExpression(be));
                }
                token => {
                    let token_type = &token.token_type;
                    let prec = Parser::postfix_precedence(token_type);
                    if prec <= prev_prec || prec == 0 {
                        break;
                    }

                    let lleft = left;
                    match token_type {
                        TokenKind::OpenParen => return self.parse_function_call(lleft),
                        TokenKind::Operator(Operator {
                            operator: OperatorKind::Lt,
                            ..
                        }) => return self.parse_function_call(lleft),
                        TokenKind::OpenBracket => {
                            let ob = check!(
                                self.errors,
                                self.expect(TokenKind::OpenBracket, line!()),
                                ParseNode
                            );
                            let value = self.parse_expression(0);
                            let cb = check!(
                                self.errors,
                                self.expect(TokenKind::CloseBracket, line!()),
                                ParseNode
                            );
                            let idx = IndexExpression {
                                index_expression: Box::new(lleft),
                                index_value: Box::new(value),
                                square_range: (ob.range.0, cb.range.1),
                            };

                            left = check!(Expression::Index(idx));
                        }
                        _ => return check!(lleft),
                    }
                }
            }
        }

        if let (
            Some(Token {
                token_type: TokenKind::Colon,
                ..
            }),
            Expression::Identifier(t),
        ) = (self.tokens.peek_next(), &left)
        {
            check!(Expression::Lambda(
                self.parse_function_type(Some((self.tokens.current().unwrap(), &t))),
                Box::new(self.parse_statement()),
            ))
        } else {
            check!(left)
        }
    }

    fn parse_expression(&mut self, prev_prec: u8) -> Expression {
        match self.tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::OpenBrace => self.parse_block_statement_expr(),
                TokenKind::Keyword(Keyword {
                    keyword: KeywordKind::If,
                    ..
                }) => {
                    self.tokens.move_next(); // eat keyword
                    let condition = self.parse_expression(0);
                    let body = self.parse_block_statement();
                    let else_clause = if let Some(Token {
                        token_type:
                            TokenKind::Keyword(Keyword {
                                keyword: KeywordKind::Else,
                                ..
                            }),
                        ..
                    }) = self.tokens.current()
                    {
                        let mut clauses = vec![];
                        while let (
                            Some(Token {
                                token_type:
                                    TokenKind::Keyword(Keyword {
                                        keyword: KeywordKind::Else,
                                        ..
                                    }),
                                range: erange,
                            }),
                            Some(Token {
                                token_type:
                                    TokenKind::Keyword(Keyword {
                                        keyword: KeywordKind::If,
                                        ..
                                    }),
                                range: irange,
                            }),
                        ) = (self.tokens.current(), self.tokens.peek_next())
                        {
                            self.tokens.move_next();
                            self.tokens.move_next();

                            let condition = self.parse_expression(0);
                            let body = self.parse_block_statement();

                            let end = body.get_range().1;
                            clauses.push(IfExpression {
                                if_token: (erange.0, irange.1),
                                condition: Box::new(condition),
                                body: Box::new(body),
                                else_clause: None,
                                range: (erange.0, end),
                            });
                        }

                        let else_clause = if let Some(Token {
                            token_type:
                                TokenKind::Keyword(Keyword {
                                    keyword: KeywordKind::Else,
                                    ..
                                }),
                            ..
                        }) = self.tokens.current()
                        {
                            let tok = self.tokens.current().unwrap();
                            self.tokens.move_next();
                            Some((tok.range, self.parse_block_statement()))
                        } else {
                            None
                        };

                        fn collect(
                            arr: &[IfExpression],
                            else_clause: Option<(Range, ParseNode)>,
                        ) -> ParseNode {
                            if arr.len() == 0 {
                                if let Some((_, body)) = else_clause {
                                    return body;
                                } else {
                                    return ParseNode::Empty;
                                }
                            } else if arr.len() == 1 {
                                if else_clause.is_none() {
                                    return ParseNode::Expression(
                                        Expression::IfExpression(arr[0].clone()),
                                        arr[0].range,
                                    );
                                }
                            }
                            let pp = collect(&arr[..arr.len() - 1], else_clause);
                            let ifexpr = arr.last().unwrap().clone();
                            let ifexpr = IfExpression {
                                else_clause: Some((pp.get_range(), Box::new(pp))),
                                ..ifexpr
                            };
                            let range = ifexpr.range;

                            return ParseNode::Expression(Expression::IfExpression(ifexpr), range);
                        }

                        let ec = collect(&clauses[..], else_clause);
                        let range = ec.get_range();
                        Some((range, Box::new(ec)))
                    } else {
                        None
                    };

                    let end = else_clause.as_ref().map_or(body.get_range().1, |f| f.0 .1);
                    check!(Expression::IfExpression(IfExpression {
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
                    self.tokens.move_next();
                    if let Some(Token {
                        token_type: TokenKind::OpenBrace,
                        ..
                    }) = self.tokens.current()
                    {
                        let body = self.parse_block_statement();
                        let range = (t.range.0, body.get_range().1);
                        check!(Expression::LoopExpression(LoopExpression {
                            keyword: t.range,
                            loop_type: Loop::Infinite(Box::new(body)),
                            range,
                        }))
                    } else {
                        let expression = self.parse_expression(0);
                        let body = self.parse_block_statement();
                        let range = (t.range.0, body.get_range().1);
                        check!(Expression::LoopExpression(LoopExpression {
                            keyword: t.range,
                            loop_type: Loop::Until(Box::new(expression), Box::new(body)),

                            range,
                        }))
                    }
                }
                _ => self.parse_operator_expression(prev_prec),
            },
            None => Err(ParseError::new(&String::from(
                "Expected some token in expression!",
            ))),
        }
    }

    fn parse_variable_decleration(&mut self, cons: bool) -> ParseNode {
        let keyword = match self.expect(Keyword::create_expect(KeywordKind::Let), line!()) {
            Ok(t) => t,
            Err(_) => check!(
                self.errors,
                self.expect(Keyword::create_expect(KeywordKind::Const), line!()),
                ParseNode
            ),
        };
        let identifier = check!(
            self.errors,
            self.expect(TokenKind::Ident("".to_string()), line!()),
            ParseNode
        );

        let var_type = match self.tokens.current() {
            Some(Token {
                token_type: TokenKind::Colon,
                ..
            }) => {
                self.tokens.move_next();
                let _type = self.parse_type();
                Some(Box::new(_type))
            }
            _ => None,
        };

        let var_initializer = match self.tokens.current() {
            Some(Token {
                token_type:
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::Assignment,
                        ..
                    }),
                ..
            }) => {
                let tok = self.tokens.current().unwrap();
                self.tokens.move_next();
                Some((Box::new(self.parse_expression(0)), tok.range))
            }
            _ => None,
        };

        // check!(self.errors, self.expect( TokenKind::Semi, line!()), ParseNode);

        let end = match &var_initializer {
            Some(s) => s.1,
            None => identifier.range,
        };
        let start = keyword.range.0;
        let vd = VariableDecleration {
            variable_type: var_type,
            possible_initializer: var_initializer,
            identifier: identifier.clone(),
            is_const: cons,
            range: (start, end.1),
        };
        check!(ParseNode::VariableDecleration(vd))
    }

    fn parse_tag(&mut self) -> ParseNode {
        let left = check!(
            self.errors,
            self.expect(TokenKind::OpenBracket, line!()),
            ParseNode
        );
        let expression = self.parse_expression(0);
        let right = check!(
            self.errors,
            self.expect(TokenKind::CloseBracket, line!()),
            ParseNode
        );
        check!(ParseNode::Tag(expression, (left.range.0, right.range.1)))
    }

    fn parse_import(&mut self) -> ParseNode {
        let keyword = check!(
            self.errors,
            self.expect(Keyword::create_expect(KeywordKind::Import), line!()),
            ParseNode
        );
        let mut modules = vec![];
        let thing = self.parse_expression(0);
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

        check!(ParseNode::Import(id))
    }

    fn parse_primary(&mut self) -> Expression {
        match self.tokens.current() {
            Some(t) => match t {
                Token {
                    token_type: TokenKind::OpenParen,
                    ..
                } => {
                    self.tokens.move_next();
                    match self.tokens.peek_next() {
                        Some(Token {
                            token_type: TokenKind::CloseParen,
                            ..
                        }) => {
                            self.tokens.move_prev();
                            check!(Expression::Lambda(
                                self.parse_function_type(None),
                                Box::new(self.parse_statement()),
                            ))
                        }
                        _ => {
                            let expr = self.parse_expression(0);
                            match expr {
                                Expression::Lambda(l, b) => check!(Expression::Lambda(l, b)),
                                _ => {
                                    check!(
                                        self.errors,
                                        self.expect(TokenKind::CloseParen, line!()),
                                        ParseNode
                                    );
                                    check!(expr)
                                }
                            }
                        }
                    }
                }
                _ => self.parse_literal(),
            },
            None => self.parse_literal(),
        }
    }

    fn parse_generic(&mut self) -> ParseNode {
        let start = check!(
            self.errors,
            self.expect(Operator::create_expect(OperatorKind::Lt), line!()),
            ParseNode
        );
        // let gt = Operator::create_expect(OperatorKind::Gt);
        let mut generic_params = vec![];
        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type:
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::Gt,
                        ..
                    }),
                ..
            }) = self.tokens.current()
            {
                break;
            }

            let type_param = check!(
                self.errors,
                self.expect(TokenKind::Ident("".to_string()), line!()),
                ParseNode
            );

            let specialization = if let Some(Token {
                token_type:
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::As,
                        ..
                    }),
                ..
            }) = self.tokens.current()
            {
                let as_tok = check!(
                    self.errors,
                    self.expect(Operator::create_expect(OperatorKind::As), line!()),
                    ParseNode
                );
                let ty = self.parse_type();
                Some(ty)
            } else {
                None
            };

            let constraints = if let Some(Token {
                token_type: TokenKind::Colon,
                ..
            }) = self.tokens.current()
            {
                Some(self.parse_generic_constraints())
            } else {
                None
            };
            generic_params.push((type_param.clone(), constraints, specialization));

            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => {
                        self.tokens.move_next();
                    }
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::Gt,
                        ..
                    }) => {
                        break;
                    }
                    _ => (),
                },
                None => {
                    self.add_error(ParseError::new(&format!("Expected token!")));
                    return ParseNode::Empty;
                }
            };
        }
        let end = check!(
            self.errors,
            self.expect(Operator::create_expect(OperatorKind::Gt), line!()),
            ParseNode
        );
        check!(ParseNode::GenericParameters(GenericParameters {
            parameters: generic_params,
            range: (start.range.0, end.range.1),
        }))
    }

    fn parse_generic_constraints(&mut self) -> Vec<Type> {
        check!(
            self.errors,
            self.expect(TokenKind::Colon, line!()),
            ParseNode
        );
        let mut constraints = vec![];
        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type:
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::Gt,
                        ..
                    }),
                ..
            }) = self.tokens.current()
            {
                break;
            }

            let constraint_type = self.parse_type();
            constraints.push(constraint_type);

            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::BitAnd,
                        ..
                    }) => {
                        self.tokens.move_next();
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
        check!(constraints)
    }

    fn parse_literal(&mut self) -> Expression {
        match self.tokens.current() {
            Some(t) => match t {
                Token {
                    token_type: TokenKind::Literal(a),
                    ..
                } => {
                    self.tokens.move_next();
                    check!(Expression::Literal(a.clone()))
                }
                Token {
                    token_type: TokenKind::OpenBracket,
                    ..
                } => self.parse_array_literal(),
                Token {
                    token_type: TokenKind::OpenBrace,
                    ..
                } => self.parse_template_initializer(None),
                Token {
                    token_type: TokenKind::Ident(_),
                    ..
                } => self.parse_ident(),

                Token {
                    token_type: TokenKind::Keyword(k),
                    ..
                } => match k.keyword {
                    KeywordKind::True => {
                        self.tokens.move_next();
                        check!(Expression::Literal(Literal::Boolean(true, t.range)))
                    }
                    KeywordKind::False => {
                        self.tokens.move_next();
                        check!(Expression::Literal(Literal::Boolean(false, t.range)))
                    }
                    KeywordKind::SELF => {
                        self.tokens.move_next();
                        check!(Expression::Literal(Literal::SELF(t.range)))
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

    fn parse_ident(&mut self) -> Expression {
        let index = self.tokens.index();
        let possible_type = self.parse_type();

        if let Some(Token {
            token_type: TokenKind::OpenBrace,
            ..
        }) = self.tokens.current()
        {
            if let Type::Empty = possible_type {
                self.add_error(ParseError::new(&format!("Type expected")));
                return Expression::Empty;
            } else {
                self.parse_template_initializer(Some(Box::new(possible_type)))
            }
        } else {
            match possible_type {
                Type::NamedType(t) => match t.token_type {
                    TokenKind::Ident(_) => Expression::Identifier(t),
                    _ => {
                        self.add_error(ParseError::new(&format!("Unexpected type in expression!")));
                        return Expression::Empty;
                    }
                },
                Type::GenericType(ty) => ty.to_expr_generic(),

                _ => {
                    // if let (Some(ind1), Some(ind2)) = (index, self.tokens.index()){
                    //     let diff = ind2 - ind1;
                    //     for _ in 0..diff {
                    //         self.tokens.move_prev();
                    //     }
                    // }
                    if let Some(ident) = self.tokens.current() {
                        self.tokens.move_next();
                        Expression::Identifier((*ident).clone())
                    } else {
                        self.add_error(ParseError::new(&format!("Expected identifer")));
                        return Expression::Empty;
                    }
                }
            }
        }
    }

    fn parse_template_initializer(&mut self, named_type: Option<Box<Type>>) -> Expression {
        let ob = check!(
            self.errors,
            self.expect(TokenKind::OpenBrace, line!()),
            ParseNode
        );
        let mut key_values = vec![];

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            let key = check!(
                self.errors,
                self.expect(TokenKind::Ident("".to_string()), line!()),
                ParseNode
            );
            let key_string = match &key.token_type {
                TokenKind::Ident(s) => s.clone(),
                _ => panic!("Shouldn't be here!"),
            };
            let value = if let Some(Token {
                token_type: TokenKind::Colon,
                ..
            }) = self.tokens.current()
            {
                self.tokens.move_next();
                self.parse_expression(0)
            } else {
                Expression::Identifier(key.clone())
            };
            key_values.push((key_string, value));

            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => self.tokens.move_next(),
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

        let cb = check!(
            self.errors,
            self.expect(TokenKind::CloseBrace, line!()),
            ParseNode
        );
        let start = named_type.as_ref().map_or(ob.range.0, |f| f.get_range().0);

        let si = TemplateInitializer {
            named_type,
            initializer_values: key_values,
            range: (start, cb.range.1),
        };
        check!(Expression::Literal(Literal::StructInitializer(si)))
    }

    fn parse_array_literal(&mut self) -> Expression {
        let ob = check!(
            self.errors,
            self.expect(TokenKind::OpenBracket, line!()),
            ParseNode
        );
        let mut values = vec![];
        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBracket,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            let value = self.parse_expression(0);
            values.push(value);
            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => self.tokens.move_next(),
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
        let cb = check!(
            self.errors,
            self.expect(TokenKind::CloseBracket, line!()),
            ParseNode
        );
        let ai = ArrayInitializer {
            elements: values,
            range: (ob.range.0, cb.range.1),
        };

        check!(Expression::Literal(Literal::Array(ai)))
    }

    fn parse_type(&mut self) -> Type {
        match self.tokens.current() {
            Some(t) => {
                let result = match t.token_type {
                    TokenKind::Ident(_) => {
                        let token = (*t).clone();
                        self.tokens.move_next();
                        check!(Type::NamedType(token))
                    }
                    TokenKind::Keyword(k) => {
                        self.tokens.move_next();
                        match k.keyword {
                            KeywordKind::Int => check!(Type::Int(32, t.range)),
                            KeywordKind::Int8 => check!(Type::Int(8, t.range)),
                            KeywordKind::Int16 => check!(Type::Int(16, t.range)),
                            KeywordKind::Int32 => check!(Type::Int(32, t.range)),
                            KeywordKind::Int64 => check!(Type::Int(64, t.range)),
                            KeywordKind::Int128 => check!(Type::Int(128, t.range)),
                            KeywordKind::Uint => check!(Type::Uint(32, t.range)),
                            KeywordKind::Uint8 => check!(Type::Uint(8, t.range)),
                            KeywordKind::Uint16 => check!(Type::Uint(16, t.range)),
                            KeywordKind::Uint32 => check!(Type::Uint(32, t.range)),
                            KeywordKind::Uint64 => check!(Type::Uint(64, t.range)),
                            KeywordKind::Uint128 => check!(Type::Uint(128, t.range)),
                            KeywordKind::Bool => check!(Type::Bool(t.range)),
                            KeywordKind::Char => check!(Type::Char(t.range)),
                            KeywordKind::Float => check!(Type::Float(32, t.range)),
                            KeywordKind::Float32 => check!(Type::Float(32, t.range)),
                            KeywordKind::Float64 => check!(Type::Float(64, t.range)),
                            _ => Err(ParseError::new(&format!("{:?} is not a valid type!", k))),
                        }
                    }
                    TokenKind::OpenBracket => {
                        let ob = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        let array_type = self.parse_type();
                        let size = if let Some(Token {
                            token_type: TokenKind::Colon,
                            ..
                        }) = self.tokens.current()
                        {
                            let tok = self.tokens.current().unwrap();
                            self.tokens.move_next();
                            let size = self.expect(
                                TokenKind::Literal(Literal::Integer(0, 0, default_range())),
                                line!(),
                            );
                            let numeric_size = match size {
                                Ok(Token {
                                    token_type: TokenKind::Literal(Literal::Integer(i, _, _)),
                                    ..
                                }) => *i as usize,
                                _ => {
                                    self.add_error(ParseError::new(&format!(
                                        "Expected constant integer for array size!"
                                    )));
                                    return Type::Empty;
                                }
                            };
                            Some((tok.range, numeric_size))
                        } else {
                            None
                        };
                        let cb = check!(
                            self.errors,
                            self.expect(TokenKind::CloseBracket, line!()),
                            ParseNode
                        );
                        check!(Type::ArrayType(ArrayType {
                            base_type: Box::new(array_type),
                            size,
                            range: (ob.range.0, cb.range.1),
                        }))
                    }
                    TokenKind::OpenParen => {
                        let op = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        let mut parameters = vec![];
                        while let Some(_) = self.tokens.peek_next() {
                            if let Some(Token {
                                token_type: TokenKind::CloseParen,
                                ..
                            }) = self.tokens.current()
                            {
                                break;
                            }
                            let parameter_type = self.parse_type();
                            parameters.push(parameter_type);

                            match self.tokens.current() {
                                Some(t) => match t.token_type {
                                    TokenKind::Comma => self.tokens.move_next(),
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
                        let cp = check!(
                            self.errors,
                            self.expect(TokenKind::CloseParen, line!()),
                            ParseNode
                        );
                        let ret_type = if let Some(Token {
                            token_type:
                                TokenKind::Operator(Operator {
                                    operator: OperatorKind::Arrow,
                                    ..
                                }),
                            ..
                        }) = self.tokens.current()
                        {
                            self.tokens.move_next();
                            self.parse_type()
                        } else {
                            Type::Unit
                        };
                        let end = ret_type.get_range().1;

                        check!(Type::FunctionType(FunctionType {
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
                        let tok = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        let ttype = self.parse_type();
                        let end = ttype.get_range().1;

                        check!(Type::ReferenceType(ReferenceType {
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
                }) = self.tokens.current()
                {
                    let lt = self.tokens.current().unwrap();
                    self.tokens.move_next();
                    let mut type_arguments = vec![];
                    let mut it = 1;
                    while let Some(_) = self.tokens.current() {
                        if let Some(Token {
                            token_type:
                                TokenKind::Operator(Operator {
                                    operator: OperatorKind::Gt,
                                    ..
                                }),
                            ..
                        }) = self.tokens.current()
                        {
                            break;
                        }

                        let arg_type = match self.parse_type() {
                            Type::Empty => {
                                for _ in 0..it {
                                    self.tokens.move_prev();
                                }
                                return result;
                            }

                            ty => ty,
                        };
                        type_arguments.push(arg_type);

                        match self.tokens.current() {
                            Some(t) => match t.token_type {
                                TokenKind::Comma => {
                                    self.tokens.move_next();
                                    it += 1;
                                }
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
                    let gt = check!(
                        self.errors,
                        self.expect(Operator::create_expect(OperatorKind::Gt), line!()),
                        ParseNode
                    );
                    return check!(Type::GenericType(GenericType {
                        base_type: Box::new(result),
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

    fn expect(&mut self, token_type: TokenKind, parser_line: u32) -> Result<&'a Token, ParseError> {
        match self.tokens.current() {
            Some(t)
                if std::mem::discriminant(&t.token_type) == std::mem::discriminant(&token_type) =>
            {
                self.tokens.move_next();
                check!(t)
            }
            Some(t) => {
                self.tokens.move_next();
                Err(ParseError::new(&format!(
                    "Expected token {:?}, found token {:?} (line: {})",
                    token_type, t.token_type, parser_line
                )))
            }
            None => {
                self.tokens.move_next();
                Err(ParseError::new(&format!(
                    "Expected token {:?} (line: {})",
                    token_type, parser_line
                )))
            }
        }
    }
}
