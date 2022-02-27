use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{Expression, Literal, LoopExpression, ParseNode, Type},
    cast,
    lexer::{Operator, TokenKind},
    symbol::{SymRef, Symbol, SymbolType},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Evaluator {
    global: SymRef,
    current_insert_point: SymRef,
}

impl Evaluator {
    fn new() -> Evaluator {
        let rc = Rc::new(RefCell::new(Symbol::new(String::from("global"))));
        Evaluator {
            global: rc.clone(),
            current_insert_point: rc,
        }
    }

    pub fn add_fn(
        &mut self,
        name: &String,
        params: Vec<Type>,
        return_type: Type,
        body: fn(Vec<Value>),
    ) {
        Symbol::insert(
            self.global.clone(),
            name,
            SymbolType::NativeFunction(Box::new(body), params, return_type),
        );
    }

    pub fn evaluate_from_ast(ast: &ParseNode) {
        let mut eval = Evaluator::new();

        eval.add_fn(
            &String::from("println"),
            vec![Type::String],
            Type::None,
            |args| {
                let oo = &args[0];
                println!("{}", oo);
            },
        );

        eval.evaluate_symbols(ast);
        let entry = if let Some(main) = eval.global.borrow().find(&String::from("main")) {
            let entry = {
                let smain = main.borrow();
                let entry = match &smain.symbol_type {
                    SymbolType::Function(_, (_, _, body)) => Some(body.as_ref().clone()),
                    _ => None,
                };
                entry
            };
            eval.current_insert_point = Rc::clone(main);
            entry
        } else {
            None
        };
        match entry {
            Some(body) => {
                let done = eval.evaluate(&body);
                println!("{:?}", done);
            }
            _ => (),
        }

        println!("{:#?}", eval.global);
    }

    fn push(&mut self, symbol: SymRef) {
        self.current_insert_point = symbol;
    }

    fn pop(&mut self) {
        let parent = self.current_insert_point.borrow().parent();
        parent.and_then::<(), _>(|f| {
            self.current_insert_point = f;
            None
        });
    }

    fn evaluate_symbols(&mut self, node: &ParseNode) {
        match node {
            ParseNode::TemplateDecleration(name, fields, generic) => {
                let fart = cast!(&name.token_type, TokenKind::Ident);

                let symbol = Symbol::insert(
                    self.current_insert_point.clone(),
                    fart,
                    SymbolType::Template,
                );

                self.push(symbol);

                for arg in fields {
                    let name = cast!(&arg.0.token_type, TokenKind::Ident);

                    Symbol::insert(
                        Rc::clone(&self.current_insert_point),
                        name,
                        SymbolType::Variable(Value::None, arg.1.clone()),
                    );
                }

                self.pop();
            }
            ParseNode::FunctionDecleration(ident, generic, ftype) => {
                let fart = cast!(&ident.token_type, TokenKind::Ident);

                let symbol = Symbol::insert(
                    self.current_insert_point.clone(),
                    fart,
                    SymbolType::Function(generic.clone(), ftype.clone()),
                );

                self.push(symbol);

                for arg in &ftype.0 {
                    let name = cast!(&arg.0.token_type, TokenKind::Ident);

                    Symbol::insert(
                        Rc::clone(&self.current_insert_point),
                        name,
                        SymbolType::Variable(Value::None, arg.1.clone()),
                    );
                }

                self.evaluate_symbols(&ftype.2);

                self.pop();
            }
            ParseNode::TemplateDecleration(ident, fields, generic) => {
                let fart = cast!(&ident.token_type, TokenKind::Ident);

                Symbol::insert(
                    self.current_insert_point.clone(),
                    fart,
                    SymbolType::Template,
                );

                for field in fields {
                    let name = cast!(&field.0.token_type, TokenKind::Ident);

                    Symbol::insert(
                        Rc::clone(&self.current_insert_point),
                        name,
                        SymbolType::Variable(Value::None, field.1.clone()),
                    );
                }
            }
            ParseNode::VariableDecleration(ident, variable_type, initializer) => {
                let fart = cast!(&ident.token_type, TokenKind::Ident);

                match variable_type {
                    Some(ty) => {
                        let ty = cast!(ty.as_ref(), ParseNode::Type);
                        Symbol::insert(
                            self.current_insert_point.clone(),
                            fart,
                            SymbolType::Variable(Value::None, ty.clone()),
                        );
                    }
                    None => {
                        let ty = self.evaluate(initializer.as_ref().unwrap());
                        let ty = ty.type_from_value();
                        Symbol::insert(
                            self.current_insert_point.clone(),
                            fart,
                            SymbolType::Variable(Value::None, ty),
                        );
                    }
                }
            }
            ParseNode::Block(statements) => {
                for statement in statements {
                    self.evaluate_symbols(statement);
                }
            }
            _ => (),
        }
    }

    fn evaluate(&mut self, node: &ParseNode) -> Value {
        match node {
            ParseNode::Expression(expr) => self.evaluate_expression(&expr),
            ParseNode::FunctionDecleration(ident, _, (_, _, body)) => {
                let name = cast!(&ident.token_type, TokenKind::Ident);
                match self.current_insert_point.borrow().find(name) {
                    Some(s) => {
                        self.current_insert_point.swap(s);
                    }
                    None => (),
                }

                // let value = self.evaluate(body);

                // value
                Value::None
            }
            ParseNode::Block(statements) => {
                let mut val = Value::None;
                for statement in statements {
                    val = self.evaluate(statement);
                }
                val
            }
            ParseNode::VariableDecleration(ident, _, initializer) => {
                let name = cast!(&ident.token_type, TokenKind::Ident);
                match initializer {
                    Some(init) => {
                        let insert = self.current_insert_point.borrow();
                        let symbol = insert.find(name);
                        let sym = if let Some(sym) = symbol {
                            Some(sym.clone())
                        } else {
                            None
                        };
                        drop(insert);

                        if let Some(sym) = sym {
                            let init = self.evaluate(init);
                            let ty = {
                                let old_sym = &sym.borrow().symbol_type;
                                let ty = cast!(&old_sym, SymbolType::Variable, 2);
                                ty.1.clone()
                            };
                            sym.borrow_mut().symbol_type = SymbolType::Variable(init, ty);
                        }
                    }
                    None => (),
                }

                Value::None
            }
            _ => Value::None,
        }
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Identifier(i) => {
                let str = cast!(&i.token_type, TokenKind::Ident);
                let var = Symbol::find_in_scope(self.current_insert_point.clone(), str);
                if var.is_some() {
                    let value = var.unwrap().borrow().symbol_type.clone();
                    match value {
                        SymbolType::Variable(v, _) => v,
                        SymbolType::Function(_, _) | SymbolType::NativeFunction(_, _, _) => {
                            Value::Identifier(str.clone())
                        }
                        _ => Value::None,
                    }
                } else {
                    Value::Identifier(str.clone())
                }
            }
            Expression::Literal(literal) => self.evaluate_literal(&literal),
            Expression::IfExpression(cond, this, els) => {
                let cond = self.evaluate_expression(cond);
                match cond {
                    Value::Boolean(b) => {
                        if b {
                            self.evaluate(this)
                        } else if els.is_some() {
                            self.evaluate(&els.as_ref().unwrap())
                        } else {
                            Value::None
                        }
                    }
                    _ => Value::None,
                }
            }
            Expression::LoopExpression(lp) => {
                let mut value = Value::None;
                match lp {
                    LoopExpression::Infinite(body) => loop {
                        value = self.evaluate(body);
                    },
                    LoopExpression::Until(condition, body) => loop {
                        let cond = self.evaluate_expression(condition);
                        match cond {
                            Value::Boolean(b) => {
                                if !b {
                                    break;
                                }
                            }
                            _ => (),
                        }
                        value = self.evaluate(body);
                    },
                }
                value
            }
            Expression::FunctionCall(f, args) => {
                let func = self.evaluate_expression(f);
                match func {
                    Value::Identifier(i) => {
                        match Symbol::find_in_scope(self.current_insert_point.clone(), &i) {
                            Some(sym) => {
                                let sym = sym.clone();

                                let s = sym.borrow();
                                match &s.symbol_type {
                                    SymbolType::Function(generic, (fargs, return_type, body)) => {
                                        for arg in args.iter().zip(fargs) {
                                            let name =
                                                cast!(&(arg.1).0.token_type, TokenKind::Ident);
                                            let expr = self.evaluate_expression(arg.0);
                                            let symbol = s.find(name);
                                            match symbol {
                                                Some(s) => {
                                                    let mut s = s.borrow_mut();
                                                    match &s.symbol_type {
                                                        SymbolType::Variable(_, t) => {
                                                            s.symbol_type = SymbolType::Variable(
                                                                expr,
                                                                t.clone(),
                                                            )
                                                        }
                                                        _ => (),
                                                    }
                                                }
                                                None => (),
                                            }
                                        }
                                        self.push(sym.clone());

                                        self.evaluate(body);

                                        self.pop();

                                        Value::None
                                    }
                                    SymbolType::NativeFunction(func, _, _) => {
                                        let vals = args
                                            .iter()
                                            .map(|f| self.evaluate_expression(f))
                                            .collect();

                                        func(vals);

                                        Value::None
                                    }
                                    _ => Value::None,
                                }
                            }
                            None => Value::None,
                        }
                    }
                    _ => Value::None,
                }
            }
            Expression::BinaryExpression(oper, left, right) => {
                match oper {
                    Operator::Assignment => {
                        let right = self.evaluate_expression(right);
                        match self.evaluate_expression(left) {
                            Value::DeReference(p) => {
                                // TODO: check correct types
                                let ty = right.type_from_value();
                                p.borrow_mut().symbol_type = SymbolType::Variable(right, ty);
                                return Value::None;
                            }
                            _ => (),
                        }

                        match left.as_ref() {
                            Expression::Identifier(i) => {
                                let name = cast!(&i.token_type, TokenKind::Ident);

                                let insert = self.current_insert_point.borrow();
                                let symbol = insert.find(name);

                                symbol.and_then::<(), _>(|f| {
                                    // TODO: check correct type
                                    let ty = right.type_from_value();
                                    f.borrow_mut().symbol_type = SymbolType::Variable(right, ty);
                                    None
                                });
                            }
                            _ => (),
                        }
                        return Value::None;
                    }
                    Operator::Dot => {
                        // match left.as_ref() {
                        //     Expression::Identifier(str) => {

                        //     }
                        //     Expression::BinaryExpression(_, _, _) => self.evaluate_expression(left),
                        //     _ => ()
                        // }
                    },
                    _ => (),
                }
                let left = self.evaluate_expression(left);
                let right = self.evaluate_expression(right);

                match oper {
                    Operator::Plus => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a + b)
                        }
                        _ => Value::None,
                    },
                    Operator::Minus => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a - b)
                        }
                        _ => Value::None,
                    },
                    Operator::Mult => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a * b)
                        }
                        _ => Value::None,
                    },
                    Operator::Divide => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a / b)
                        }
                        _ => Value::None,
                    },
                    Operator::Percent => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a % b)
                        }
                        _ => Value::None,
                    },
                    Operator::BitOr => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a | b)
                        }
                        _ => Value::None,
                    },
                    Operator::BitAnd => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a & b)
                        }
                        _ => Value::None,
                    },
                    Operator::BitXor => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a ^ b)
                        }
                        _ => Value::None,
                    },
                    Operator::LogicalOr => match (left, right) {
                        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a || b),
                        _ => Value::None,
                    },
                    Operator::LogicalAnd => match (left, right) {
                        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a && b),
                        _ => Value::None,
                    },
                    Operator::LogicalXor => match (left, right) {
                        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a != b),
                        _ => Value::None,
                    },
                    Operator::Gt => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => Value::Boolean(a > b),
                        _ => Value::None,
                    },
                    Operator::Lt => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => Value::Boolean(a < b),
                        _ => Value::None,
                    },
                    Operator::GtEq | Operator::NLt => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::Boolean(a >= b)
                        }
                        _ => Value::None,
                    },
                    Operator::LtEq | Operator::NGt => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::Boolean(a <= b)
                        }
                        _ => Value::None,
                    },
                    Operator::BitLeft => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a << b)
                        }
                        _ => Value::None,
                    },
                    Operator::BitRight => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::SignedInteger(a >> b)
                        }
                        _ => Value::None,
                    },
                    Operator::Eq => match (left, right) {
                        (Value::SignedInteger(a), Value::SignedInteger(b)) => {
                            Value::Boolean(a == b)
                        }
                        _ => Value::None,
                    },

                    _ => Value::None,
                }
            }
            Expression::UnaryExpression(o, oexpr) => {
                let expr = self.evaluate_expression(oexpr);
                match o {
                    Operator::Minus => match expr {
                        Value::SignedInteger(a) => Value::SignedInteger(-a),
                        _ => Value::None,
                    },
                    Operator::LogicalNot => match expr {
                        Value::Boolean(a) => Value::Boolean(!a),
                        _ => Value::None,
                    },
                    Operator::BitNot => match expr {
                        Value::SignedInteger(a) => Value::SignedInteger(!a),
                        _ => Value::None,
                    },
                    Operator::DeRef => match expr {
                        Value::Reference(a) => Value::DeReference(a),
                        _ => Value::None,
                    },
                    Operator::BitAnd => match expr {
                        _ => match oexpr.as_ref() {
                            Expression::Identifier(i) => {
                                let name = cast!(&i.token_type, TokenKind::Ident);

                                let insert = self.current_insert_point.borrow();
                                let symbol = insert.find(name);

                                if symbol.is_some() {
                                    let symbol = symbol.unwrap();
                                    Value::Reference(symbol.clone())
                                } else {
                                    Value::None
                                }
                            }
                            _ => Value::None,
                        },
                    },
                    _ => Value::None,
                }
            }
            _ => Value::None,
        }
    }

    fn evaluate_literal(&mut self, literal: &Literal) -> Value {
        match literal {
            Literal::Integer(i, _) => Value::SignedInteger(*i as _),
            Literal::Float(i) => Value::Float(*i as _),
            Literal::Boolean(i) => Value::Boolean(*i as _),
            Literal::Array(i) => {
                Value::Array(i.iter().map(|f| self.evaluate_expression(f)).collect())
            }
            Literal::String(s) => Value::String(s.clone()),
            Literal::TemplateInitializer(n, f) => {
                match n {
                    Some(ttype) => match ttype.as_ref() {
                        Type::NamedType(t) => {
                            let name = cast!(&t.token_type, TokenKind::Ident);
                            let found =
                                Symbol::find_in_scope(self.current_insert_point.clone(), name);
                            match found {
                                Some(sym) => {
                                    // TODO: check that initilizer values match the structs declaration
                                    let mut hsh: HashMap<String, Value> = HashMap::default();
                                    for expr in f {
                                        hsh.insert(
                                            expr.0.clone(),
                                            self.evaluate_expression(&expr.1.as_ref().unwrap()),
                                        );
                                    }
                                    return Value::Template(
                                        n.as_ref().map(|f| f.as_ref().clone()),
                                        hsh,
                                    );
                                }
                                None => (),
                            }
                        }
                        _ => (),
                    },
                    None => (),
                }

                let mut hsh: HashMap<String, Value> = HashMap::default();
                for expr in f {
                    hsh.insert(
                        expr.0.clone(),
                        self.evaluate_expression(&expr.1.as_ref().unwrap()),
                    );
                }
                Value::Template(n.as_ref().map(|f| f.as_ref().clone()), hsh)
            }
            _ => Value::None,
        }
    }

    // fn evaluate_type(&mut self, eval_type: &Type) -> T
}
