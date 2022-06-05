use std::fmt::{self, Display};

use crate::{default_range, OperatorKind, Range, Token, TokenKind};
use colored::{ColoredString, Colorize};
use dsl_util::cast;

pub struct Fmt<F>(pub F)
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result;

impl<F> fmt::Display for Fmt<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

pub trait AstIndexable: Display {
    fn num_children(&self) -> usize;
    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable>;
    fn child_at_bx<'a>(&'a self, _index: usize) -> Box<dyn AstIndexable + 'a> {
        panic!("This type doesn't used box values!")
    }

    fn write(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        index: u32,
        indent: &String,
        last: bool,
    ) -> std::fmt::Result {
        write!(f, "{}", indent)?;
        if index != 0 {
            write!(f, "{}", if last { "└──" } else { "├──" })?;
        }
        let nindent = format!(
            "{}{}",
            indent,
            if index == 0 {
                ""
            } else if last {
                "    "
            } else {
                "│   "
            }
        );
        write!(f, "{}\n", self)?;

        let n = self.num_children();
        for i in 0..n {
            let child = self.child_at(i);
            if let Some(child) = child {
                child.write(
                    f,
                    (i + 1).try_into().unwrap(),
                    &nindent,
                    if i == n - 1 { true } else { false },
                )?;
            } else {
                let child = self.child_at_bx(i);
                child.write(
                    f,
                    (i + 1).try_into().unwrap(),
                    &nindent,
                    if i == n - 1 { true } else { false },
                )?;
            }
        }

        write!(f, "")
    }

    fn format(&self) -> String {
        format!("{}", Fmt(|f| self.write(f, 0, &String::from(""), false)))
    }
}

impl Loop {
    pub fn get_range(&self) -> Range {
        match self {
            Loop::Infinite(b) => b.get_range(),
            Loop::Until(e, b) => (e.get_range().0, b.get_range().1),
        }
    }
}

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

impl AstIndexable for BinaryExpression {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
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

impl AstIndexable for UnaryExpression {
    fn num_children(&self) -> usize {
        1
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(match index {
            0 => &*self.expression,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    pub if_token: Range,
    pub condition: Box<Expression>,
    pub body: Box<ParseNode>,
    pub else_clause: Option<(
        /* else token */ Range,
        /* else clause body*/ Box<ParseNode>,
    )>,
    pub range: Range,
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("If").cyan())
    }
}

impl AstIndexable for IfExpression {
    fn num_children(&self) -> usize {
        if let Some(_) = self.else_clause {
            3
        } else {
            2
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(match index {
            0 => &*self.condition,
            1 => &*self.body,
            2 => &*self.else_clause.as_ref().unwrap().1,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub expression_to_call: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub paren_tokens: Range,
    pub range: Range,
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Function Call").cyan())
    }
}

impl AstIndexable for FunctionCall {
    fn num_children(&self) -> usize {
        1 + self.arguments.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(match index {
            0 => &*self.expression_to_call,
            _ => &self.arguments[index],
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

impl AstIndexable for IndexExpression {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(match index {
            0 => &*self.index_expression,
            1 => &*self.index_value,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Loop {
    Infinite(/* Body */ Box<ParseNode>),
    Until(
        /* Condition */ Box<Expression>,
        /* Body */ Box<ParseNode>,
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpression {
    pub keyword: Range,
    pub loop_type: Loop,
    pub range: Range,
}

impl Display for LoopExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.loop_type {
            Loop::Infinite(_) => write!(f, "{}", ColoredString::from("Unconditional Loop").cyan()),
            Loop::Until(_, _) => write!(f, "{}", ColoredString::from("Conditional Loop").cyan()),
        }
    }
}

impl AstIndexable for LoopExpression {
    fn num_children(&self) -> usize {
        match &self.loop_type {
            Loop::Infinite(_) => 0,
            Loop::Until(_, _) => 1,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(match &self.loop_type {
            Loop::Infinite(b) => &**b,
            Loop::Until(b, p) => match index {
                0 => &**b,
                1 => &**p,
                _ => panic!(),
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Token),
    Literal(Literal),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    FunctionCall(FunctionCall),
    Lambda(FunctionSignature, Box<ParseNode>),
    Index(IndexExpression),

    IfExpression(IfExpression),
    LoopExpression(LoopExpression),
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
            Expression::IfExpression(t) => (t.range),
            Expression::LoopExpression(t) => (t.range),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(i) => write!(f, "{}", cast!(&i.token_type, TokenKind::Ident)),
            Expression::Literal(i) => write!(f, "{}", i),
            Expression::BinaryExpression(b) => write!(f, "{}", b),
            Expression::UnaryExpression(b) => write!(f, "{}", b),
            Expression::FunctionCall(b) => write!(f, "{}", b),
            Expression::Lambda(_, _) => write!(f, "{}", ColoredString::from("Lambda").cyan()),
            Expression::Index(b) => write!(f, "{}", b),
            Expression::IfExpression(b) => write!(f, "{}", b),
            Expression::LoopExpression(b) => write!(f, "{}", b),
        }
    }
}

impl AstIndexable for Expression {
    fn num_children(&self) -> usize {
        match self {
            Expression::Identifier(_) => 0,
            Expression::Literal(l) => l.num_children(),
            Expression::BinaryExpression(l) => l.num_children(),
            Expression::UnaryExpression(l) => l.num_children(),
            Expression::FunctionCall(l) => l.num_children(),
            Expression::Lambda(_, _) => 0,
            Expression::Index(l) => l.num_children(),
            Expression::IfExpression(l) => l.num_children(),
            Expression::LoopExpression(l) => l.num_children(),
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        match self {
            Expression::Literal(l) => l.child_at(index),
            Expression::BinaryExpression(l) => l.child_at(index),
            Expression::UnaryExpression(l) => l.child_at(index),
            Expression::FunctionCall(l) => l.child_at(index),
            Expression::Index(l) => l.child_at(index),
            Expression::IfExpression(l) => l.child_at(index),
            Expression::LoopExpression(l) => l.child_at(index),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn AstIndexable + 'a)> {
        match self {
            Expression::Literal(l) => l.child_at_bx(index),
            Expression::BinaryExpression(l) => l.child_at_bx(index),
            Expression::UnaryExpression(l) => l.child_at_bx(index),
            Expression::FunctionCall(l) => l.child_at_bx(index),
            Expression::Index(l) => l.child_at_bx(index),
            Expression::IfExpression(l) => l.child_at_bx(index),
            Expression::LoopExpression(l) => l.child_at_bx(index),
            _ => panic!(),
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Expression {
//     kind: ExpressionKind,
//     range: Range,
// }

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

impl AstIndexable for FunctionSignature {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        match index {
            0 => None,
            1 => Some(&*self.return_type),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn AstIndexable + 'a)> {
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
                            let grp = Grouper(format!(
                                "{}: {}",
                                cast!(&symbol.token_type, TokenKind::Ident),
                                symbol_type
                            ));
                            let b: Box<dyn AstIndexable> = Box::new(grp);
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

impl AstIndexable for FunctionType {
    fn num_children(&self) -> usize {
        2
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        match index {
            0 => None,
            1 => Some(&*self.return_type),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn AstIndexable + 'a)> {
        match index {
            0 => Box::new(CreateParent(
                ColoredString::from("Parameters").green().to_string(),
                self.parameters
                    .iter()
                    .map(|f| {
                        let b: &dyn AstIndexable = f;
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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    NamedType(Token),
    Int(u8, Range),
    Uint(u8, Range),
    Bool(Range),
    Float(u8, Range),
    Char(Range),
    ArrayType(ArrayType),
    FunctionType(FunctionType),
    ReferenceType(ReferenceType),
    GenericType(GenericType),
}

impl Type {
    pub fn get_range(&self) -> Range {
        match self {
            Type::Unit => default_range(),
            Type::NamedType(t) => t.range,
            Type::Int(_, t) => t.clone(),
            Type::Uint(_, t) => t.clone(),
            Type::Bool(t) => t.clone(),
            Type::Float(_, t) => t.clone(),
            Type::Char(t) => t.clone(),
            Type::ArrayType(t) => t.range,
            Type::FunctionType(t) => t.range,
            Type::ReferenceType(t) => t.range,
            Type::GenericType(t) => t.range,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => f.write_str("()"),
            Self::NamedType(t) => f.write_str(cast!(&t.token_type, TokenKind::Ident)),
            Self::Int(t, _) => write!(f, "int{}", t),
            Self::Uint(t, _) => write!(f, "uint{}", t),
            Self::Bool(_) => write!(f, "bool"),
            Self::Float(s, _) => write!(f, "float{}", s),
            Self::Char(_) => write!(f, "char"),
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
            Self::ReferenceType(ReferenceType { base_type, .. }) => {
                write!(f, "&{}", base_type)
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

impl AstIndexable for Type {
    fn num_children(&self) -> usize {
        0
    }

    fn child_at(&self, _index: usize) -> Option<&dyn AstIndexable> {
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

impl AstIndexable for VariableDecleration {
    fn num_children(&self) -> usize {
        (if self.variable_type.is_some() { 1 } else { 0 })
            + (if self.possible_initializer.is_some() {
                1
            } else {
                0
            })
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
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

impl AstIndexable for FunctionDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            3
        } else {
            2
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        match index {
            0 => Some(&self.function_type),
            1 => Some(&*self.body),
            2 => Some(&**self.generic.as_ref().unwrap()),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateDecleration {
    pub struct_keyword: Range,
    pub token: Token,
    pub fields: Vec<TypeSymbol>,
    pub generic: Option<Box<ParseNode>>,
    pub range: Range,
}

impl Display for TemplateDecleration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} `{}`",
            ColoredString::from("Template Decleration").cyan(),
            cast!(&self.token.token_type, TokenKind::Ident)
        )
    }
}

impl AstIndexable for TemplateDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            2
        } else {
            1
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        match index {
            0 => None,
            1 => Some(&**self.generic.as_ref().unwrap()),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn AstIndexable + 'a)> {
        match index {
            0 => Box::new(CreateParentBx(
                ColoredString::from("Parameters").green().to_string(),
                self.fields
                    .iter()
                    .map(
                        |TypeSymbol {
                             symbol_type,
                             symbol,
                         }| {
                            let grp = Grouper(format!(
                                "{}: {}",
                                cast!(&symbol.token_type, TokenKind::Ident),
                                symbol_type
                            ));
                            let b: Box<dyn AstIndexable> = Box::new(grp);
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

impl AstIndexable for ActionDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            2
        } else {
            1
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
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

impl AstIndexable for SpecDecleration {
    fn num_children(&self) -> usize {
        if self.generic.is_some() {
            1 + self.body.len()
        } else {
            self.body.len()
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
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
                write!(f, "{}", cast!(&tok.token_type, TokenKind::Ident),)
            }
        }
    }
}

impl AstIndexable for SpecBody {
    fn num_children(&self) -> usize {
        match self {
            Self::Function(_, _) => 1,
        }
    }

    fn child_at(&self, _index: usize) -> Option<&dyn AstIndexable> {
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
    pub parameters: Vec<(Token, Option<Vec<Type>>)>,
    pub range: Range,
}

impl Display for GenericParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Generic Parameter").cyan())
    }
}

impl AstIndexable for GenericParameters {
    fn num_children(&self) -> usize {
        self.parameters.len()
    }

    fn child_at(&self, _index: usize) -> Option<&dyn AstIndexable> {
        None
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn AstIndexable + 'a> {
        let (child_name, restraints) = &self.parameters[index];
        if let Some(res) = &restraints {
            Box::new(CreateParent(
                cast!(&child_name.token_type, TokenKind::Ident).clone(),
                res.iter()
                    .map(|f| {
                        let b: &dyn AstIndexable = f;
                        b
                    })
                    .collect(),
            ))
        } else {
            Box::new(Grouper(
                cast!(&child_name.token_type, TokenKind::Ident).clone(),
            ))
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

impl AstIndexable for ImportDecleration {
    fn num_children(&self) -> usize {
        self.path.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(&self.path[index])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseNode {
    None,
    Expression(Expression, Range),
    Type(Type, Range),
    VariableDecleration(VariableDecleration),
    FunctionDecleration(FunctionDecleration),
    Block(/* Statements */ Vec<ParseNode>, Range),
    Yield(Box<Expression>, Range),
    Return(Box<Expression>, Range),
    TemplateDecleration(TemplateDecleration),
    TypeDecleration(TypeDecleration),
    ActionDecleration(ActionDecleration),
    SpecDecleration(SpecDecleration),
    GenericParameters(GenericParameters),
    Tag(/* Expression of tag */ Expression, Range),
    TagCollection(Vec<ParseNode>, Box<ParseNode>, Range),
    Import(ImportDecleration),
    Punctuation(Punctuation, Range),
}

impl ParseNode {
    pub fn get_range(&self) -> Range {
        match self {
            ParseNode::Expression(_, r) => r.clone(),
            ParseNode::Type(_, r) => r.clone(),
            ParseNode::VariableDecleration(v) => v.range,
            ParseNode::FunctionDecleration(f) => f.range,
            ParseNode::Block(_, f) => f.clone(),
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

            ParseNode::None => default_range(),
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
            ParseNode::Block(_, _) => write!(f, "{}", ColoredString::from("Block").cyan()),
            ParseNode::Yield(_, _) => write!(f, "{}", ColoredString::from("Yield").cyan()),
            ParseNode::Return(_, _) => write!(f, "{}", ColoredString::from("Return").cyan()),
            ParseNode::TemplateDecleration(i) => write!(f, "{}", i),
            ParseNode::TypeDecleration(i) => write!(f, "{}", i),
            ParseNode::ActionDecleration(i) => write!(f, "{}", i),
            ParseNode::SpecDecleration(i) => write!(f, "{}", i),
            ParseNode::GenericParameters(i) => write!(f, "{}", i),
            ParseNode::Tag(_, _) => write!(f, "{}", ColoredString::from("Tag").cyan()),
            ParseNode::TagCollection(_, _, _) => write!(f, "{}", ColoredString::from("Tag Collection").cyan()),
            ParseNode::Import(i) => write!(f, "{}", i),
            ParseNode::Punctuation(i, _) => write!(f, "{}", i),
            ParseNode::None => write!(f, "{}", ColoredString::from("None").red()),
        }
    }
}

impl AstIndexable for ParseNode {
    fn num_children(&self) -> usize {
        match self {
            ParseNode::Expression(i, _) => i.num_children(),
            ParseNode::VariableDecleration(i) => i.num_children(),
            ParseNode::FunctionDecleration(i) => i.num_children(),
            ParseNode::Block(i, _) => i.len(),
            ParseNode::Yield(_, _) => 1,
            ParseNode::Return(_, _) => 1,
            ParseNode::TemplateDecleration(i) => i.num_children(),
            ParseNode::ActionDecleration(i) => i.num_children(),
            ParseNode::SpecDecleration(i) => i.num_children(),
            ParseNode::GenericParameters(i) => i.num_children(),
            ParseNode::Tag(_, _) => 1,
            ParseNode::TagCollection(i, _, _) => i.len() + 1,
            ParseNode::Import(i) => i.num_children(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        match self {
            ParseNode::Expression(i, _) => i.child_at(index),
            ParseNode::VariableDecleration(i) => i.child_at(index),
            ParseNode::FunctionDecleration(i) => i.child_at(index),
            ParseNode::Block(i, _) => Some(&i[index]),
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
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<(dyn AstIndexable + 'a)> {
        match self {
            ParseNode::Expression(i, _) => i.child_at_bx(index),
            ParseNode::VariableDecleration(i) => i.child_at_bx(index),
            ParseNode::FunctionDecleration(i) => i.child_at_bx(index),
            ParseNode::TemplateDecleration(i) => i.child_at_bx(index),
            ParseNode::ActionDecleration(i) => i.child_at_bx(index),
            ParseNode::SpecDecleration(i) => i.child_at_bx(index),
            ParseNode::GenericParameters(i) => i.child_at_bx(index),
            ParseNode::Import(i) => i.child_at_bx(index),
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

impl AstIndexable for ArrayInitializer {
    fn num_children(&self) -> usize {
        self.elements.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(&self.elements[index])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateInitializer {
    pub named_type: Option<Box<Type>>,
    pub initializer_values: Vec<(String, Option<Expression>)>,
    pub range: Range,
}

impl Display for TemplateInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Template Initializer").cyan())
    }
}

impl AstIndexable for TemplateInitializer {
    fn num_children(&self) -> usize {
        if let Some(_) = &self.named_type {
            self.initializer_values.len() + 2
        } else {
            self.initializer_values.len()
        }
    }

    fn child_at(&self, _index: usize) -> Option<&dyn AstIndexable> {
        None
    }

    fn child_at_bx(&self, index: usize) -> Box<dyn AstIndexable> {
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
                    if let Some(value) = value {
                        Box::new(CreateParent(
                            ColoredString::from(format!("{}", name).as_str())
                                .green()
                                .to_string(),
                            vec![],
                        ))
                    } else {
                        Box::new(Grouper(
                            ColoredString::from(format!("{}", name).as_str())
                                .blue()
                                .to_string(),
                        ))
                    }
                }
            }

            // self.initializer_values.len() + 2
        } else {
            let (name, value) = &self.initializer_values[index];
            if let Some(value) = value {
                Box::new(CreateParent(
                    ColoredString::from(format!("{}", name).as_str())
                        .green()
                        .to_string(),
                    vec![],
                ))
            } else {
                Box::new(Grouper(
                    ColoredString::from(format!("{}", name).as_str())
                        .blue()
                        .to_string(),
                ))
            }
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
}

impl Literal {
    pub fn get_range(&self) -> Range {
        match self {
            Literal::Integer(_, _, r) => r.clone(),
            Literal::Float(_, r) => r.clone(),
            Literal::Boolean(_, r) => r.clone(),
            Literal::Array(r) => r.range.clone(),
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
            Literal::Array(b) => write!(f, "{}", b),
            Literal::StructInitializer(b) => write!(f, "{}", b),
            Literal::String(b, _) => write!(f, "String \"{}\"", b),
            Literal::Empty => write!(f, "Empty"),
        }
    }
}

impl AstIndexable for Literal {
    fn num_children(&self) -> usize {
        match self {
            Literal::Array(b) => b.num_children(),
            Literal::StructInitializer(b) => b.num_children(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        match self {
            Literal::Array(b) => b.child_at(index),
            Literal::StructInitializer(b) => b.child_at(index),
            _ => panic!(),
        }
    }
}

struct Grouper(String);

impl Display for Grouper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AstIndexable for Grouper {
    fn num_children(&self) -> usize {
        0
    }

    fn child_at(&self, _index: usize) -> Option<&dyn AstIndexable> {
        panic!()
    }
}

struct CreateParent<'a>(String, Vec<&'a dyn AstIndexable>);

impl Display for CreateParent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AstIndexable for CreateParent<'_> {
    fn num_children(&self) -> usize {
        self.1.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(self.1[index])
    }
}

struct CreateParentBx(String, Vec<Box<dyn AstIndexable>>);

impl Display for CreateParentBx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AstIndexable for CreateParentBx {
    fn num_children(&self) -> usize {
        self.1.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn AstIndexable> {
        Some(&*self.1[index])
    }
}
