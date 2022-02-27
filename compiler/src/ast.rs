use std::collections::HashMap;

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
pub enum LoopExpression {
    Infinite(/* Body */ Box<ParseNode>),
    Until(
        /* Condition */ Box<Expression>,
        /* Body */ Box<ParseNode>,
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Token),
    Literal(Literal),
    BinaryExpression(
        /* Operator */ Operator,
        /* Left hand */ Box<Expression>,
        /* Right hand */ Box<Expression>,
    ),
    UnaryExpression(Operator, Box<Expression>),
    LoopExpression(LoopExpression),
    IfExpression(
        /* Condition */ Box<Expression>,
        /* Body */ Box<ParseNode>,
        /* Possible else clause */ Option<Box<ParseNode>>,
    ),
    FunctionCall(
        /* To be called */ Box<Expression>,
        /* Arguments */ Vec<Expression>,
    ),
    Lambda(FunctionType),
    Index(
        /* Indexable value */ Box<Expression>,
        /* Value indexing */ Box<Expression>,
    ),
}



pub type FunctionType = (
    /* Parameters with their type */ Vec<(Token, Type)>,
    /* Return type */ Type,
    /* Body */ Box<ParseNode>,
);
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    NamedType(Token),
    Int(u8),
    Uint(u8),
    Bool,
    Float,
    Char,
    ArrayType(
        /* Base type of the array */ Box<Type>,
        /* Possible size */ Option<usize>,
    ),
    FunctionType(
        /* Parameter types */ Vec<Type>,
        /* Return type*/ Box<Type>,
    ),
    ReferenceType(/* Reference base type*/ Box<Type>),
    GenericType(
        /* Base Type */ Box<Type>,
        /* Generic Arguments */ Vec<Type>,
    ),
    String,
    UnnamedTemplate(HashMap<String, Type>),
    None
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNode {
    None,
    Expression(Expression),
    Type(Type),
    // Identifier, Type, Initializer
    VariableDecleration(
        /* Identifier */ Token,
        /* Possible type */ Option<Box<ParseNode>>,
        /* Possible initializer */ Option<Box<ParseNode>>,
    ),
    FunctionDecleration(
        /* Identifier */ Token,
        /* Possible Generic */ Option<Box<ParseNode>>,
        /* Type */ FunctionType
    ),
    Block(/* Statements */ Vec<ParseNode>),
    Yield(Box<Expression>),
    Return(Box<Expression>),
    TemplateDecleration(
        /* Identifier of template */ Token,
        /* Fields of template */ Vec<(Token, Type)>,
        /* Possible Generic Parameter */ Option<Box<ParseNode>>,
    ),
    TypeDecleration(/* New type */ Type, /* Old type */ Type),
    ActionDecleration(
        /* Struct for action */ Type,
        /* Possible specification */ Option<Type>,
        /* Body */ Box<ParseNode>,
    ),
    GenericParameters(
        /* Parameters with their optional constraints */ Vec<(Token, Option<Vec<Type>>)>,
    ),
    Tag(/* Expression of tag */ Expression),
    TagCollection(Vec<ParseNode>, Option<Box<ParseNode>>),
    Import(
        /* Module path to import*/ Vec<Expression>,
        /* Is wildcard .* */ bool,
    ),
    Include(
        /* Module path to import*/ Vec<Expression>,
    ),
    UseKeyword(Token),
    Use()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Empty,
    Integer(u64, u8),
    Float(f64),
    Boolean(bool),

    // The following varients are formed in the parser
    Array(/* Initializer values */ Vec<Expression>),
    TemplateInitializer(
        /* Possible named type */ Option<Box<Type>>,
        /* Initializer values  */ Vec<(String, Option<Expression>)>,
    ),
    String(String),
}
