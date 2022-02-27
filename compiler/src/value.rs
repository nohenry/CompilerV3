use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Write},
    rc::Rc,
};

use crate::{
    ast::Type,
    lexer::Operator,
    symbol::{SymRef, Symbol},
};

#[derive(Debug, Clone)]
pub enum Value {
    None,
    UnsignedInteger(u64),
    SignedInteger(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    Array(Vec<Value>),
    String(String),
    Template(Option<Type>, HashMap<String, Value>),
    Reference(SymRef),
    DeReference(SymRef),
    // Function()
}

impl Value {
    fn can_apply_binary(&self, operator: &Operator, other: &Value) -> bool {
        match self {
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => write!(f, "{}", str),
            Value::UnsignedInteger(i) => write!(f, "{}", i),
            Value::SignedInteger(i) => write!(f, "{}", i),
            Value::Float(i) => write!(f, "{}", i),
            Value::Boolean(i) => write!(f, "{}", i),
            Value::Identifier(i) => write!(f, "{}", i),
            Value::Array(i) => write!(f, "{:?}", i),
            Value::Template(i, vals) => write!(f, "{:?}", vals),
            Value::Reference(i) => write!(f, "{:?}", i.as_ref().borrow()),
            Value::DeReference(i) => write!(f, "{:?}", i.as_ref().borrow()),
            Value::None => write!(f, "None"),
        }
    }
}
