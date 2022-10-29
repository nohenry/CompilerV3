use std::collections::{linked_list::Cursor, LinkedList};

use colored::{ColoredString, Colorize};
use ts_errors::{pexpect, ptry, ParseError};
use ts_lexer::{
    default_range,
    new_ast::{Expression, Literal, SourceFile, Statement},
    KeywordKind, Operator, OperatorKind, Token, TokenKind,
};

pub struct Parser<'a> {
    tokens: Cursor<'a, &'a Token>,
    errors: Vec<ParseError>,
    file: SourceFile,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a LinkedList<&'a Token>) -> Parser<'a> {
        Parser {
            errors: Vec::new(),
            tokens: tokens.cursor_front(),
            file: SourceFile {
                statements: Vec::new(),
            },
        }
    }

    pub fn get_file(&self) -> &SourceFile {
        &self.file
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
        // let mut current_tags = vec![];

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
                Some(s) => statements.push(s),
            }
        }

        let end = if let Some(t) = parser.tokens.current() {
            t.range
        } else if let Some(t) = parser.tokens.back() {
            t.range
        } else {
            default_range()
        };

        // parser.ast = ParseNode::Block(statements, (start.0, end.1));

        parser
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.tokens.current() {
            Some(t) => match t.token_type {
                _ => {
                    let expr = ptry!(self.parse_expression(0));
                    Some(Statement::Expression(Box::new(expr)))
                }
            },
            None => None,
        }
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

                Some(Expression::UnaryExpression {
                    op: o.operator,
                    expr: Box::new(right),
                })
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

                    let (left, right) = if let (Some(left), Some(right)) = (left, right) {
                        (left, right)
                    } else {
                        return None;
                    };

                    // let right = if let (OperatorKind::Dot, Expression::Generic(tok, prms, rng)) =
                    //     (o.operator, &right)
                    // {
                    //     let right = &*tok;
                    //     let start = lleft.get_range().0;
                    //     let end = right.get_range().1;
                    //     let be = BinaryExpression {
                    //         left: Box::new(lleft),
                    //         operator: o.operator,
                    //         right: right.clone(),
                    //         range: (start, end),
                    //     };
                    //     left = Some(Expression::Generic(
                    //         Box::new(Expression::BinaryExpression(be)),
                    //         prms.clone(),
                    //         rng.clone(),
                    //     ));

                    //     return self.parse_function_call(left);
                    // } else {
                    //     right
                    // };
                    // let start = lleft.get_range().0;
                    // let end = right.get_range().1;
                    // let be = BinaryExpression {
                    //     left: Box::new(lleft),
                    //     operator: o.operator,
                    //     right: Box::new(right),
                    //     range: (start, end),
                    // };
                    // Some(Expression::BinaryExpression(be))

                    Some(Expression::BinaryExpression {
                        left: Box::new(left),
                        op: o.operator,
                        right: Box::new(right),
                    })
                }
                token => {
                    let token_type = &token.token_type;
                    let prec = Parser::postfix_precedence(token_type);
                    if prec <= prev_prec || prec == 0 {
                        break;
                    }

                    match token_type {
                        // TokenKind::OpenParen => return self.parse_function_call(left),
                        // TokenKind::Operator(Operator {
                        //     operator: OperatorKind::Lt,
                        //     ..
                        // }) => return self.parse_function_call(left),
                        TokenKind::OpenBracket => {
                            let ob = pexpect!(self, TokenKind::OpenBracket);
                            let value = self.parse_expression(0);
                            let cb = pexpect!(self, TokenKind::CloseBracket);

                            if let (Some(value), Some(left)) = (value, left) {
                                Some(Expression::Member {
                                    expr: Box::new(left),
                                    member: Box::new(value),
                                })
                            } else {
                                None
                            }
                        }
                        _ => return left,
                    }
                }
            }
        }

        left
        // if let (
        //     Some(Token {
        //         token_type: TokenKind::Colon,
        //         ..
        //     }),
        //     Some(Expression::Identifier(t)),
        // ) = (self.tokens.current(), &left)
        // {
        //     let ty = self.parse_function_type(Some((self.tokens.current().unwrap(), &t)));
        //     let st = self.parse_statement();
        //     if let (Some(ty), Some(st)) = (ty, st) {
        //         Some(Expression::Lambda(ty, Box::new(st)))
        //     } else {
        //         None
        //     }
        // } else {
        //     left
        // }
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        match self.tokens.current() {
            Some(t) => match t {
                Token {
                    token_type: TokenKind::OpenParen,
                    ..
                } => {
                    self.tokens.move_next();
                    // match self.tokens.current() {
                    //     Some(Token {
                    //         token_type: TokenKind::CloseParen,
                    //         ..
                    //     }) => {
                    //         self.tokens.move_prev();
                    //         let ty = self.parse_function_type(None);
                    //         let st = self.parse_statement();
                    //         let (Some(ty), Some(st)) = (ty, st) else {
                    //                 return None;
                    //             };

                    //         Some(Expression::Lambda(ty, Box::new(st)))
                    //     }
                    //     _ => {
                    let expr = self.parse_expression(0);
                    match expr {
                        // Some(Expression::Lambda(l, b)) => Some(Expression::Lambda(l, b)),
                        Some(e) => {
                            pexpect!(self, TokenKind::CloseParen);
                            Some(e)
                        }
                        None => {
                            pexpect!(self, TokenKind::CloseParen);
                            None
                        } // }
                          // }
                    }
                }
                _ => self.parse_literal(),
            },
            None => self.parse_literal(),
        }
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
                // Token {
                //     token_type: TokenKind::OpenBracket,
                //     ..
                // } => self.parse_array_literal(),
                // Token {
                //     token_type: TokenKind::OpenBrace,
                //     ..
                // } => self.parse_interface_initializer(None),
                // Token {
                //     token_type: TokenKind::Ident(_),
                //     ..
                // } => self.parse_ident(),
                Token {
                    token_type: TokenKind::Keyword(k),
                    ..
                } => match k.keyword {
                    KeywordKind::True => {
                        self.tokens.move_next();
                        Some(Expression::Literal(Literal::Boolean(true)))
                    }
                    KeywordKind::False => {
                        self.tokens.move_next();
                        Some(Expression::Literal(Literal::Boolean(false)))
                    }
                    KeywordKind::Null => {
                        self.tokens.move_next();
                        Some(Expression::Literal(Literal::Null(t.range)))
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
    // fn parse_type(&mut self) -> Option<Type> {
    // }

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
