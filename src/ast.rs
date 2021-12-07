use crate::lexer::{Operator, Token};

// pub struct VariableDecleration {
//     identifier: String,
//     variable_type: Option<Box<ParseNode>>,
//     initializer: Option<Box<ParseNode>>,
// }

// #[derive(Debug)]
// pub struct BinaryExpression {
//     left: Box<ParseNode>,
//     operator: Operator,
//     right: Box<ParseNode>,
// }
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryExpression(Operator, Box<Expression>, Box<Expression>),
    UnaryExpression(Operator, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    NamedType(Token),
    Int(u8),
    Uint(u8),
    Bool,
    Float,
    Char,
    ArrayType(Box<Type>, Option<usize>),
    FunctionType(Vec<Type>, Box<Type>),
    ReferenceType(Box<Type>)
}

#[derive(Debug)]
pub enum ParseNode {
    None,
    Expression(Expression),
    Type(Type),
    // Identifier, Type, Initializer
    VariableDecleration(Token, Option<Box<ParseNode>>, Option<Box<ParseNode>>),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Empty,
    Integer(u64, u8),
    Float(f64),

    // The following varients are formed in the parser
    Array(Vec<Expression>),
    TemplateInitializer(Option<Box<Type>>, Vec<(String, Option<Expression>)>)
}
