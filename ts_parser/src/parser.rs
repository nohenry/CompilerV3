use std::collections::{linked_list::Cursor, LinkedList};
use std::ops::Index;

use colored::{ColoredString, Colorize};
use ts_lexer::ast::{
    ArrayInitializer, ArrayType, BinaryExpression, ClassBody, ClassDecleration, Expression,
    ForLoop, FunctionCall, FunctionDecleration, FunctionSignature, FunctionType, GenericParameters,
    GenericType, IfStatement, ImportDecleration, IndexExpression, Literal, LoopType, ParseNode,
    TemplateInitializer, Type, TypeDecleration, TypeSymbol, UnaryExpression, VariableDecleration,
    WhileLoop,
};
// use ts_lexer::new_ast::FunctionDecleration;
use ts_lexer::{
    default_range, Keyword, KeywordKind, Operator, OperatorKind, Range, Token, TokenKind,
};

use ts_errors::{pexpect, ptry, ParseError};

mod new_parser;

pub struct Parser<'a> {
    tokens: Cursor<'a, &'a Token>,
    errors: Vec<ParseError>,
    ast: ParseNode,
}

pub struct TokenStream<'a> {
    pub(self) tokens: &'a LinkedList<&'a Token>,
    pub(self) range: std::ops::Range<usize>,
}

pub struct TokenStreamIter<'a> {
    stream: TokenStream<'a>,
    index: usize,
}

impl<'a> Iterator for TokenStreamIter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stream.range.end - self.stream.range.start {
            Some(self.stream[self.index])
        } else {
            None
        }
    }
}

impl<'a> TokenStream<'a> {
    pub fn new(
        tokens: &'a LinkedList<&'a Token>,
        range: std::ops::Range<usize>,
    ) -> TokenStream<'a> {
        TokenStream { tokens, range }
    }
}

impl<'a> Index<usize> for TokenStream<'a> {
    type Output = &'a Token;

    fn index(&self, index: usize) -> &Self::Output {
        self.tokens.iter().nth(index + self.range.start).unwrap()
    }
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

    pub fn add_error<T>(&mut self, error: ParseError) -> Option<T> {
        self.errors.push(error);
        None
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

        let mut last_index = parser.tokens.index().unwrap();

        while let Some(_) = parser.tokens.current() {
            let statement = parser.parse_statement();

            if let Some(index) = parser.tokens.index() {
                if index == last_index {
                    parser.tokens.move_next();
                }

                last_index = index;
            }

            match statement {
                None => (),
                Some(tag @ ParseNode::Tag(_, _)) => {
                    current_tags.push(tag);
                }
                Some(statement) => {
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

        parser.ast = ParseNode::Block(statements, (start.0, end.1));

        parser
    }

    fn parse_statement(&mut self) -> Option<ParseNode> {
        match self.tokens.current() {
            Some(t) => match t.token_type {
                TokenKind::Keyword(k) => match k.keyword {
                    KeywordKind::Class => self.parse_class(),
                    KeywordKind::If => self.parse_if(),
                    KeywordKind::While => self.parse_while_loop(),
                    KeywordKind::For => self.parse_for_loop(),
                    KeywordKind::Function => self.parse_function(),
                    KeywordKind::Export => {
                        let tok = self.tokens.current().unwrap();
                        self.tokens.move_next();

                        let stmt = ptry!(self.parse_statement());

                        Some(ParseNode::Export(Box::new(stmt), tok.range))
                    }
                    KeywordKind::Import => {
                        let res = self.parse_import();
                        res
                    }
                    KeywordKind::Type => {
                        self.tokens.move_next();
                        let new_type = pexpect!(self, TokenKind::Ident(String::from("")));

                        let eq = pexpect!(self, Operator::create_expect(OperatorKind::Assignment));
                        let current_type = ptry!(self.parse_type());

                        let end = current_type.get_range().1;

                        let td = TypeDecleration {
                            type_keyword: t.range,
                            token: new_type.clone(),
                            old_type: current_type,
                            assignment: eq.range,
                            range: (t.range.0, end),
                        };

                        Some(ParseNode::TypeDecleration(td))
                    }
                    KeywordKind::Let => self.parse_variable_decleration(false),
                    KeywordKind::Const => self.parse_variable_decleration(true),
                    KeywordKind::Yield => {
                        let tok = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        Some(ParseNode::Yield(
                            Box::new(ptry!(self.parse_expression(0))),
                            tok.range,
                        ))
                    }
                    KeywordKind::Return => {
                        let tok = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        Some(ParseNode::Return(
                            Box::new(ptry!(self.parse_expression(0))),
                            tok.range,
                        ))
                    }
                    _ => {
                        let expr = ptry!(self.parse_expression(0));
                        let rng = expr.get_range();
                        Some(ParseNode::Expression(expr, rng))
                    }
                },
                TokenKind::OpenBrace => self.parse_block_statement(),
                _ => {
                    let expr = ptry!(self.parse_expression(0));
                    let rng = expr.get_range();
                    Some(ParseNode::Expression(expr, rng))
                }
            },
            None => None,
        }
    }

    fn parse_class(&mut self) -> Option<ParseNode> {
        let kw = pexpect!(self, Keyword::create_expect(KeywordKind::Class));
        let identifier = pexpect!(self, TokenKind::Ident("".to_string()));
        let generic = if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Lt,
                    ..
                }),
            ..
        }) = self.tokens.current()
        {
            Some(Box::new(ptry!(self.parse_generic())))
        } else {
            None
        };

        let ob = pexpect!(self, TokenKind::OpenBrace);
        let mut fields = vec![];

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }

            let identifier = pexpect!(self, TokenKind::Ident("".to_string()));

            pexpect!(self, TokenKind::Colon);

            let field_type = ptry!(self.parse_type());

            let ts = TypeSymbol {
                symbol_type: field_type,
                symbol: identifier.clone(),
            };
            fields.push(ts);
        }

        let cb = pexpect!(self, TokenKind::CloseBrace);
        let sd = ClassDecleration {
            struct_keyword: kw.clone().range,
            token: identifier.clone(),
            body: fields,
            generic,
            range: (kw.range.0, cb.range.1),
        };
        Some(ParseNode::TemplateDecleration(sd))
    }

    fn parse_class_body(&mut self) -> Option<Vec<ClassBody>> {
        let mut members = vec![];
        match self.tokens.current() {
            Some(Token {
                token_type:
                    TokenKind::Keyword(Keyword {
                        keyword: KeywordKind::Private | KeywordKind::Public | KeywordKind::Protected,
                        ..
                    }),
                ..
            }) => {
                let vis = self.tokens.current().unwrap();
                self.tokens.move_next();

                if let Some(Token {
                    token_type:
                        TokenKind::Keyword(Keyword {
                            keyword: KeywordKind::Function,
                            ..
                        }),
                    ..
                }) = self.tokens.current()
                {
                    members.push(self.parse_function());
                }
            }
            Some(_) => {}
            None => return self.add_error(ParseError::new(&"Expected tokens".into())),
        }

        members.into_iter().collect::<Option<Vec<_>>>()
    }

    fn parse_function_call(&mut self, to_be_called: Option<Expression>) -> Option<Expression> {
        let op = pexpect!(self, TokenKind::OpenParen);
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
                        return self.add_error(ParseError::new(&format!(
                            "Expected comma or closing parenthesis!"
                        )))
                    }
                },
                None => return self.add_error(ParseError::new(&format!("Expected token!"))),
            };
        }

        let cp = pexpect!(self, TokenKind::CloseParen);

        let to_be_called = ptry!(to_be_called);

        let start = to_be_called.get_range().0;
        let (to_be_called, generic) = if let Expression::Generic(ident, args, _) = to_be_called {
            (*ident, Some(args))
        } else {
            (to_be_called, None)
        };

        let args = ptry!(args.into_iter().map(|f| f).collect::<Option<_>>());

        let fc = FunctionCall {
            expression_to_call: Box::new(to_be_called),
            arguments: args,
            paren_tokens: (op.range.0, cp.range.1),
            generic,
            range: (start, cp.range.1),
        };
        Some(Expression::FunctionCall(fc))
    }

    fn parse_function(&mut self) -> Option<ParseNode> {
        self.parse_function_decleration()
            .map(|f| ParseNode::FunctionDecleration(f))
        // Some(ParseNode::FunctionDecleration())
    }

    fn parse_function_decleration(&mut self) -> Option<FunctionDecleration> {
        let keyword = pexpect!(self, Keyword::create_expect(KeywordKind::Function));
        let ident_token = pexpect!(self, TokenKind::Ident("".to_string()));
        let generic = if let Some(Token {
            token_type:
                TokenKind::Operator(Operator {
                    operator: OperatorKind::Lt,
                    ..
                }),
            ..
        }) = self.tokens.current()
        {
            if let Some(gen) = self.parse_generic() {
                Some(Box::new(gen))
            } else {
                None
            }
        } else {
            None
        };

        let fn_type = self.parse_function_type(None);
        let body = ptry!(self.parse_statement());

        let end = body.get_range().1;

        let Some(fn_type) = fn_type else {
            return None;
        };
        let fd = FunctionDecleration {
            identifier: ident_token.clone(),
            function_type: fn_type,
            body: Box::new(body),
            generic,
            range: (ident_token.range.0, end),
        };
    }

    fn try_parse_type(&mut self) -> Type {
        if let Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) = self.tokens.current()
        {
            self.tokens.move_next();
            if let Some(ty) = self.parse_type() {
                ty
            } else {
                Type::Empty
            }
        } else {
            Type::Empty
        }
    }

    fn parse_function_type(
        &mut self,
        first: Option<(&Token, &Token)>,
    ) -> Option<FunctionSignature> {
        let mut params = vec![];

        let op = match first {
            Some((op, prm)) => {
                let parameter_type = self.try_parse_type();
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
                            return self.add_error(ParseError::new(&format!(
                                "Expected comma or closing parenthesis!"
                            )))
                        }
                    },
                    None => return self.add_error(ParseError::new(&format!("Expected token!"))),
                };
                op
            }
            None => pexpect!(self, TokenKind::OpenParen),
        };

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseParen,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            let identifier = pexpect!(self, TokenKind::Ident("".to_string()));
            let parameter_type = self.try_parse_type();

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
                        return self.add_error(ParseError::new(&format!(
                            "Expected comma or closing parenthesis!"
                        )))
                    }
                },
                None => return self.add_error(ParseError::new(&format!("Expected token!"))),
            };
        }

        let cp = pexpect!(self, TokenKind::CloseParen);

        let ret_type = if let Some(Token {
            token_type: TokenKind::Colon,
            ..
        }) = self.tokens.current()
        {
            self.tokens.move_next();
            ptry!(self.parse_type())
        } else {
            Type::Unit
        };

        let end = ret_type.get_range().1;
        Some(FunctionSignature {
            parameters: params,
            return_type: Box::new(ret_type),
            parens: (op.range.0, cp.range.1),
            range: (op.range.0, end),
        })
    }

    fn parse_block_statement(&mut self) -> Option<ParseNode> {
        let op = pexpect!(self, TokenKind::OpenBrace);
        let mut statements = vec![];
        let mut last_index = 0;
        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }

            statements.push(self.parse_statement());

            let index = self.tokens.index().unwrap();
            if index == last_index {
                self.tokens.move_next();
            }

            last_index = index;
            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::CloseBrace => {
                        break;
                    }
                    _ => (),
                },
                None => return self.add_error(ParseError::new(&format!("Expected token!"))),
            };
        }
        let cp = pexpect!(self, TokenKind::CloseBrace);

        let statements = ptry!(statements.into_iter().map(|f| f).collect::<Option<_>>());

        Some(ParseNode::Block(statements, (op.range.0, cp.range.1)))
    }

    fn parse_expression(&mut self, prev_prec: u8) -> Option<Expression> {
        let mut left = if let Some(Token {
            token_type: TokenKind::Operator(o),
            ..
        }) = self.tokens.current()
        {
            let uprec = Parser::unary_precedence(*o);
            if uprec != 0 && uprec >= prev_prec {
                self.tokens.move_next();
                let right = ptry!(self.parse_expression(uprec));

                let end = right.get_range().1;
                Some(Expression::UnaryExpression(UnaryExpression {
                    expression: Box::new(right),
                    operator: o.operator,
                    range: (o.range.0, end),
                }))
            } else {
                self.parse_expression(0)
            }
        } else {
            self.parse_primary()
        };

        while let Some(t) = self.tokens.current() {
            left = match t {
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

                    let (lleft, right) = if let (Some(left), Some(right)) = (left, right) {
                        (left, right)
                    } else {
                        return None;
                    };

                    let right = if let (OperatorKind::Dot, Expression::Generic(tok, prms, rng)) =
                        (o.operator, &right)
                    {
                        let right = &*tok;
                        let start = lleft.get_range().0;
                        let end = right.get_range().1;
                        let be = BinaryExpression {
                            left: Box::new(lleft),
                            operator: o.operator,
                            right: right.clone(),
                            range: (start, end),
                        };
                        left = Some(Expression::Generic(
                            Box::new(Expression::BinaryExpression(be)),
                            prms.clone(),
                            rng.clone(),
                        ));

                        return self.parse_function_call(left);
                    } else {
                        right
                    };
                    let start = lleft.get_range().0;
                    let end = right.get_range().1;
                    let be = BinaryExpression {
                        left: Box::new(lleft),
                        operator: o.operator,
                        right: Box::new(right),
                        range: (start, end),
                    };
                    Some(Expression::BinaryExpression(be))
                }
                token => {
                    let token_type = &token.token_type;
                    let prec = Parser::postfix_precedence(token_type);
                    if prec <= prev_prec || prec == 0 {
                        break;
                    }

                    match token_type {
                        TokenKind::OpenParen => return self.parse_function_call(left),
                        TokenKind::Operator(Operator {
                            operator: OperatorKind::Lt,
                            ..
                        }) => return self.parse_function_call(left),
                        TokenKind::OpenBracket => {
                            let ob = pexpect!(self, TokenKind::OpenBracket);
                            let value = self.parse_expression(0);
                            let cb = pexpect!(self, TokenKind::CloseBracket);

                            if let (Some(value), Some(left)) = (value, left) {
                                let idx = IndexExpression {
                                    index_expression: Box::new(left),
                                    index_value: Box::new(value),
                                    square_range: (ob.range.0, cb.range.1),
                                };
                                Some(Expression::Index(idx))
                            } else {
                                None
                            }
                        }
                        _ => return left,
                    }
                }
            }
        }

        if let (
            Some(Token {
                token_type: TokenKind::Colon,
                ..
            }),
            Some(Expression::Identifier(t)),
        ) = (self.tokens.current(), &left)
        {
            let ty = self.parse_function_type(Some((self.tokens.current().unwrap(), &t)));
            let st = self.parse_statement();
            if let (Some(ty), Some(st)) = (ty, st) {
                Some(Expression::Lambda(ty, Box::new(st)))
            } else {
                None
            }
        } else {
            left
        }
    }

    fn parse_if(&mut self) -> Option<ParseNode> {
        let tok = pexpect!(self, Keyword::create_expect(KeywordKind::If));
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

                let (Some(body), Some(condition)) = (body, condition) else {
                                continue;
                            };

                let end = body.get_range().1;

                clauses.push(IfStatement {
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
                if let Some(body) = self.parse_block_statement() {
                    Some((tok.range, body))
                } else {
                    None
                }
            } else {
                None
            };

            fn collect(arr: &[IfStatement], else_clause: Option<(Range, ParseNode)>) -> ParseNode {
                if arr.len() == 0 {
                    if let Some((_, body)) = else_clause {
                        return body;
                    } else {
                        return ParseNode::Empty;
                    }
                } else if arr.len() == 1 {
                    if else_clause.is_none() {
                        return ParseNode::IfStatement(arr[0].clone());
                    }
                }
                let pp = collect(&arr[..arr.len() - 1], else_clause);
                let ifexpr = arr.last().unwrap().clone();
                let ifexpr = IfStatement {
                    else_clause: Some((pp.get_range(), Box::new(pp))),
                    ..ifexpr
                };
                return ParseNode::IfStatement(ifexpr);
            }

            let ec = collect(&clauses[..], else_clause);
            let range = ec.get_range();
            Some((range, Box::new(ec)))
        } else {
            None
        };

        let (Some(body), Some(condition)) = (body, condition) else {
                                return None;
                            };
        let end = else_clause.as_ref().map_or(body.get_range().1, |f| f.0 .1);
        Some(ParseNode::IfStatement(IfStatement {
            if_token: tok.range,
            condition: Box::new(condition),
            body: Box::new(body),
            else_clause,
            range: (tok.range.0, end),
        }))
    }

    fn parse_while_loop(&mut self) -> Option<ParseNode> {
        let tok = pexpect!(self, Keyword::create_expect(KeywordKind::While));

        let open = pexpect!(self, TokenKind::OpenParen);
        let expr = self.parse_expression(0);
        let close = pexpect!(self, TokenKind::CloseParen);

        let body = self.parse_block_statement();

        let (Some(body), Some(expr)) = (body, expr) else {
            return None;
        };

        let range = (tok.range.0, body.get_range().1);
        Some(ParseNode::WhileLoop(WhileLoop {
            body: Box::new(body),
            condition: Box::new(expr),
            while_token: tok.range,
            range,
        }))
    }

    fn parse_for_loop(&mut self) -> Option<ParseNode> {
        let tok = pexpect!(self, Keyword::create_expect(KeywordKind::For));
        let open = pexpect!(self, TokenKind::OpenParen);

        self.tokens.move_next();
        if let Some(tok) = self.tokens.peek_next() {
            match tok.token_type {
                TokenKind::Keyword(Keyword {
                    keyword: KeywordKind::Of,
                    ..
                }) => {
                    let vkey = pexpect!(
                        self,
                        Keyword::create_expect(KeywordKind::Const),
                        Keyword::create_expect(KeywordKind::Let)
                    );
                    let ident = pexpect!(self, TokenKind::Ident(String::new()));

                    let of = pexpect!(self, Keyword::create_expect(KeywordKind::Of));

                    let expr = self.parse_expression(0);
                    let body = self.parse_statement();

                    let (Some(expr), Some(body)) = (expr, body) else {
                        return None;
                    };
                    let cons = if let TokenKind::Keyword(Keyword {
                        keyword: KeywordKind::Const,
                        ..
                    }) = vkey.token_type
                    {
                        true
                    } else {
                        false
                    };
                    let end = body.get_range().1;
                    Some(ParseNode::ForLoop(ForLoop {
                        keyword: tok.range,
                        loop_type: LoopType::Of {
                            expr: Box::new(expr),
                            var: Box::new(ParseNode::VariableDecleration(VariableDecleration {
                                identifier: ident.clone(),
                                is_const: cons,
                                possible_initializer: None,
                                variable_type: None,
                                range: (vkey.range.0, ident.range.1),
                            })),
                            body: Box::new(body),
                        },
                        range: (tok.range.0, end),
                    }))
                }

                TokenKind::Operator(Operator {
                    operator: OperatorKind::In,
                    ..
                }) => {
                    let vkey = pexpect!(
                        self,
                        Keyword::create_expect(KeywordKind::Const),
                        Keyword::create_expect(KeywordKind::Let)
                    );
                    let ident = pexpect!(self, TokenKind::Ident(String::new()));

                    let of = pexpect!(self, Keyword::create_expect(KeywordKind::In));

                    let expr = self.parse_expression(0);
                    let body = self.parse_statement();

                    let (Some(expr), Some(body)) = (expr, body) else {
                        return None;
                    };
                    let cons = if let TokenKind::Keyword(Keyword {
                        keyword: KeywordKind::Const,
                        ..
                    }) = vkey.token_type
                    {
                        true
                    } else {
                        false
                    };
                    let end = body.get_range().1;
                    Some(ParseNode::ForLoop(ForLoop {
                        keyword: tok.range,
                        loop_type: LoopType::In {
                            expr: Box::new(expr),
                            var: Box::new(ParseNode::VariableDecleration(VariableDecleration {
                                identifier: ident.clone(),
                                is_const: cons,
                                possible_initializer: None,
                                variable_type: None,
                                range: (vkey.range.0, ident.range.1),
                            })),
                            body: Box::new(body),
                        },
                        range: (tok.range.0, end),
                    }))
                }
                _ => {
                    let init = if let Some(tok) = self.tokens.current() {
                        match tok.token_type {
                            TokenKind::Keyword(Keyword {
                                keyword: KeywordKind::Let,
                                ..
                            }) => self.parse_variable_decleration(false),
                            TokenKind::Keyword(Keyword {
                                keyword: KeywordKind::Const,
                                ..
                            }) => self.parse_variable_decleration(true),
                            TokenKind::Semi => Some(ParseNode::Empty),
                            _ => None,
                        }
                    } else {
                        return self.add_error(ParseError::new(&"Expexted tokens!".into()));
                    };

                    let _ = pexpect!(self, TokenKind::Semi);

                    let cond = if let Some(tok) = self.tokens.current() {
                        match tok.token_type {
                            TokenKind::Semi => Some(Expression::Empty),
                            _ => self.parse_expression(0),
                        }
                    } else {
                        return self.add_error(ParseError::new(&"Expexted tokens!".into()));
                    };

                    let _ = pexpect!(self, TokenKind::Semi);

                    let update = self.parse_expression(0);

                    let close = pexpect!(self, TokenKind::CloseParen);

                    let body = self.parse_statement();

                    let (Some(cond), Some(init), Some(update), Some(body)) = (cond, init, update, body) else {
                        return None
                    };

                    let end = body.get_range().1;
                    Some(ParseNode::ForLoop(ForLoop {
                        keyword: tok.range,
                        loop_type: LoopType::Condition {
                            condition: Box::new(cond),
                            init: Box::new(init),
                            update: Box::new(update),
                            body: Box::new(body),
                        },
                        range: (tok.range.0, end),
                    }))
                }
            }
        } else {
            return self.add_error(ParseError::new(&format!("Expected tokens!")));
        }
    }

    fn parse_variable_decleration(&mut self, cons: bool) -> Option<ParseNode> {
        let keyword = match self.expect(Keyword::create_expect(KeywordKind::Let), line!()) {
            Ok(t) => t,
            Err(_) => pexpect!(self, Keyword::create_expect(KeywordKind::Const)),
        };
        let identifier = pexpect!(self, TokenKind::Ident("".to_string()));

        let var_type = match self.tokens.current() {
            Some(Token {
                token_type: TokenKind::Colon,
                ..
            }) => {
                self.tokens.move_next();
                let ptype = self.parse_type();

                ptype.map(|f| Box::new(f))
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
                let pexpr = self.parse_expression(0);

                pexpr.map(|f| (Box::new(f), tok.range))
            }
            _ => None,
        };

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

        Some(ParseNode::VariableDecleration(vd))
    }

    fn parse_import(&mut self) -> Option<ParseNode> {
        let keyword = pexpect!(self, Keyword::create_expect(KeywordKind::Import));

        let mut modules = vec![];
        let thing = ptry!(self.parse_expression(0));

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

        Some(ParseNode::Import(id))
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        match self.tokens.current() {
            Some(t) => match t {
                Token {
                    token_type: TokenKind::OpenParen,
                    ..
                } => {
                    self.tokens.move_next();
                    match self.tokens.current() {
                        Some(Token {
                            token_type: TokenKind::CloseParen,
                            ..
                        }) => {
                            self.tokens.move_prev();
                            let ty = self.parse_function_type(None);
                            let st = self.parse_statement();
                            let (Some(ty), Some(st)) = (ty, st) else {
                                    return None;
                                };

                            Some(Expression::Lambda(ty, Box::new(st)))
                        }
                        _ => {
                            let expr = self.parse_expression(0);
                            match expr {
                                Some(Expression::Lambda(l, b)) => Some(Expression::Lambda(l, b)),
                                Some(e) => {
                                    pexpect!(self, TokenKind::CloseParen);
                                    Some(e)
                                }
                                None => {
                                    pexpect!(self, TokenKind::CloseParen);
                                    None
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

    fn parse_generic(&mut self) -> Option<ParseNode> {
        let start = pexpect!(self, Operator::create_expect(OperatorKind::Lt));
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

            let type_param = pexpect!(self, TokenKind::Ident("".to_string()));

            let specialization = if let Some(Token {
                token_type:
                    TokenKind::Operator(Operator {
                        operator: OperatorKind::As,
                        ..
                    }),
                ..
            }) = self.tokens.current()
            {
                let as_tok = pexpect!(self, Operator::create_expect(OperatorKind::As));
                let ty = self.parse_type();
                ty
            } else {
                None
            };

            let constraints = if let Some(Token {
                token_type: TokenKind::Colon,
                ..
            }) = self.tokens.current()
            {
                self.parse_generic_constraints()
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
                    return self.add_error(ParseError::new(&format!("Expected token!")));
                }
            };
        }
        let end = pexpect!(self, Operator::create_expect(OperatorKind::Gt));
        Some(ParseNode::GenericParameters(GenericParameters {
            parameters: generic_params,
            range: (start.range.0, end.range.1),
        }))
    }

    fn parse_generic_constraints(&mut self) -> Option<Vec<Type>> {
        pexpect!(self, TokenKind::Colon);
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
                None => return self.add_error(ParseError::new(&format!("Expected token!"))),
            };
        }

        let constraints = ptry!(constraints.into_iter().map(|f| f).collect::<Option<_>>());

        Some(constraints)
    }

    fn parse_literal(&mut self) -> Option<Expression> {
        match self.tokens.current() {
            Some(t) => match t {
                Token {
                    token_type: TokenKind::Literal(a),
                    ..
                } => {
                    self.tokens.move_next();
                    Some(Expression::Literal(a.clone()))
                }
                Token {
                    token_type: TokenKind::OpenBracket,
                    ..
                } => self.parse_array_literal(),
                Token {
                    token_type: TokenKind::OpenBrace,
                    ..
                } => self.parse_interface_initializer(None),
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
                        Some(Expression::Literal(Literal::Boolean(true, t.range)))
                    }
                    KeywordKind::False => {
                        self.tokens.move_next();
                        Some(Expression::Literal(Literal::Boolean(false, t.range)))
                    }
                    KeywordKind::Null => {
                        self.tokens.move_next();
                        Some(Expression::Literal(Literal::Null(t.range)))
                    }
                    KeywordKind::Undefined => {
                        self.tokens.move_next();
                        Some(Expression::Literal(Literal::Undefined(t.range)))
                    }
                    _ => self.add_error(ParseError::new(&format!(
                        "Keyword {:?} is not a valid literal!",
                        k
                    ))),
                },
                _ => self.add_error(ParseError::new(&"Unkown literal value!".to_string())),
            },
            None => self.add_error(ParseError::new(&"Unkown literal value!".to_string())),
        }
    }

    fn parse_ident(&mut self) -> Option<Expression> {
        let possible_type = self.parse_type();

        if let Some(Token {
            token_type: TokenKind::OpenBrace,
            ..
        }) = self.tokens.current()
        {
            if let Some(ty) = possible_type {
                self.parse_interface_initializer(Some(Box::new(ty)))
            } else {
                self.add_error(ParseError::new(&format!("Type expected")))
            }
        } else {
            match possible_type {
                Some(Type::NamedType(t)) => match t.token_type {
                    TokenKind::Ident(_) => Some(Expression::Identifier(t)),
                    _ => {
                        self.add_error(ParseError::new(&format!("Unexpected type in expression!")))
                    }
                },
                Some(Type::GenericType(ty)) => Some(ty.to_expr_generic()),
                _ => {
                    if let Some(ident) = self.tokens.current() {
                        self.tokens.move_next();
                        Some(Expression::Identifier((*ident).clone()))
                    } else {
                        self.add_error(ParseError::new(&format!("Expected identifer")))
                    }
                }
            }
        }
    }

    fn parse_interface_initializer(&mut self, named_type: Option<Box<Type>>) -> Option<Expression> {
        let ob = pexpect!(self, TokenKind::OpenBrace);
        let mut key_values = vec![];

        while let Some(_) = self.tokens.current() {
            if let Some(Token {
                token_type: TokenKind::CloseBrace,
                ..
            }) = self.tokens.current()
            {
                break;
            }
            let key = pexpect!(self, TokenKind::Ident("".to_string()));
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
                Some(Expression::Identifier(key.clone()))
            };

            key_values.push(value.map(|f| (key_string, f)));

            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => self.tokens.move_next(),
                    TokenKind::CloseBrace => {
                        break;
                    }
                    _ => {
                        return self.add_error(ParseError::new(&format!(
                            "Expected comma or closing brace!"
                        )))
                    }
                },
                None => return self.add_error(ParseError::new(&format!("Expected token!"))),
            };
        }

        let cb = pexpect!(self, TokenKind::CloseBrace);
        let start = named_type.as_ref().map_or(ob.range.0, |f| f.get_range().0);

        let key_values = ptry!(key_values.into_iter().map(|f| f).collect::<Option<_>>());

        let si = TemplateInitializer {
            named_type,
            initializer_values: key_values,
            range: (start, cb.range.1),
        };
        Some(Expression::Literal(Literal::StructInitializer(si)))
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let ob = pexpect!(self, TokenKind::OpenBracket);
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
            if let Some(value) = value {
                values.push(value);
            }

            match self.tokens.current() {
                Some(t) => match t.token_type {
                    TokenKind::Comma => self.tokens.move_next(),
                    TokenKind::CloseBracket => {
                        break;
                    }
                    _ => {
                        return self.add_error(ParseError::new(&format!(
                            "Expected comma or closing bracket!"
                        )))
                    }
                },
                None => return self.add_error(ParseError::new(&format!("Expected token!"))),
            };
        }
        let cb = pexpect!(self, TokenKind::CloseBracket);
        let ai = ArrayInitializer {
            elements: values,
            range: (ob.range.0, cb.range.1),
        };

        Some(Expression::Literal(Literal::Array(ai)))
    }

    fn parse_type(&mut self) -> Option<Type> {
        match self.tokens.current() {
            Some(t) => {
                let mut result = match t.token_type {
                    TokenKind::Ident(_) => {
                        let token = (*t).clone();
                        self.tokens.move_next();
                        Some(Type::NamedType(token))
                    }
                    TokenKind::Keyword(k) => {
                        self.tokens.move_next();
                        match k.keyword {
                            KeywordKind::Number => Some(Type::Number((*t).clone())),
                            KeywordKind::Boolean => Some(Type::Boolean((*t).clone())),
                            KeywordKind::String => Some(Type::String((*t).clone())),
                            KeywordKind::Null => Some(Type::Null((*t).clone())),
                            KeywordKind::Undefined => Some(Type::Undefined((*t).clone())),
                            _ => None,
                        }
                    }
                    TokenKind::OpenBracket => {
                        let ob = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        let array_type = ptry!(self.parse_type());
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
                                    return self.add_error(ParseError::new(&format!(
                                        "Expected constant integer for array size!"
                                    )))
                                }
                            };
                            Some((tok.range, numeric_size))
                        } else {
                            None
                        };
                        let cb = pexpect!(self, TokenKind::CloseBracket);
                        Some(Type::ArrayType(ArrayType {
                            base_type: Box::new(array_type),
                            size,
                            range: (ob.range.0, cb.range.1),
                        }))
                    }
                    TokenKind::OpenParen => {
                        let op = self.tokens.current().unwrap();
                        self.tokens.move_next();
                        let mut parameters = vec![];
                        while let Some(_) = self.tokens.current() {
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
                                        return self.add_error(ParseError::new(&format!(
                                            "Expected comma or closing parenthesis!"
                                        )))
                                    }
                                },
                                None => {
                                    return self
                                        .add_error(ParseError::new(&format!("Expected token!")))
                                }
                            };
                        }
                        let cp = pexpect!(self, TokenKind::CloseParen);
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
                            ptry!(self.parse_type())
                        } else {
                            Type::Unit
                        };
                        let end = ret_type.get_range().1;

                        let parameters = ptry!(parameters
                            .into_iter()
                            .map(|f| f)
                            .collect::<Option<Vec<Type>>>());

                        Some(Type::FunctionType(FunctionType {
                            parameters,
                            return_type: Box::new(ret_type),
                            parens: (op.range.0, cp.range.1),
                            range: (op.range.0, end),
                        }))
                    }
                    _ => {
                        return self
                            .add_error(ParseError::new(&format!("{:?} is not a valid type!", t)))
                    }
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
                            None => {
                                for _ in 0..it {
                                    self.tokens.move_prev();
                                }
                                return result;
                            }
                            Some(ty) => ty,
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
                                    return self.add_error(ParseError::new(&format!(
                                        "Expected comma or closing bracket!"
                                    )))
                                }
                            },
                            None => {
                                return self.add_error(ParseError::new(&format!("Expected token!")))
                            }
                        };
                    }
                    let gt = pexpect!(self, Operator::create_expect(OperatorKind::Gt));

                    let Some(res) = result else {
                        return None
                    };
                    result = Some(Type::GenericType(GenericType {
                        base_type: Box::new(res),
                        arguments: type_arguments,
                        range: (lt.range.0, gt.range.1),
                    }));
                }

                if let Some(Token {
                    token_type:
                        TokenKind::Operator(Operator {
                            operator: OperatorKind::BitOr,
                            ..
                        }),
                    ..
                }) = self.tokens.current()
                {
                    let mut vc = vec![result];
                    while let Some(Token {
                        token_type:
                            TokenKind::Operator(Operator {
                                operator: OperatorKind::BitOr,
                                ..
                            }),
                        ..
                    }) = self.tokens.current()
                    {
                        self.tokens.move_next();
                        let ty = self.parse_type();
                        vc.push(ty);
                    }

                    let vc: Option<_> = vc.into_iter().map(|f| f).collect();
                    let Some(vc) = vc else {
                        return None;
                    };
                    result = Some(Type::Union(vc));
                }

                result
            }
            None => {
                return self.add_error(ParseError::new(&format!("Expected more tokens for type!")))
            }
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

    fn expect_multi(
        &mut self,
        token_type: &[TokenKind],
        parser_line: u32,
    ) -> Result<&'a Token, ParseError> {
        let tok = token_type.iter().find(|tok| match self.tokens.current() {
            Some(t) if std::mem::discriminant(&t.token_type) == std::mem::discriminant(&tok) => {
                true
            }
            Some(t) => false,
            None => false,
        });
        if let Some(tok) = tok {
            let tok = self.tokens.current().unwrap();
            self.tokens.move_next();
            Ok(*tok)
        } else {
            if token_type.len() > 1 {
                let strs: String = token_type
                    .iter()
                    .map(|f| format!("{:?}", std::mem::discriminant(f)))
                    .collect();
                let Some(tok) = self.tokens.current() else {
                    return Err(ParseError::new(&format!("Expected tokens {}", strs)))
                };
                Err(ParseError::new(&format!(
                    "Expected tokens {}, found token {:?} (line: {})",
                    strs,
                    std::mem::discriminant(&tok.token_type),
                    parser_line
                )))
            } else {
                let Some(tok) = self.tokens.current() else {
                return Err(ParseError::new(&format!("Expected token {:?}", std::mem::discriminant(&token_type[0]))))
            };
                Err(ParseError::new(&format!(
                    "Expected token {:?}, found token {:?} (line: {})",
                    std::mem::discriminant(&token_type[0]),
                    std::mem::discriminant(&tok.token_type),
                    parser_line
                )))
            }
        }
    }

    fn expect(&mut self, token_type: TokenKind, parser_line: u32) -> Result<&'a Token, ParseError> {
        match self.tokens.current() {
            Some(t)
                if std::mem::discriminant(&t.token_type) == std::mem::discriminant(&token_type) =>
            {
                self.tokens.move_next();
                Ok(t)
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
