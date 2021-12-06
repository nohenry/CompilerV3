use crate::lexer::{Literal, Operator, Token};

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
#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryExpression(Operator, Box<Expression>, Box<Expression>),
    UnaryExpression(Operator, Box<Expression>),
}

#[derive(Debug)]
pub enum ParseNode {
    None,
    List,
    Expression(Expression),
    // Identifier, Type, Initializer
    VariableDecleration(Token, Option<Box<ParseNode>>, Option<Box<ParseNode>>),
}
