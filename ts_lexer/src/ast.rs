use std::fmt::{self, Display};

use crate::{default_range, OperatorKind, Range, Token, TokenKind};
use colored::{ColoredString, Colorize};
use rt_format::{Format, FormatArgument, Specifier};
use ts_util::{cast, CreateParent, CreateParentBx, Grouper, TreeDisplay};

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: OperatorKind,
    pub right: Box<Expression>,
    pub range: Range,
}

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            ColoredString::from("Binary Expression").cyan(),
            self.operator
        )
    }
}

impl TreeDisplay for BinaryExpression {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(match index {
            0 => &*self.left,
            1 => &*self.right,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub expression: Box<Expression>,
    pub operator: OperatorKind,
    pub range: Range,
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            ColoredString::from("Unary Expression").cyan(),
            self.operator
        )
    }
}

impl TreeDisplay for UnaryExpression {
    fn num_children(&self) -> usize {
        1
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(match index {
            0 => &*self.expression,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub if_token: Range,
    pub condition: Box<Expression>,
    pub body: Box<ParseNode>,
    pub else_clause: Option<(
        /* else token */ Range,
        /* else clause body*/ Box<ParseNode>,
    )>,
    pub range: Range,
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("If").cyan())
    }
}

impl TreeDisplay for IfStatement {
    fn num_children(&self) -> usize {
        if let Some(_) = self.else_clause {
            3
        } else {
            2
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(match index {
            0 => &*self.condition,
            1 => &*self.body,
            2 => &*self.else_clause.as_ref().unwrap().1,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub while_token: Range,
    pub condition: Box<Expression>,
    pub body: Box<ParseNode>,
    pub range: Range,
}

impl Display for WhileLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("While").cyan())
    }
}

impl TreeDisplay for WhileLoop {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(match index {
            0 => &*self.condition,
            1 => &*self.body,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub expression_to_call: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub paren_tokens: Range,
    pub generic: Option<Vec<Type>>,
    pub range: Range,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Function Call").cyan())
    }
}

impl TreeDisplay for FunctionCall {
    fn num_children(&self) -> usize {
        1 + self.arguments.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(match index {
            0 => &*self.expression_to_call,
            _ => &self.arguments[index - 1],
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpression {
    pub index_expression: Box<Expression>,
    pub index_value: Box<Expression>,
    pub square_range: Range,
}

impl Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Index").cyan())
    }
}

impl TreeDisplay for IndexExpression {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(match index {
            0 => &*self.index_expression,
            1 => &*self.index_value,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoopType {
    Condition {
        init: Box<ParseNode>,
        condition: Box<Expression>,
        update: Box<Expression>,
        body: Box<ParseNode>,
    },
    Of {
        var: Box<ParseNode>,
        expr: Box<Expression>,
        body: Box<ParseNode>,
    },
    In {
        var: Box<ParseNode>,
        expr: Box<Expression>,
        body: Box<ParseNode>,
    },
}

impl LoopType {
    pub fn get_range(&self) -> Range {
        match self {
            LoopType::Condition { init, update, .. } => (init.get_range().0, update.get_range().1),
            LoopType::Of { var, expr, .. } => (var.get_range().0, expr.get_range().1),
            LoopType::In { var, expr, .. } => (var.get_range().0, expr.get_range().1),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForLoop {
    pub keyword: Range,
    pub loop_type: LoopType,
    pub range: Range,
}

impl Display for ForLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.loop_type {
            LoopType::Condition { .. } => {
                write!(f, "{}", ColoredString::from("Conditional Loop").cyan())
            }
            LoopType::Of { .. } => {
                write!(f, "{}", ColoredString::from("For Of Loop").cyan())
            }
            LoopType::In { .. } => {
                write!(f, "{}", ColoredString::from("For In Loop").cyan())
            }
        }
    }
}

impl TreeDisplay for ForLoop {
    fn num_children(&self) -> usize {
        match &self.loop_type {
            LoopType::Condition { .. } => 4,
            LoopType::Of { .. } => 3,
            LoopType::In { .. } => 3,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(match &self.loop_type {
            LoopType::Condition {
                condition,
                init,
                update,
                body,
            } => match index {
                0 => &**condition,
                1 => &**init,
                2 => &**update,
                3 => &**body,
                _ => panic!(),
            },
            LoopType::Of { var, expr, body } => match index {
                0 => &**var,
                1 => &**expr,
                2 => &**body,
                _ => panic!(),
            },
            LoopType::In { var, expr, body } => match index {
                0 => &**var,
                1 => &**expr,
                2 => &**body,
                _ => panic!(),
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Empty,
    Identifier(Token),
    Literal(Literal),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    FunctionCall(FunctionCall),
    Lambda(FunctionSignature, Box<ParseNode>),
    Index(IndexExpression),

    LoopExpression(ForLoop),

    Generic(Box<Expression>, Vec<Type>, Range),
}

impl Expression {
    pub fn get_range(&self) -> Range {
        match self {
            Expression::Identifier(t) => t.range,
            Expression::Literal(t) => t.get_range(),
            Expression::BinaryExpression(t) => t.range,
            Expression::UnaryExpression(t) => t.range,
            Expression::FunctionCall(t) => t.range,
            Expression::Lambda(t, _) => t.range,
            Expression::Index(t) => (t.index_expression.get_range().0, t.square_range.1),
            Expression::LoopExpression(t) => (t.range),
            Expression::Generic(_, _, r) => r.clone(),
            Expression::Empty => panic!(),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Empty => write!(f, "Empty"),
            Expression::Identifier(i) => write!(f, "{}", i.as_string()),
            Expression::Literal(i) => write!(f, "{}", i),
            Expression::BinaryExpression(b) => write!(f, "{}", b),
            Expression::UnaryExpression(b) => write!(f, "{}", b),
            Expression::FunctionCall(b) => write!(f, "{}", b),
            Expression::Lambda(_, _) => write!(f, "{}", ColoredString::from("Lambda").cyan()),
            Expression::Index(b) => write!(f, "{}", b),
            Expression::LoopExpression(b) => write!(f, "{}", b),

            Expression::Generic(_, _, _) => write!(f, "GenericType"),
        }
    }
}

impl TreeDisplay for Expression {
    fn num_children(&self) -> usize {
        match self {
            Expression::Empty => 0,
            Expression::Identifier(_) => 0,
            Expression::Literal(l) => l.num_children(),
            Expression::BinaryExpression(l) => l.num_children(),
            Expression::UnaryExpression(l) => l.num_children(),
            Expression::FunctionCall(l) => l.num_children(),
            Expression::Lambda(_, _) => 0,
            Expression::Index(l) => l.num_children(),
            Expression::LoopExpression(l) => l.num_children(),

            Expression::Generic(_, _, _) => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Expression::Literal(l) => l.child_at(index),
            Expression::BinaryExpression(l) => l.child_at(index),
            Expression::UnaryExpression(l) => l.child_at(index),
            Expression::FunctionCall(l) => l.child_at(index),
            Expression::Index(l) => l.child_at(index),
            Expression::LoopExpression(l) => l.child_at(index),

            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn TreeDisplay + 'a)> {
        match self {
            Expression::Literal(l) => l.child_at_bx(index),
            Expression::BinaryExpression(l) => l.child_at_bx(index),
            Expression::UnaryExpression(l) => l.child_at_bx(index),
            Expression::FunctionCall(l) => l.child_at_bx(index),
            Expression::Index(l) => l.child_at_bx(index),
            Expression::LoopExpression(l) => l.child_at_bx(index),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct IdentSymbol {
    pub identifier: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSymbol {
    pub symbol_type: Type,
    pub symbol: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub parameters: Vec<TypeSymbol>,
    pub return_type: Box<Type>,
    pub parens: Range,
    pub range: Range,
}

impl Display for FunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Function Signature").cyan())
    }
}

impl TreeDisplay for FunctionSignature {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match index {
            0 => None,
            1 => Some(&*self.return_type),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn TreeDisplay + 'a)> {
        match index {
            0 => Box::new(CreateParentBx(
                ColoredString::from("Parameters").green().to_string(),
                self.parameters
                    .iter()
                    .map(
                        |TypeSymbol {
                             symbol_type,
                             symbol,
                         }| {
                            let grp = Grouper(format!("{}: {}", symbol.as_string(), symbol_type));
                            let b: Box<dyn TreeDisplay> = Box::new(grp);
                            b
                        },
                    )
                    .collect(),
            )),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
    pub parens: Range,
    pub range: Range,
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Function Type").cyan())
    }
}

impl TreeDisplay for FunctionType {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match index {
            0 => None,
            1 => Some(&*self.return_type),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn TreeDisplay + 'a)> {
        match index {
            0 => Box::new(CreateParent(
                ColoredString::from("Parameters").green().to_string(),
                self.parameters
                    .iter()
                    .map(|f| {
                        let b: &dyn TreeDisplay = f;
                        b
                    })
                    .collect(),
            )),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub base_type: Box<Type>,
    pub size: Option<(/* Colon */ Range, /* Size */ usize)>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceType {
    pub reference: Range,
    pub base_type: Box<Type>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericType {
    pub base_type: Box<Type>,
    pub arguments: Vec<Type>,
    pub range: Range,
}

impl GenericType {
    pub fn to_expr_generic(self) -> Expression {
        let name = match *self.base_type {
            Type::NamedType(t) => t,
            _ => panic!(),
        };

        Expression::Generic(
            Box::new(Expression::Identifier(name)),
            self.arguments,
            self.range,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Empty,
    Unit,
    Union(Vec<Type>),
    Null(Token),
    Undefined(Token),
    NamedType(Token),
    Number(Token),
    Boolean(Token),
    String(Token),
    ArrayType(ArrayType),
    FunctionType(FunctionType),
    GenericType(GenericType),
}

impl Type {
    pub fn get_range(&self) -> Range {
        match self {
            Type::Empty => panic!(),
            Type::Unit => default_range(),
            Type::Union(f) => {
                if let (Some(f), Some(l)) = (f.first(), f.last()) {
                    (f.get_range().0, l.get_range().1)
                } else {
                    default_range()
                }
            }
            Type::Null(t) => t.range,
            Type::Undefined(t) => t.range,
            Type::NamedType(t) => t.range,
            Type::Number(t) => t.range,
            Type::Boolean(t) => t.range,
            Type::String(t) => t.range,
            Type::ArrayType(t) => t.range,
            Type::FunctionType(t) => t.range,
            Type::GenericType(t) => t.range,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Unit => f.write_str("()"),
            Self::Union(u) => {
                write!(
                    f,
                    "{}",
                    u.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            }
            Self::NamedType(t) => f.write_str(&t.as_string()),
            Self::Number(_) => write!(f, "number"),
            Self::String(_) => write!(f, "string"),
            Self::Boolean(_) => write!(f, "boolean"),
            Self::Null(_) => write!(f, "null"),
            Self::Undefined(_) => write!(f, "undefined"),

            Self::ArrayType(ArrayType {
                base_type, size, ..
            }) => {
                if let Some((_, t)) = size {
                    write!(f, "[{}: {}]", base_type, t)
                } else {
                    write!(f, "[{}]", base_type)
                }
            }
            Self::FunctionType(FunctionType {
                parameters,
                return_type,
                ..
            }) => {
                write!(f, "(")?;
                if parameters.len() >= 1 {
                    write!(f, "{}", parameters[0])?;
                    for t in &parameters[1..] {
                        write!(f, ", {}", t)?;
                    }
                }
                write!(f, ")")?;
                if let Type::Unit = **return_type {
                    write!(f, " =>")
                } else {
                    write!(f, ":{} =>", return_type)
                }
            }
            Self::GenericType(GenericType {
                arguments,
                base_type,
                ..
            }) => {
                write!(f, "{}<", base_type)?;
                write!(f, "{}", arguments[0])?;
                for t in &arguments[1..] {
                    write!(f, ", {}", t)?;
                }
                write!(f, ">")
            }
        }
    }
}

impl TreeDisplay for Type {
    fn num_children(&self) -> usize {
        0
    }

    fn child_at(&self, _index: usize) -> Option<&dyn TreeDisplay> {
        panic!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    Comma,
    FunctionArrow,
}

impl Display for Punctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} `{}`",
            ColoredString::from("Punctuation").cyan(),
            match self {
                Self::Comma => ",",
                Self::FunctionArrow => "=>",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecleration {
    pub variable_type: Option<Box<Type>>,
    pub identifier: Token,
    pub possible_initializer: Option<(
        /* Initializer */ Box<Expression>,
        /* Position of assignment operator*/ Range,
    )>,
    pub is_const: bool,
    pub range: Range,
}

impl Display for VariableDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} `{}`",
            ColoredString::from("Variable Decleration").cyan(),
            cast!(&self.identifier.token_type, TokenKind::Ident)
        )
    }
}

impl TreeDisplay for VariableDecleration {
    fn num_children(&self) -> usize {
        (if self.variable_type.is_some() { 1 } else { 0 })
            + (if self.possible_initializer.is_some() {
                1
            } else {
                0
            })
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match index {
            0 => {
                if let Some(ref ty) = self.variable_type {
                    Some(&**ty)
                } else if let Some((ref init, _)) = self.possible_initializer {
                    Some(&**init)
                } else {
                    None
                }
            }
            1 => {
                if let Some((ref init, _)) = self.possible_initializer {
                    Some(&**init)
                } else {
                    None
                }
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecleration {
    pub identifier: Token,
    pub function_type: FunctionSignature,
    pub body: Box<ParseNode>,
    pub generic: Option<Box<ParseNode>>,
    pub range: Range,
}

impl Display for FunctionDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} `{}`",
            ColoredString::from("Function Decleration").cyan(),
            cast!(&self.identifier.token_type, TokenKind::Ident)
        )
    }
}

impl TreeDisplay for FunctionDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            3
        } else {
            2
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match index {
            0 => Some(&self.function_type),
            1 => Some(&*self.body),
            2 => Some(&**self.generic.as_ref().unwrap()),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassDecleration {
    pub struct_keyword: Range,
    pub token: Token,
    pub body: Vec<ClassBody>,
    pub generic: Option<Box<ParseNode>>,
    pub range: Range,
}

// impl Clone for ClassDecleration {
//     fn clone(&self) -> Self {
//         Self { struct_keyword: self.struct_keyword.clone(), token: self.token.clone(), body: self.body.clone(), generic: self.generic.clone(), range: self.range.clone() }
//     }
// }

impl PartialEq for ClassDecleration {
    fn eq(&self, other: &Self) -> bool {
        self.struct_keyword == other.struct_keyword
            && self.token == other.token
            && self.generic == other.generic
            && self.range == other.range
            && self
                .body
                .iter()
                .zip(other.body.iter())
                .find(|f| f.0 != f.1)
                .is_none()
    }
}

impl Display for ClassDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} `{}`",
            ColoredString::from("Template Decleration").cyan(),
            cast!(&self.token.token_type, TokenKind::Ident)
        )
    }
}

impl TreeDisplay for ClassDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            2
        } else {
            1
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match index {
            0 => None,
            1 => Some(&**self.generic.as_ref().unwrap()),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn TreeDisplay + 'a)> {
        match index {
            0 => Box::new(CreateParentBx(
                ColoredString::from("Parameters").green().to_string(),
                self.body
                    .iter()
                    .map(|c| match c {
                        ClassBody::Field {
                            ident,
                            ty,
                            visibility,
                        } => {
                            let grp =
                                Grouper(format!("{:?} {}: {}", visibility, ident.as_string(), ty));
                            let b: Box<dyn TreeDisplay> = Box::new(grp);
                            b
                        }
                        ClassBody::Function(fd, vis) => {
                            let cp =
                                CreateParentBx(format!("{:?}", vis), vec![Box::new(fd.clone())]);
                            let b: Box<dyn TreeDisplay> = Box::new(cp);
                            b
                        }
                    })
                    .collect(),
            )),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Private,
    Public,
    Protected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassBody {
    Field {
        ident: Token,
        ty: Type,
        visibility: Visibility,
    },
    Function(FunctionDecleration, Visibility),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecleration {
    pub type_keyword: Range,
    pub token: Token,
    pub old_type: Type,
    pub assignment: Range,
    pub range: Range,
}

impl Display for TypeDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} `{}` = {}",
            ColoredString::from("Type").cyan(),
            cast!(&self.token.token_type, TokenKind::Ident),
            self.old_type
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionDecleration {
    pub action_keyword: Range,
    pub template_type: Type,
    pub generic: Option<Box<ParseNode>>,
    pub specification: Option<Type>,
    pub body: Box<ParseNode>,
    pub range: Range,
}

impl Display for ActionDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(spec) = &self.specification {
            write!(
                f,
                "{} {}: {}",
                ColoredString::from("Action Decleration").cyan(),
                self.template_type,
                spec,
            )
        } else {
            write!(
                f,
                "{} {}",
                ColoredString::from("Action Decleration").cyan(),
                self.template_type,
            )
        }
    }
}

impl TreeDisplay for ActionDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            2
        } else {
            1
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match index {
            0 => {
                if let Some(generic) = &self.generic {
                    Some(&**generic)
                } else {
                    Some(&*self.body)
                }
            }
            1 => Some(&*self.body),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecDecleration {
    pub spec_keyword: Range,
    pub identifier: Token,
    pub generic: Option<Box<ParseNode>>,
    pub body: Vec<SpecBody>,
    pub range: Range,
}

impl Display for SpecDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            ColoredString::from("Spec Decleration").cyan(),
            cast!(&self.identifier.token_type, TokenKind::Ident),
        )
    }
}

impl TreeDisplay for SpecDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            1 + self.body.len()
        } else {
            self.body.len()
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match index {
            0 => {
                if let Some(generic) = &self.generic {
                    Some(&**generic)
                } else {
                    Some(&self.body[index])
                }
            }
            1 => Some(&self.body[index - 1]),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecBody {
    Function(Token, FunctionSignature),
}

impl Display for SpecBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecBody::Function(tok, _) => {
                write!(f, "{}", tok.as_string(),)
            }
        }
    }
}

impl TreeDisplay for SpecBody {
    fn num_children(&self) -> usize {
        match self {
            Self::Function(_, _) => 1,
        }
    }

    fn child_at(&self, _index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Self::Function(_, fnb) => Some(fnb),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericEntry {
    pub token: Token,
    pub constraints: Option<Vec<ParseNode>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParameters {
    pub parameters: Vec<(Token, Option<Vec<Type>>, Option<Type>)>,
    pub range: Range,
}

impl Display for GenericParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Generic Parameter").cyan())
    }
}

impl TreeDisplay for GenericParameters {
    fn num_children(&self) -> usize {
        self.parameters.len()
    }

    fn child_at(&self, _index: usize) -> Option<&dyn TreeDisplay> {
        None
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        let (child_name, restraints, specialization) = &self.parameters[index];
        if let Some(res) = &restraints {
            Box::new(CreateParent(
                child_name.as_string().clone(),
                res.iter()
                    .map(|f| {
                        let b: &dyn TreeDisplay = f;
                        b
                    })
                    .collect(),
            ))
        } else if let Some(spec) = &specialization {
            Box::new(CreateParent(child_name.as_string().clone(), vec![spec]))
        } else {
            Box::new(Grouper(child_name.as_string().clone()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecleration {
    pub import_keyword: Range,
    pub path: Vec<Expression>,
    pub range: Range,
}

impl Display for ImportDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Import").cyan())
    }
}

impl TreeDisplay for ImportDecleration {
    fn num_children(&self) -> usize {
        self.path.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(&self.path[index])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNode {
    Empty,
    Expression(Expression, Range),
    Type(Type, Range),
    VariableDecleration(VariableDecleration),
    FunctionDecleration(FunctionDecleration),
    IfStatement(IfStatement),
    Yield(Box<Expression>, Range),
    Return(Box<Expression>, Range),
    TemplateDecleration(ClassDecleration),
    TypeDecleration(TypeDecleration),
    ActionDecleration(ActionDecleration),
    SpecDecleration(SpecDecleration),
    GenericParameters(GenericParameters),
    Tag(/* Expression of tag */ Expression, Range),
    TagCollection(Vec<ParseNode>, Box<ParseNode>, Range),
    Import(ImportDecleration),
    Punctuation(Punctuation, Range),
    Export(Box<ParseNode>, Range),

    Block(/* Statements */ Vec<ParseNode>, Range),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
}

impl ParseNode {
    pub fn get_range(&self) -> Range {
        match self {
            ParseNode::Expression(_, r) => r.clone(),
            ParseNode::Type(_, r) => r.clone(),
            ParseNode::VariableDecleration(v) => v.range,
            ParseNode::FunctionDecleration(f) => f.range,
            ParseNode::Yield(_, f) => f.clone(),
            ParseNode::Return(_, f) => f.clone(),
            ParseNode::TemplateDecleration(s) => s.range,
            ParseNode::TypeDecleration(t) => t.range,
            ParseNode::ActionDecleration(l) => l.range,
            ParseNode::SpecDecleration(l) => l.range,
            ParseNode::GenericParameters(l) => l.range,
            ParseNode::Tag(_, r) => r.clone(),
            ParseNode::TagCollection(_, _, r) => r.clone(),
            ParseNode::Import(i) => i.range,
            ParseNode::Punctuation(_, p) => p.clone(),
            ParseNode::Export(_, p) => p.clone(),

            ParseNode::Block(_, f) => f.clone(),
            ParseNode::Empty => default_range(),
            ParseNode::IfStatement(t) => t.range,
            ParseNode::WhileLoop(t) => t.range,
            ParseNode::ForLoop(t) => t.range,
        }
    }
}

impl Display for ParseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseNode::Expression(i, _) => write!(f, "{}", i),
            ParseNode::Type(i, _) => write!(f, "{} {}", ColoredString::from("Type").cyan(), i),
            ParseNode::VariableDecleration(i) => write!(f, "{}", i),
            ParseNode::FunctionDecleration(i) => write!(f, "{}", i),
            ParseNode::IfStatement(i) => write!(f, "{}", i),
            ParseNode::Yield(_, _) => write!(f, "{}", ColoredString::from("Yield").cyan()),
            ParseNode::Return(_, _) => write!(f, "{}", ColoredString::from("Return").cyan()),
            ParseNode::TemplateDecleration(i) => write!(f, "{}", i),
            ParseNode::TypeDecleration(i) => write!(f, "{}", i),
            ParseNode::ActionDecleration(i) => write!(f, "{}", i),
            ParseNode::SpecDecleration(i) => write!(f, "{}", i),
            ParseNode::GenericParameters(i) => write!(f, "{}", i),
            ParseNode::Tag(_, _) => write!(f, "{}", ColoredString::from("Tag").cyan()),
            ParseNode::TagCollection(_, _, _) => {
                write!(f, "{}", ColoredString::from("Tag Collection").cyan())
            }
            ParseNode::Import(i) => write!(f, "{}", i),
            ParseNode::Punctuation(i, _) => write!(f, "{}", i),
            ParseNode::Export(i, _) => write!(f, "{}", i),
            ParseNode::Empty => write!(f, "{}", ColoredString::from("None").red()),
            ParseNode::Block(_, _) => write!(f, "{}", ColoredString::from("Block").cyan()),
            ParseNode::WhileLoop(w) => write!(f, "{}", w),
            ParseNode::ForLoop(w) => write!(f, "{}", w),
        }
    }
}

impl TreeDisplay for ParseNode {
    fn num_children(&self) -> usize {
        match self {
            ParseNode::Expression(i, _) => i.num_children(),
            ParseNode::VariableDecleration(i) => i.num_children(),
            ParseNode::FunctionDecleration(i) => i.num_children(),
            ParseNode::Yield(_, _) => 1,
            ParseNode::Return(_, _) => 1,
            ParseNode::TemplateDecleration(i) => i.num_children(),
            ParseNode::ActionDecleration(i) => i.num_children(),
            ParseNode::SpecDecleration(i) => i.num_children(),
            ParseNode::GenericParameters(i) => i.num_children(),
            ParseNode::Tag(_, _) => 1,
            ParseNode::TagCollection(i, _, _) => i.len() + 1,
            ParseNode::Import(i) => i.num_children(),
            ParseNode::Export(i, _) => i.num_children(),
            ParseNode::Block(i, _) => i.len(),
            ParseNode::IfStatement(l) => l.num_children(),
            ParseNode::WhileLoop(l) => l.num_children(),
            ParseNode::ForLoop(l) => l.num_children(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            ParseNode::Expression(i, _) => i.child_at(index),
            ParseNode::VariableDecleration(i) => i.child_at(index),
            ParseNode::FunctionDecleration(i) => i.child_at(index),
            ParseNode::Yield(i, _) => Some(&**i),
            ParseNode::Return(i, _) => Some(&**i),
            ParseNode::TemplateDecleration(i) => i.child_at(index),
            ParseNode::ActionDecleration(i) => i.child_at(index),
            ParseNode::SpecDecleration(i) => i.child_at(index),
            ParseNode::GenericParameters(i) => i.child_at(index),
            ParseNode::Tag(i, _) => Some(i),
            ParseNode::TagCollection(i, b, _) => {
                if index < i.len() {
                    Some(&i[index])
                } else {
                    Some(&**b)
                }
            }
            ParseNode::Import(i) => i.child_at(index),
            ParseNode::Export(i, _) => i.child_at(index),
            ParseNode::Block(i, _) => Some(&i[index]),
            ParseNode::IfStatement(l) => l.child_at(index),
            ParseNode::WhileLoop(l) => l.child_at(index),
            ParseNode::ForLoop(l) => l.child_at(index),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn TreeDisplay + 'a)> {
        match self {
            ParseNode::Expression(i, _) => i.child_at_bx(index),
            ParseNode::VariableDecleration(i) => i.child_at_bx(index),
            ParseNode::FunctionDecleration(i) => i.child_at_bx(index),
            ParseNode::TemplateDecleration(i) => i.child_at_bx(index),
            ParseNode::ActionDecleration(i) => i.child_at_bx(index),
            ParseNode::SpecDecleration(i) => i.child_at_bx(index),
            ParseNode::GenericParameters(i) => i.child_at_bx(index),
            ParseNode::Import(i) => i.child_at_bx(index),
            ParseNode::Export(i, _) => i.child_at_bx(index),
            ParseNode::IfStatement(l) => l.child_at_bx(index),
            ParseNode::WhileLoop(l) => l.child_at_bx(index),
            ParseNode::ForLoop(l) => l.child_at_bx(index),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayInitializer {
    pub elements: Vec<Expression>,
    pub range: Range,
}

impl Display for ArrayInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Array").cyan())
    }
}

impl TreeDisplay for ArrayInitializer {
    fn num_children(&self) -> usize {
        self.elements.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        Some(&self.elements[index])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateInitializer {
    pub named_type: Option<Box<Type>>,
    pub initializer_values: Vec<(String, Expression)>,
    pub range: Range,
}

impl Display for TemplateInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Template Initializer").cyan())
    }
}

impl TreeDisplay for TemplateInitializer {
    fn num_children(&self) -> usize {
        if let Some(_) = &self.named_type {
            self.initializer_values.len() + 2
        } else {
            self.initializer_values.len()
        }
    }

    fn child_at(&self, _index: usize) -> Option<&dyn TreeDisplay> {
        None
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        // self.initializer_values[index].1.as_ref().unwrap()
        // &Grouper("Pod".to_string())
        if let Some(ty) = &self.named_type {
            match index {
                0 => Box::new(Grouper(
                    ColoredString::from(format!("Type").as_str())
                        .blue()
                        .to_string(),
                )),
                1 => Box::new(Grouper(
                    ColoredString::from(format!("{}", ty).as_str())
                        .blue()
                        .to_string(),
                )),
                _c => {
                    let (name, value) = &self.initializer_values[index - 2];
                    Box::new(CreateParent(
                        ColoredString::from(format!("{}", name).as_str())
                            .green()
                            .to_string(),
                        vec![value],
                    ))
                }
            }
        } else {
            let (name, value) = &self.initializer_values[index];
            Box::new(CreateParent(
                ColoredString::from(format!("{}", name).as_str())
                    .green()
                    .to_string(),
                vec![value],
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Empty,
    Integer(u64, u8, Range),
    Float(f64, Range),
    Boolean(bool, Range),

    // The following varients are formed in the parser
    Array(ArrayInitializer),
    StructInitializer(TemplateInitializer),
    String(String, Range),
    Null(Range),
    Undefined(Range),
}

impl FormatArgument for Literal {
    fn supports_format(&self, spec: &Specifier) -> bool {
        match self {
            Self::Integer(_, _, _) => true,
            Self::Boolean(_, _) => true,
            Self::String(_, _) => true,
            Self::Float(_, _) => match spec.format {
                Format::Display | Format::Debug | Format::LowerExp | Format::UpperExp => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(val, _, _) => fmt::Display::fmt(&val, f),
            Self::Float(val, _) => fmt::Display::fmt(&val, f),
            Self::Boolean(val, _) => fmt::Display::fmt(&val, f),
            Self::String(val, _) => fmt::Display::fmt(&val, f),
            _ => panic!(),
        }
    }

    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }

    fn fmt_octal(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(val, _, _) => fmt::Octal::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(val, _, _) => fmt::LowerHex::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(val, _, _) => fmt::UpperHex::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_binary(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(val, _, _) => fmt::Binary::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_lower_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(val, _, _) => fmt::LowerExp::fmt(&val, f),
            Self::Float(val, _) => fmt::LowerExp::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn fmt_upper_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(val, _, _) => fmt::UpperExp::fmt(&val, f),
            Self::Float(val, _) => fmt::UpperExp::fmt(&val, f),
            _ => Err(fmt::Error),
        }
    }

    fn to_usize(&self) -> Result<usize, ()> {
        match self {
            Self::Integer(val, _, _) => (*val).try_into().map_err(|_| ()),
            _ => Err(()),
        }
    }
}

impl Literal {
    pub fn get_range(&self) -> Range {
        match self {
            Literal::Integer(_, _, r) => r.clone(),
            Literal::Float(_, r) => r.clone(),
            Literal::Boolean(_, r) => r.clone(),
            Literal::Array(r) => r.range.clone(),
            Literal::Null(r) => r.clone(),
            Literal::Undefined(r) => r.clone(),
            Literal::StructInitializer(r) => r.range.clone(),
            Literal::String(_, r) => r.clone(),
            Literal::Empty => default_range(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(i, _, _) => write!(
                f,
                "Integer {}",
                ColoredString::from(format!("{}", i).as_str()).yellow()
            ),
            Literal::Float(b, _) => write!(
                f,
                "Float {}",
                ColoredString::from(format!("{}", b).as_str()).yellow()
            ),
            Literal::Boolean(b, _) => write!(
                f,
                "Boolean {}",
                ColoredString::from(format!("{}", b).as_str()).yellow()
            ),
            Literal::Null(_) => write!(
                f,
                "{}",
                ColoredString::from(format!("Null").as_str()).yellow()
            ),
            Literal::Undefined(_) => write!(
                f,
                "{}",
                ColoredString::from(format!("Undefined").as_str()).yellow()
            ),
            Literal::Array(b) => write!(f, "{}", b),
            Literal::StructInitializer(b) => write!(f, "{}", b),
            Literal::String(b, _) => write!(
                f,
                "String {}",
                ColoredString::from(format!("'{}'", b).as_str()).yellow()
            ),
            Literal::Empty => write!(f, "Empty"),
        }
    }
}

impl TreeDisplay for Literal {
    fn num_children(&self) -> usize {
        match self {
            Literal::Array(b) => b.num_children(),
            Literal::StructInitializer(b) => b.num_children(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Literal::Array(b) => b.child_at(index),
            Literal::StructInitializer(b) => b.child_at(index),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        match self {
            Literal::StructInitializer(b) => b.child_at_bx(index),
            _ => panic!(),
        }
    }
}
