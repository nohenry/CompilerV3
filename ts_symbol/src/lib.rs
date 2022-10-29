#![feature(box_patterns)]
use std::{collections::HashMap, ffi::CString, fmt::Display, rc::Rc};

use linked_hash_map::LinkedHashMap;
use ts_errors::CodeGenError;
// use ts_lexer::ast::{Expression, FunctionSignature, ParseNode, TypeSymbol};
use ts_lexer::new_ast::{Expression, FunctionDecleration , Statement, Type as TypeSymbol};
use ts_util::{CreateParent, Grouper, TreeDisplay, NULL_STR};

use bitflags::bitflags;

pub type Path = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum GenericType {
    Generic(Option<Vec<Type>>),
    Specialization(Type),
}

impl ToString for GenericType {
    fn to_string(&self) -> String {
        match self {
            GenericType::Generic(_) => "".to_string(),
            GenericType::Specialization(sp) => sp.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Empty,
    Literal {
        literal_type: Type,
    },
    Variable {
        variable_type: Type,
        constant: bool,
    },
    Function {
        function_type: Type,
    },
    MemberFunction {
        func: Box<Value>,
        var: Box<Value>,
    },
    FunctionTemplate {
        path: Vec<String>,
        ty: Rc<FunctionDecleration>,
        body: Rc<Statement>,
        ty_params: LinkedHashMap<String, GenericType>,
        existing: HashMap<Vec<String>, Vec<String>>,
        specialization: HashMap<Vec<String>, Vec<String>>,
    },
    SpecFunction {
        parameters: Vec<(String, Type)>,
        return_type: Type,
    },
    Template {
        template_type: Type,
    },
    TemplateFields {
        fields: HashMap<String, Value>,
        template_type: Type,
    },
    Path {
        path: Vec<String>,
    },
}

impl Default for Value {
    fn default() -> Self {
        Self::Empty
    }
}

impl Value {
    pub fn get_type(&self) -> &Type {
        match self {
            Self::Literal { literal_type, .. } => literal_type,
            Self::Variable { variable_type, .. } => variable_type,
            Self::Function { function_type, .. } => function_type,
            Self::MemberFunction { func, .. } => func.get_type(),
            Self::Template { template_type, .. } => template_type,
            Self::TemplateFields { template_type, .. } => template_type,
            _ => {
                panic!("Called on unkown value!")
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::Empty => true,
            _ => false,
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            Value::Variable {
                constant,
                variable_type,
                ..
            } => *constant | variable_type.is_const(),
            _ => panic!(),
        }
    }

    pub fn has_value(&self) -> bool {
        match self {
            Value::Empty => false,
            _ => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal { literal_type, .. } => write!(f, "{}", literal_type),
            Value::Variable { variable_type, .. } => write!(f, "{}", variable_type),
            Value::Function { function_type, .. } => write!(f, "{}", function_type),
            Value::SpecFunction { .. } => write!(f, ""),
            Value::MemberFunction { func, .. } => write!(f, "{}", func),
            Value::Template { template_type, .. } => write!(f, "{}", template_type),
            Value::TemplateFields { template_type, .. } => write!(f, "{}", template_type),
            Value::FunctionTemplate { .. } => write!(f, "Function Template"),
            Value::Empty => write!(f, "Empty"),
            Value::Path { path } => write!(f, "Path {:?}", path),
        }
    }
}

impl TreeDisplay for Value {
    fn num_children(&self) -> usize {
        match self {
            Value::Literal { literal_type, .. } => literal_type.num_children(),
            Value::Variable { variable_type, .. } => variable_type.num_children(),
            Value::Function { function_type, .. } => function_type.num_children(),
            Value::Template { template_type, .. } => template_type.num_children(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Value::Literal { literal_type, .. } => literal_type.child_at(index),
            Value::Variable { variable_type, .. } => variable_type.child_at(index),
            Value::Function { function_type, .. } => function_type.child_at(index),
            Value::Template { template_type, .. } => template_type.child_at(index),
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        match self {
            Value::Function { function_type, .. } => function_type.child_at_bx(index),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Empty,
    Unit,
    Number(f64),
    Boolean(bool),
    String(String),
    Array(LinkedHashMap<i64, Type>),
    Object {
        base_type: Box<Type>,
        constant: bool,

        fields: HashMap<String, Type>,
    },
    Function {
        parameters: LinkedHashMap<String, Type>,
        return_type: Box<Type>,

        path: Path,
    },
    Class {
        fields: LinkedHashMap<String, Type>,
        path: Path,
    },
    TemplateTemplate {
        path: Path,
        fields: Rc<Vec<TypeSymbol>>,
        ty_params: LinkedHashMap<String, GenericType>,

        existing: HashMap<Vec<String>, Vec<String>>,
        specialization: HashMap<Vec<String>, Vec<String>>,
    },
    Generic {
        base_type: Box<Type>,
        parameters: Vec<Type>,
    },
    Interface {
        path: Path,
    },
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Object {
                    base_type: l_base_type,
                    constant: l_constant,
                    ..
                },
                Self::Object {
                    base_type: r_base_type,
                    constant: r_constant,
                    ..
                },
            ) => l_base_type == r_base_type && l_constant == r_constant,
            (
                Self::Function {
                    parameters: l_parameters,
                    return_type: l_return_type,
                    ..
                },
                Self::Function {
                    parameters: r_parameters,
                    return_type: r_return_type,
                    ..
                },
            ) => {
                for p in r_parameters.values().zip(l_parameters.values()) {
                    if p.0 != p.1 {
                        return false;
                    }
                }

                if l_return_type != r_return_type {
                    return false;
                }

                true
            }
            (
                Self::Class {
                    fields: l_fields,
                    path: l_path,
                },
                Self::Class {
                    fields: r_fields,
                    path: r_path,
                },
            ) => l_fields == r_fields && l_path == r_path,
            (
                Self::TemplateTemplate {
                    path: l_path,
                    fields: l_fields,
                    ty_params: l_ty_params,
                    existing: l_existing,
                    specialization: l_specialization,
                },
                Self::TemplateTemplate {
                    path: r_path,
                    fields: r_fields,
                    ty_params: r_ty_params,
                    existing: r_existing,
                    specialization: r_specialization,
                },
            ) => {
                let eq: Option<()> = l_ty_params
                    .iter()
                    .zip(r_ty_params.iter())
                    .map(|(l, r)| {
                        if l.0 == r.0 && l.1 == r.1 {
                            Some(())
                        } else {
                            None
                        }
                    })
                    .collect();
                l_path == r_path
                // TODO: idk
                    // && l_fields == r_fields
                    && l_existing == r_existing
                    && l_specialization == r_specialization
            }
            (
                Self::Generic {
                    base_type: l_base_type,
                    parameters: l_parameters,
                },
                Self::Generic {
                    base_type: r_base_type,
                    parameters: r_parameters,
                },
            ) => l_base_type == r_base_type && l_parameters == r_parameters,
            (Self::Interface { path: l_path }, Self::Interface { path: r_path }) => {
                l_path == r_path
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Type {
    pub fn is_empty(&self) -> bool {
        match self {
            Type::Empty => true,
            _ => false,
        }
    }

    pub fn unit_ty() -> Type {
        Type::Unit {}
    }

    pub fn resolve_properties<'a>(&'a self) -> Option<&'a LinkedHashMap<String, Type>> {
        match self {
            Type::Class { fields, .. } => Some(fields),
            _ => None,
        }
    }

    pub fn resolve_base_type(&self) -> &Type {
        match self {
            Type::Object { base_type, .. } => base_type.resolve_base_type(),
            _ => self,
        }
    }

    pub fn resolve_path(&self) -> Option<Vec<String>> {
        match self {
            Type::Class { path, .. } => Some(path.clone()),
            Type::TemplateTemplate { path, .. } => Some(path.clone()),
            Type::Number { .. } => Some(vec!["core".to_string(), format!("number")]),

            Type::Boolean { .. } => Some(vec!["core".to_string(), format!("boolean")]),
            Type::String { .. } => Some(vec!["core".to_string(), format!("string")]),
            Type::Generic { base_type, .. } => base_type.resolve_path(),
            _ => None,
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            Type::Object { constant, .. } => *constant,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Unit { .. } => f.write_str("()"),
            Self::Number { .. } => {
                write!(f, "numberl")
            }

            Self::Boolean { .. } => write!(f, "bool"),
            Self::String { .. } => write!(f, "string"),
            Self::Array { .. } => {
                write!(f, "[]")
            }
            Self::Function {
                parameters,
                return_type,
                ..
            } => {
                write!(f, "(")?;
                if parameters.len() >= 1 {
                    let mut iter = parameters.iter();
                    write!(f, "{}", iter.next().unwrap().1)?;
                    for (_, t) in iter {
                        write!(f, ", {}", t)?;
                    }
                }
                write!(f, ")")?;
                if let Type::Unit { .. } = **return_type {
                    write!(f, " =>")
                } else {
                    write!(f, ":{} =>", return_type)
                }
            }
            Self::Object { base_type, .. } => {
                write!(f, "{}", base_type)
            }
            Self::String { .. } => {
                write!(f, "string")
            }
            Self::Class { .. } => {
                write!(f, "**Class**")
            }
            Self::Interface { .. } => {
                write!(f, "**Interface**")
            }
            Self::TemplateTemplate { path, .. } => {
                if let Some(p) = path.last() {
                    write!(f, "{}", p)
                } else {
                    write!(f, "**Template**")
                }
            }
            Self::Generic { .. } => {
                write!(f, "**Generic**")
            } // Self::(GenericType {
              //     arguments,
              //     base_type,
              //     ..
              // }) => {
              //     write!(f, "{}<", base_type)?;
              //     write!(f, "{}", arguments[0])?;
              //     for t in &arguments[1..] {
              //         write!(f, ", {}", t)?;
              //     }
              //     write!(f, ">")
              // }
        }
    }
}

impl TreeDisplay for Type {
    fn num_children(&self) -> usize {
        match self {
            Type::Object { .. } => 1,
            Type::Function { parameters, .. } => parameters.len() + 1,
            Type::Class { fields, .. } => fields.len(),
            _ => 0,
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match self {
            Type::Object { base_type, .. } => Some(base_type.as_ref()),
            Type::Function { return_type, .. } if index == 0 => Some(return_type.as_ref()),
            Type::Function { .. } => None,
            Type::Class { .. } => None,
            _ => panic!(),
        }
    }

    fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
        match self {
            Type::Function { parameters, .. } => Box::new(CreateParent(
                parameters.keys().nth(index - 1).unwrap().clone(),
                vec![parameters.values().nth(index - 1).unwrap()],
            )),
            Type::Class { fields, .. } => Box::new(Grouper(format!(
                "{}: {}",
                fields.keys().nth(index).unwrap(),
                fields.values().nth(index).unwrap()
            ))),
            _ => panic!(),
        }
    }
}

bitflags! {
    pub struct SymbolFlags: u32 {
        const CONSTANT = 0b00000001;
        const EXPORT = 0b00000010;
    }
}

pub enum SymbolValue {
    Empty,
    Variable(Value),
    Function(Value),
    // SpecFunction(Value),
    Field(Type),
    Template(Type),
    Action(Type),
    Spec(Type),
    Alias(Type),
    Generic(Type, GenericType),
    Primitive(Type),
    Macro(Box<dyn Fn(&Vec<Expression>) -> Statement>),
    Module,
}

impl std::fmt::Debug for SymbolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Variable(arg0) => f.debug_tuple("Variable").field(arg0).finish(),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
            Self::Field(arg0) => f.debug_tuple("Field").field(arg0).finish(),
            Self::Template(arg0) => f.debug_tuple("Template").field(arg0).finish(),
            Self::Action(arg0) => f.debug_tuple("Action").field(arg0).finish(),
            Self::Spec(arg0) => f.debug_tuple("Spec").field(arg0).finish(),
            Self::Alias(arg0) => f.debug_tuple("Alias").field(arg0).finish(),
            Self::Generic(arg0, arg1) => f.debug_tuple("Generic").field(arg0).field(arg1).finish(),
            Self::Primitive(arg0) => f.debug_tuple("Primitive").field(arg0).finish(),
            Self::Macro(arg0) => f.debug_tuple("Macro").finish(),
            Self::Module => write!(f, "Module"),
        }
    }
}

impl SymbolValue {
    pub fn is_const(&self) -> bool {
        match self {
            Self::Variable(v) => v.is_const(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub value: SymbolValue,
    pub children: LinkedHashMap<String, Symbol>,
    pub flags: SymbolFlags,
}

impl Symbol {
    pub fn root() -> Symbol {
        Symbol {
            name: String::from("root"),
            value: SymbolValue::Empty,
            children: LinkedHashMap::new(),
            flags: SymbolFlags::empty(),
        }
    }

    pub fn new(name: String, value: SymbolValue) -> Symbol {
        Symbol {
            name,
            value,
            children: LinkedHashMap::new(),
            flags: SymbolFlags::empty(),
        }
    }

    pub fn new_flags(name: String, value: SymbolValue, flags: SymbolFlags) -> Symbol {
        Symbol {
            name,
            value,
            children: LinkedHashMap::new(),
            flags,
        }
    }

    pub fn add_child<T: ToString>(&mut self, name: &T, value: SymbolValue) -> &mut Symbol {
        self.children
            .insert(name.to_string(), Symbol::new(name.to_string(), value));
        self.children.get_mut(&name.to_string()).unwrap()
    }

    pub fn add_child_flags<T: ToString>(
        &mut self,
        name: &T,
        value: SymbolValue,
        flags: SymbolFlags,
    ) -> &mut Symbol {
        self.children.insert(
            name.to_string(),
            Symbol::new_flags(name.to_string(), value, flags),
        );
        self.children.get_mut(&name.to_string()).unwrap()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.flags.is_empty() {
            match self.value {
                SymbolValue::Variable(_) => write!(f, "Variable `{}`", self.name),
                SymbolValue::Action(_) => write!(f, "Action `{}`", self.name),
                SymbolValue::Alias(_) => write!(f, "Alias `{}`", self.name),
                SymbolValue::Empty => write!(f, "{}", self.name),
                SymbolValue::Field(_) => write!(f, "Field `{}`", self.name),
                SymbolValue::Macro(_) => write!(f, "Maro `{}`", self.name),
                SymbolValue::Function(_) => write!(f, "Function `{}`", self.name),
                // SymbolValue::SpecFunction(_) => write!(f, "Spec Function `{}`", self.name),
                SymbolValue::Module => write!(f, "Module `{}`", self.name),
                SymbolValue::Spec(_) => write!(f, "Spec `{}`", self.name),
                SymbolValue::Template(_) => write!(f, "Template `{}`", self.name),
                SymbolValue::Primitive(_) => write!(f, "Primititve `{}`", self.name),
                SymbolValue::Generic(_, _) => write!(f, "Generic `{}`", self.name),
            }
        } else {
            match self.value {
                SymbolValue::Variable(_) => {
                    write!(f, "Variable `{}` ({:?})", self.name, self.flags)
                }
                SymbolValue::Action(_) => write!(f, "Action `{}` ({:?})", self.name, self.flags),
                SymbolValue::Alias(_) => write!(f, "Alias `{}` ({:?})", self.name, self.flags),
                SymbolValue::Empty => write!(f, "{} ({:?})", self.name, self.flags),
                SymbolValue::Macro(_) => write!(f, "Macro {} ({:?})", self.name, self.flags),
                SymbolValue::Field(_) => write!(f, "Field `{}` ({:?})", self.name, self.flags),
                SymbolValue::Function(_) => {
                    write!(f, "Function `{}` ({:?})", self.name, self.flags)
                }
                // SymbolValue::SpecFunction(_) => {
                // write!(f, "Spec Function `{}` ({:?})", self.name, self.flags)
                // }
                SymbolValue::Module => write!(f, "Module `{}` ({:?})", self.name, self.flags),
                SymbolValue::Spec(_) => write!(f, "Spec `{}` ({:?})", self.name, self.flags),
                SymbolValue::Template(_) => {
                    write!(f, "Template `{}` ({:?})", self.name, self.flags)
                }
                SymbolValue::Primitive(_) => {
                    write!(f, "Primititve `{}` {:?}", self.name, self.flags)
                }
                SymbolValue::Generic(_, _) => write!(f, "Generic `{}` {:?}", self.name, self.flags),
            }
        }
    }
}

impl TreeDisplay for Symbol {
    fn num_children(&self) -> usize {
        match &self.value {
            SymbolValue::Generic(_, ty) => match ty {
                GenericType::Generic(Some(t)) => t.len(),
                GenericType::Specialization(_) => 1,
                _ => 0,
            },
            _ => self.children.len(),
        }
    }

    fn child_at(&self, index: usize) -> Option<&dyn TreeDisplay> {
        match &self.value {
            SymbolValue::Generic(_, ty) => match ty {
                GenericType::Generic(Some(t)) => Some(&t[index]),
                GenericType::Specialization(t) => Some(t),
                _ => panic!(),
            },
            _ => Some(self.children.values().nth(index).unwrap()),
        }
    }

    // fn child_at_bx<'a>(&'a self, index: usize) -> Box<dyn TreeDisplay + 'a> {
    //     let p = self.children.values().nth(index).unwrap();
    //     Box::new(CreateParent(
    //         self.children.keys().nth(index).unwrap().clone(),
    //         vec![p],
    //     ))
    // }
}
