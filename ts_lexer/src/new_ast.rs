use std::fmt::Display;

use colored::{ColoredString, Colorize};
use ts_util::{Grouper, TreeDisplay};

use crate::OperatorKind;

#[derive(Debug)]
pub struct TypeParameter {
    pub params: Vec<(String, Option<Type>)>,
}

impl Display for TypeParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColoredString::from("Type Parameter").cyan(),)
    }
}

impl TreeDisplay for TypeParameter {
    fn num_children(&self) -> usize {
        self.params.len()
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        None
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        let p = &self.params[index];
        if let Some(p1) = &p.1 {
            Box::new(Grouper(format!("{}: {}", p.0, p1)))
        } else {
            Box::new(Grouper(format!("{}", p.0)))
        }
    }
}

#[derive(Debug)]
pub struct TypeArgument {
    pub args: Vec<Type>,
}

#[derive(Debug)]
pub enum PropertyName {
    Identifier(String),
    String(String),
    Numeric(f64),
    Computed(Box<Expression>),
}

impl Display for PropertyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(iden) => write!(f, "{}", iden),
            Self::String(s) => write!(f, "'{}'", s),
            Self::Numeric(s) => write!(f, "{}", s),
            Self::Computed(s) => write!(f, "Computed"),
        }
    }
}

#[derive(Debug)]
pub enum Accessibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug)]
pub enum Binding {
    Identifier(String),
    ObjectPattern(Vec<(Box<Binding>, Option<Box<Expression>>)>),
    ArrayPattern(Vec<(Box<Binding>, Option<Box<Expression>>)>),
}

impl Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binding::Identifier(ident) => write!(f, "{}", ident),
            Binding::ObjectPattern(bind) => {
                write!(f, "{{")?;
                for b in bind {
                    write!(f, "{}", b.0)?;
                    if let Some(eq) = &b.1 {
                        write!(f, " = {}", eq)?;
                    }
                }
                write!(f, "}}")
            }
            Binding::ArrayPattern(bind) => {
                write!(f, "[")?;
                for b in bind {
                    write!(f, "{}", b.0)?;
                    if let Some(eq) = &b.1 {
                        write!(f, " = {}", eq)?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub modifiers: Vec<Accessibility>,
    pub name: Binding,
    pub ty: Option<Box<Type>>,
    pub initializer: Option<Box<Expression>>,
    pub optional: bool,
    pub rest: bool,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rest {
            write!(f, "...")?;
        }
        write!(
            f,
            "{}",
            self.modifiers
                .iter()
                .map(|f| format!("{:?}", f))
                .collect::<Vec<_>>()
                .join(" ")
        )?;
        write!(f, "{}", self.name)?;
        if self.optional {
            write!(f, "?")?;
        }
        if let Some(ty) = &self.ty {
            write!(f, ": {}", ty)?;
        }
        if let Some(init) = &self.initializer {
            write!(f, " = {}", init)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CallSignature {
    pub ty_params: TypeParameter,
    pub params: Vec<Parameter>,
    pub return_type: Box<Type>,
}

impl Display for CallSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ty_params.params.len() > 0 {
            write!(f, "<")?;
            write!(
                f,
                "{}",
                self.ty_params
                    .params
                    .iter()
                    .map(|f| {
                        if let Some(v) = &f.1 {
                            format!("{}: {}", f.0, v)
                        } else {
                            format!("{}", f.0)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(".")
            )?;
            write!(f, ">")?;
        }

        write!(f, "(")?;
        write!(
            f,
            "{}",
            self.params
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join(".")
        )?;
        write!(f, ")")?;

        write!(f, "{}", self.return_type)
    }
}

#[derive(Debug)]
pub struct IndexSignature {
    pub name: String,
    pub ty: Box<Type>,
    pub ret_ty: Box<Type>,
}

impl Display for IndexSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}: {}]: {}", self.name, self.ty, self.ret_ty)
    }
}

#[derive(Debug)]
pub enum TypeMember {
    PropertySignature {
        name: PropertyName,
        optional: bool,
        ty: Option<Box<Type>>,
    },
    CallSignature(CallSignature),
    ConstructSignature {
        ty_params: TypeParameter,
        params: Vec<Parameter>,
        ret_ty: Box<Type>,
    },
    IndexSignature(IndexSignature),
    MethodSignature {
        name: PropertyName,
        optional: bool,
        call: CallSignature,
    },
}

#[derive(Debug)]
pub struct TypeReference {
    pub name: String,
    pub arguments: Option<TypeArgument>,
}

impl Display for TypeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(args) = &self.arguments {
            write!(
                f,
                "{}",
                args.args
                    .iter()
                    .map(|f| format!("{}", f))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ObjectType {
    pub members: Vec<TypeMember>,
}

#[derive(Debug)]
pub enum Type {
    Any,
    Number,
    Boolean,
    String,
    Symbol,
    Void,
    Null,
    Undefined,
    TypeReference(TypeReference),
    Object(ObjectType),
    Array {
        base: Box<Type>,
    },
    Tuple {
        tys: Vec<Type>,
    },
    Union {
        tys: Vec<Type>,
    },
    Intersection {
        tys: Vec<Type>,
    },
    TypeQuery {
        tys: Vec<Type>,
    },
    This,
    Function {
        ty_params: TypeParameter,
        params: Vec<Parameter>,
        ret_ty: Box<Type>,
    },
    Constructor {
        ty_params: TypeParameter,
        params: Vec<Parameter>,
        ret_ty: Box<Type>,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => f.write_str("any"),
            Self::Union { tys } => {
                write!(
                    f,
                    "{}",
                    tys.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            }
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Boolean => write!(f, "boolean"),
            Self::Symbol => write!(f, "symbol"),
            Self::Void => write!(f, "void"),
            Self::Null => write!(f, "null"),
            Self::Undefined => write!(f, "undefined"),
            Self::This => write!(f, "this"),
            Self::Array { base } => {
                write!(f, "[{}]", base)
            }
            Self::Object(o) => {
                write!(f, "Object")
            }
            Self::Tuple { tys } => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    tys.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
                write!(f, "]")
            }
            Self::Intersection { tys } => {
                write!(
                    f,
                    "{}",
                    tys.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(" & ")
                )
            }
            Self::TypeQuery { tys } => {
                write!(
                    f,
                    "{}",
                    tys.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(".")
                )
            }
            Self::Function {
                params,
                ret_ty,
                ty_params,
            } => {
                if ty_params.params.len() > 0 {
                    write!(f, "<")?;
                    write!(
                        f,
                        "{}",
                        ty_params
                            .params
                            .iter()
                            .map(|f| {
                                if let Some(v) = &f.1 {
                                    format!("{}: {}", f.0, v)
                                } else {
                                    format!("{}", f.0)
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(".")
                    )?;
                    write!(f, ">")?;
                }

                write!(f, "(")?;
                write!(
                    f,
                    "{}",
                    params
                        .iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(".")
                )?;
                write!(f, ")")?;

                write!(f, "{}", ret_ty)
            }
            Self::Constructor {
                params,
                ret_ty,
                ty_params,
            } => {
                write!(f, "new")?;
                if ty_params.params.len() > 0 {
                    write!(f, "<")?;
                    write!(
                        f,
                        "{}",
                        ty_params
                            .params
                            .iter()
                            .map(|f| {
                                if let Some(v) = &f.1 {
                                    format!("{}: {}", f.0, v)
                                } else {
                                    format!("{}", f.0)
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(".")
                    )?;
                    write!(f, ">")?;
                }

                write!(f, "(")?;
                write!(
                    f,
                    "{}",
                    params
                        .iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(".")
                )?;
                write!(f, ")")?;

                write!(f, "{}", ret_ty)
            }
            Self::TypeReference(r) => {
                write!(f, "{}", r)
            }
        }
    }
}

impl TreeDisplay for Type {
    fn num_children(&self) -> usize {
        match self {
            Self::Object(s) => s.members.len(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        None
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        match self {
            Self::Object(ObjectType { members }) => match &members[index] {
                TypeMember::PropertySignature { name, optional, ty } => Box::new(Grouper(format!(
                    "{}{}{}",
                    name,
                    if *optional { "?" } else { "" },
                    if let Some(ty) = &ty {
                        format!("{}", ty)
                    } else {
                        "".to_string()
                    }
                ))),
                TypeMember::CallSignature(c) => Box::new(Grouper(format!("{}", c))),
                TypeMember::ConstructSignature {
                    params,
                    ret_ty,
                    ty_params,
                } => {
                    let ar = if ty_params.params.len() > 0 {
                        let p = format!(
                            "{}",
                            ty_params
                                .params
                                .iter()
                                .map(|f| {
                                    if let Some(v) = &f.1 {
                                        format!("{}: {}", f.0, v)
                                    } else {
                                        format!("{}", f.0)
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(".")
                        );
                        format!("<{}>", p)
                    } else {
                        String::from("")
                    };
                    let p = format!(
                        "{}",
                        params
                            .iter()
                            .map(|f| format!("{}", f))
                            .collect::<Vec<_>>()
                            .join(".")
                    );
                    Box::new(Grouper(format!("new {}({}): {}", ar, p, ret_ty)))
                }
                TypeMember::MethodSignature {
                    call,
                    name,
                    optional,
                } => Box::new(Grouper(format!(
                    "{}{}{}",
                    name,
                    if *optional { "?" } else { "" },
                    call
                ))),
                TypeMember::IndexSignature(i) => Box::new(Grouper(i.to_string())),
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum Property {
    Ident {
        name: String,
    },
    Initializer {
        name: String,
        expr: Box<Expression>,
    },
    Method {
        name: PropertyName,
        params: Vec<Parameter>,
        get: bool,
        set: bool,
        body: Block,
    },
    Field {
        name: PropertyName,
        expr: Box<Expression>,
    },
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident { name } => write!(f, "{}", name),
            Self::Initializer { name, expr } => write!(f, "{}: {}", name, expr),
            Self::Field { name, expr } => write!(f, "{}: {}", name, expr),
            Self::Method {
                name,
                params,
                get,
                set,
                body,
            } => {
                if *get {
                    write!(f, "get")?;
                }
                if *set {
                    write!(f, "set")?;
                }
                write!(f, "{}", name)?;
                write!(
                    f,
                    "{}",
                    params
                        .iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(".")
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Null,
    This,
    Ident(String),
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Expression>),
    Object(Vec<Property>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(
                f,
                "{}",
                ColoredString::from(format!("null").as_str()).yellow()
            ),
            Self::This => write!(
                f,
                "{}",
                ColoredString::from(format!("this").as_str()).yellow()
            ),
            Self::Ident(s) => write!(f, "{}", s),
            Self::Boolean(b) => write!(
                f,
                "{}",
                ColoredString::from(format!("{}", b).as_str()).yellow()
            ),
            Self::Number(b) => write!(
                f,
                "{}",
                ColoredString::from(format!("{}", b).as_str()).yellow()
            ),
            Self::String(b) => write!(
                f,
                "'{}'",
                ColoredString::from(format!("{}", b).as_str()).yellow()
            ),
            Self::Array(b) => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    b.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(".")
                )?;
                write!(f, "]")
            }
            Self::Object(b) => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    b.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<_>>()
                        .join(".")
                )?;
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug)]
pub enum ArrowParams {
    Ident(String),
    Parens(Vec<Parameter>),
}

impl Display for ArrowParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrowParams::Ident(s) => write!(f, "{}", s)?,
            ArrowParams::Parens(pars) => pars.iter().enumerate().for_each(|(s, p)| {
                if s == 0 {
                    write!(f, "({}, ", p);
                } else if s == pars.len() - 1 {
                    write!(f, "{})", p);
                } else {
                    write!(f, "{}, ", p);
                }
            }),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ArrowBody {
    Expr(Box<Expression>),
    Stmt(Box<Statement>),
}

impl Display for ArrowBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrowBody::Expr(e) => write!(f, "{}", &*e),
            ArrowBody::Stmt(e) => {Ok(())}, //write!(f, "{{{}}}", &*e),
        }
    }
}





#[derive(Debug)]
pub struct Arguments {
    pub ty_args: Option<TypeArgument>,
    pub args: Vec<Expression>,
}

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if let Some(tys) = self.ty_args {
        //     for (i, a) in self.args.iter().enumerate() {
        //         if i == self.args.len() - 1 {
        //             write!(f, "{}", a)
        //         } else {
        //             write!(f, "{}, ", a)
        //         }
        //     }
        // } else {
        for (i, a) in self.args.iter().enumerate() {
            if i == self.args.len() - 1 {
                write!(f, "{}", a)?
            } else {
                write!(f, "{}, ", a)?
            }
        }
        Ok(())
        // }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    ArrowFunction {
        params: ArrowParams,
        body: ArrowBody,
    },
    Yield(Option<Box<Expression>>),
    BinaryExpression {
        left: Box<Expression>,
        op: OperatorKind,
        right: Box<Expression>,
    },
    UnaryExpression {
        op: OperatorKind,
        expr: Box<Expression>,
    },
    Conditional {
        cond: Box<Expression>,
        then: Box<Expression>,
        not: Box<Expression>,
    },
    Assign {
        op: OperatorKind,
    },
    Member {
        expr: Box<Expression>,
        member: Box<Expression>,
    },
    Super,
    New {
        expr: Box<Expression>,
        args: Option<Arguments>,
    },
    Call {
        expr: Box<Expression>,
        args: Arguments,
    },
    Spread {
        expr: Box<Expression>,
    },
    Export,
    Empty,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::ArrowFunction { params, body } => write!(f, "{} => {}", params, body),
            Self::Yield(s) => {
                if let Some(ye) = s {
                    write!(f, "yield {}", &*ye)
                } else {
                    write!(f, "yield")
                }
            }
            Self::BinaryExpression { left, op, right } => {
                write!(f, "{} {} {}", &*left, op, &*right)
            }
            Self::UnaryExpression { op, expr } => {
                write!(f, "{} {}", op, &*expr)
            }
            Self::Conditional { cond, then, not } => {
                write!(f, "{} ? {} : {}", &*cond, &*then, &*not)
            }
            Self::Assign { op } => {
                write!(f, "{}", op)
            }
            Self::Member { expr, member } => {
                write!(f, "{}[{}]", &*expr, &*member)
            }
            Self::Super => {
                write!(f, "super")
            }
            Self::New { expr, args } => {
                if let Some(args) = args {
                    write!(f, "new {}({})", &*expr, args)
                } else {
                    write!(f, "new {}()", &*expr)
                }
            }
            Self::Call { expr, args } => {
                write!(f, "{}({})", &*expr, args)
            }
            Self::Spread { expr } => {
                write!(f, "...{}", &*expr)
            }
            Self::Export => {
                write!(f, "export")
            }
            _ => Ok(()),
        }
    }
}


// impl TreeDisplay for Expression {
//     fn num_children(&self) -> usize {
//         self.params.len()
//     }

//     fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
//         None
//     }

//     fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
//         let p = &self.params[index];
//         if let Some(p1) = &p.1 {
//             Box::new(Grouper(format!("{}: {}", p.0, p1)))
//         } else {
//             Box::new(Grouper(format!("{}", p.0)))
//         }
//     }
// }

#[derive(Debug)]
pub struct VariableDecleration {
    pub name: Binding,
    pub init: Option<Box<Expression>>,
    pub constant: bool,
}

#[derive(Debug)]
pub enum Loop {
    While {
        expr: Box<Expression>,
        body: Box<Statement>,
    },
    For {
        init: Box<Statement>,
        cond: Box<Expression>,
        update: Box<Expression>,
    },
    ForOf {
        name: Binding,
        constant: bool,
        expr: Box<Expression>,
    },
    ForIn {
        name: Binding,
        constant: bool,
        expr: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum LabelItem {
    Statement(Box<Statement>),
    Function(FunctionDecleration),
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Block(Block),
    Variable(Vec<VariableDecleration>),
    Expression(Box<Expression>),
    If {
        expr: Box<Expression>,
        body: Box<Statement>,
        else_clause: Box<Statement>,
    },
    Loop(Loop),
    Continue(Option<String>),
    Break(Option<String>),
    Return(Option<Box<Expression>>),
    Switch {
        expr: Box<Expression>,
        cases: Vec<(Expression, Vec<Statement>)>,
        default: Vec<Statement>,
    },
    Label(String, LabelItem),
    Throw(Box<Expression>),
    Try {
        body: Box<Block>,
        catch: Option<(Binding, Box<Block>)>,
        finally: Option<Box<Block>>,
    },
    Debugger,
    Function(FunctionDecleration),
    Class(ClassDecleration),
    Interface(Interface),
    TypeAlias {
        name: String,
        ty_params: TypeParameter,
        ty: Box<Type>,
    },
    Enum(Enum),
    Module {
        path: Vec<String>,
        stmts: Vec<Statement>,
    },
    Export {
        stmt: Option<Box<Statement>>,
        from: Option<Box<Statement>>,
        default: bool,
    },
    ExportClause {
        names: Vec<(String, Option<String>)>,
    },
}

#[derive(Debug)]
pub struct FunctionDecleration {
    pub name: String,
    pub call: CallSignature,
    pub body: Option<Block>,
}

#[derive(Debug)]
pub struct ClassHeritage {
    pub extends: Option<TypeReference>,
    pub implements: Option<Vec<TypeReference>>,
}

#[derive(Debug)]
pub struct GetAccessor {
    pub name: PropertyName,
    pub ty: Option<Box<Type>>,
    pub body: Block,
}

#[derive(Debug)]
pub struct SetAccessor {
    pub name: PropertyName,

    pub pram_name: Binding,
    pub pram_ty: Option<Box<Type>>,

    pub body: Block,
}

#[derive(Debug)]
pub enum ClassElement {
    Constructor {
        accessibility: Accessibility,
        body: Option<Block>,
    },
    MemberVariable {
        accessibility: Accessibility,
        is_static: bool,
        name: PropertyName,
        ty: Option<Box<Type>>,
        init: Box<Expression>,
    },
    MemberFunction {
        accessibility: Accessibility,
        is_static: bool,
        name: PropertyName,
        call: CallSignature,
        body: Option<Block>,
    },
    MemberGet {
        accessibility: Accessibility,
        is_static: bool,
        get: GetAccessor,
    },
    MemberSet {
        accessibility: Accessibility,
        is_static: bool,
        set: SetAccessor,
    },
    Index(IndexSignature),
}

#[derive(Debug)]
pub struct ClassDecleration {
    pub ident: Option<String>,
    pub ty_params: TypeParameter,
    pub heritage: ClassHeritage,
    pub body: Vec<ClassElement>,
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub ty_params: TypeParameter,
    pub extends: Vec<TypeReference>,
    pub object_ty: ObjectType,
}

#[derive(Debug)]
pub struct EnumMember {
    pub name: PropertyName,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct Enum {
    pub is_const: bool,
    pub name: String,
    pub body: Vec<EnumMember>,
}

#[derive(Debug)]
pub struct SourceFile {
    pub statements: Vec<Statement>,
}
