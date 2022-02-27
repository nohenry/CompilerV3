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
    pub fn type_from_value(&self) -> Type {
        match self {
            Value::UnsignedInteger(_) => Type::Int(32),
            Value::SignedInteger(_) => Type::Int(32),
            Value::Float(_) => Type::Float,
            Value::Boolean(_) => Type::Bool,
            Value::Array(vs) => Type::ArrayType(Box::new(vs[0].type_from_value()), Some(vs.len())),
            Value::String(_) => Type::String,
            Value::Template(t, vs) => match t {
                Some(s) => s.clone(),
                None => {
                    let mut hsh = HashMap::default();
                    for ks in vs.iter() {
                        hsh.insert(ks.0.clone(), (ks.1).type_from_value());
                    }
                    Type::UnnamedTemplate(hsh)
                }
            },
            _ => Type::None,
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
