use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Expression, Literal, ParseNode},
    cast,
    lexer::{Operator, TokenKind},
    symbol::{Symbol, SymbolType},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Evaluator {
    global: Rc<RefCell<Symbol>>,
    current_insert_point: Rc<RefCell<Symbol>>,
}

impl Evaluator {
    fn new() -> Evaluator {
        let rc = Rc::new(RefCell::new(Symbol::new(String::from("global"))));
        Evaluator {
            global: rc.clone(),
            current_insert_point: rc,
        }
    }

    pub fn evaluate_from_ast(ast: &ParseNode) {
        let eval = Evaluator::new();

        eval.evaluate_symbols(ast);
        if let Some(main) = eval.global.borrow().find(&String::from("main")) {
            match &main.borrow().symbol_type {
                SymbolType::Function(body) => {
                    let done = eval.evaluate(body);
                    println!("{:?}", done);
                }
                _ => (),
            }
        }

        println!("{:#?}", eval.global);
    }

    fn evaluate_symbols(&self, node: &ParseNode) {
        match node {
            ParseNode::FunctionDecleration(ident, generic, (_, _, body)) => {
                let fart = cast!(&ident.token_type, TokenKind::Ident);
                self.current_insert_point
                    .borrow_mut()
                    .insert(fart, SymbolType::Function(body.clone()))
            }
            ParseNode::TemplateDecleration(ident, fields, generic) => {
                let fart = cast!(&ident.token_type, TokenKind::Ident);
                self.current_insert_point
                    .borrow_mut()
                    .insert(fart, SymbolType::Template);

                for field in fields {
                    let name = cast!(&field.0.token_type, TokenKind::Ident);
                    self.current_insert_point
                        .borrow_mut()
                        .insert(name, SymbolType::Variable);
                }
            }
            ParseNode::VariableDecleration(ident, variable_type, initializer) => {
                let fart = cast!(&ident.token_type, TokenKind::Ident);
                self.current_insert_point
                    .borrow_mut()
                    .insert(fart, SymbolType::Variable)
            }
            ParseNode::Block(statements) => {
                for statement in statements {
                    self.evaluate_symbols(statement);
                }
            }
            _ => (),
        }
    }

    fn evaluate(&self, node: &ParseNode) -> Value {
        match node {
            ParseNode::Expression(expr) => self.evaluate_expression(&expr),
            ParseNode::FunctionDecleration(_, _, (_, _, body)) => self.evaluate(body),
            ParseNode::Block(statements) => {
                let mut val = Value::None;
                for statement in statements {
                    val = self.evaluate(statement);
                }
            val
            }
            _ => Value::None,
        }
    }

    fn evaluate_expression(&self, expression: &Expression) -> Value {
        match expression {
            Expression::Literal(literal) => self.evaluate_literal(&literal),
            Expression::BinaryExpression(oper, left, right) => {
                let left = self.evaluate_expression(left);
                let right = self.evaluate_expression(right);

                match oper {
                    Operator::Plus => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a + b)
                        }
                        _ => Value::None,
                    },
                    _ => Value::None,
                }
            }
            _ => Value::None,
        }
    }

    fn evaluate_literal(&self, literal: &Literal) -> Value {
        match literal {
            Literal::Integer(i, _) => Value::SignedInteger(*i as _),
            _ => Value::None,
        }
    }
}
