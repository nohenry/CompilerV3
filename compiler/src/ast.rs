use crate::lexer::{OperatorKind, Range, Token};

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
pub struct BinaryExpression {
    left: Box<Expression>,
    operator: OperatorKind,
    right: Box<Expression>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    expression: Box<Expression>,
    operator: OperatorKind,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    if_token: Range,
    condition: Box<Expression>,
    body: Box<ParseNode>,
    else_clause: Option<(
        /* else token */ Range,
        /* else clause body*/ Box<ParseNode>,
    )>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    expression_to_call: Box<Expression>,
    arguments: Vec<ParseNode>,
    paren_tokens: Range,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Token),
    Literal(Literal),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    LoopExpression(LoopExpression),
    IfExpression(IfExpression),
    FunctionCall(FunctionCall),
    Lambda(FunctionSignature),
    Index(
        /* Indexable value */ Box<Expression>,
        /* Value indexing */ Box<Expression>,
    ),
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Expression {
//     kind: ExpressionKind,
//     range: Range,
// }

#[derive(Debug, Clone, PartialEq)]
struct IdentSymbol {
    identifier: Token,
}

#[derive(Debug, Clone, PartialEq)]
struct TypeSymbol {
    symbol: Box<ParseNode>,
    colon: Range,
    symbol_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    parameters: Vec<ParseNode>,
    type_arrow: Option<Punctuation>,
    return_type: Box<Type>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    base_type: Box<Type>,
    size: Option<(/* Colon */ Range, /* Size */ usize)>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceType {
    reference: Range,
    base_type: Box<Type>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericType {
    base_type: Box<Type>,
    arguments: Vec<ParseNode>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit(Range),
    NamedType(Token),
    Int(u8, Range),
    Uint(u8, Range),
    Bool(Range),
    Float(Range),
    Char(Range),
    ArrayType(ArrayType),
    FunctionType(FunctionSignature),
    ReferenceType(ReferenceType),
    GenericType(GenericType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    Comma,
    FunctionArrow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecleration {
    let_token: Range,
    symbol: Box<ParseNode>,
    possible_initializer: Option<(
        /* Initializer */ Box<Expression>,
        /* Position of assignment operator*/ Range,
    )>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecleration {
    identifier: Token,
    possible_generic: Option<(Box<ParseNode>,)>,
    function_type: FunctionSignature,
    body: Box<ParseNode>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateDecleration {
    struct_keyword: Range,
    token: Token,
    fields: Vec<TypeSymbol>,
    generic: Option<Box<ParseNode>>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecleration {
    type_keyword: Range,
    token: Token,
    old_type: Type,
    assignment: Range,
    generic: Option<Box<ParseNode>>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionDecleration {
    action_keyword: Range,
    template_type: Type,
    generic: Option<Box<ParseNode>>,
    specification: Option<(Range, Type)>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericEntry {
    token: Token,
    constraints: Option<Vec<ParseNode>>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParameters {
    parameters: Vec<ParseNode>,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecleration {
    import_keyword: Range,
    path: Vec<Expression>,
    wildcard: bool,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNode {
    None,
    Expression(Expression, Range),
    Type(Type, Range),
    VariableDecleration(VariableDecleration),
    FunctionDecleration(FunctionDecleration),
    Block(/* Statements */ Vec<ParseNode>, Range),
    Yield(Box<Expression>, Range, Range),
    Return(Box<Expression>, Range, Range),
    TemplateDecleration(TemplateDecleration),
    TypeDecleration(TypeDecleration),
    ActionDecleration(ActionDecleration),
    GenericParameters(GenericParameters),
    Tag(/* Expression of tag */ Expression, Range),
    TagCollection(Vec<ParseNode>, Option<Box<ParseNode>>, Range),
    Import(ImportDecleration),
    Punctuation(Punctuation, Range),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInitializer {
    elements: Vec<ParseNode>,
    bracket_token: Range,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateInitializer {
    named_type: Option<Box<Type>>,
    initializer_values: Vec<(String, Option<Expression>)>,
    brace_token: Range,
    range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Empty,
    Integer(u64, u8, Range),
    Float(f64, Range),
    Boolean(bool, Range),

    // The following varients are formed in the parser
    Array(ArrayInitializer),
    TemplateInitializer(TemplateInitializer),
    String(String, Range),
}
