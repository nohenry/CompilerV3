#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod lexer {
    use std::{collections::hash_map::RandomState, fmt, iter::Peekable, str::Chars};
    use trielib::*;
    pub enum Literal {
        Empty,
        Integer(u64, u8),
        Float(f64),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Literal {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Literal::Empty,) => ::core::fmt::Formatter::write_str(f, "Empty"),
                (&Literal::Integer(ref __self_0, ref __self_1),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_1));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Literal::Float(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Float");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Literal {
        #[inline]
        fn clone(&self) -> Literal {
            match (&*self,) {
                (&Literal::Empty,) => Literal::Empty,
                (&Literal::Integer(ref __self_0, ref __self_1),) => Literal::Integer(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                ),
                (&Literal::Float(ref __self_0),) => {
                    Literal::Float(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    pub struct LexError {
        error: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for LexError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                LexError {
                    error: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "LexError");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "error",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for LexError {
        #[inline]
        fn clone(&self) -> LexError {
            match *self {
                LexError {
                    error: ref __self_0_0,
                } => LexError {
                    error: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl fmt::Display for LexError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["invalid first item to double"],
                &match () {
                    _args => [],
                },
            ))
        }
    }
    impl LexError {
        fn new(error: &String) -> Self {
            LexError {
                error: error.clone(),
            }
        }
    }
    use crate::trie;
    use lazy_static::lazy_static;
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct KEYWORD_TRIE {
        __private_field: (),
    }
    #[doc(hidden)]
    static KEYWORD_TRIE: KEYWORD_TRIE = KEYWORD_TRIE {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for KEYWORD_TRIE {
        type Target = trie::TrieNode<Keyword>;
        fn deref(&self) -> &trie::TrieNode<Keyword> {
            #[inline(always)]
            fn __static_ref_initialize() -> trie::TrieNode<Keyword> {
                {
                    let mut root_node = trie::TrieNode::<Keyword>::new();
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "match".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "match".len() - 1 {
                                Some(Keyword::Match)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "when".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "when".len() - 1 {
                                Some(Keyword::When)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "in".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "in".len() - 1 {
                                Some(Keyword::In)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "if".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "if".len() - 1 {
                                Some(Keyword::If)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "elif".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "elif".len() - 1 {
                                Some(Keyword::Elif)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "else".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "else".len() - 1 {
                                Some(Keyword::Else)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "loop".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "loop".len() - 1 {
                                Some(Keyword::Loop)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "return".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "return".len() - 1 {
                                Some(Keyword::Return)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "int".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "int".len() - 1 {
                                Some(Keyword::Int)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "uint".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "uint".len() - 1 {
                                Some(Keyword::Uint)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "float".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "float".len() - 1 {
                                Some(Keyword::Float)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "char".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "char".len() - 1 {
                                Some(Keyword::Char)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "bool".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "bool".len() - 1 {
                                Some(Keyword::Bool)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "template".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "template".len() - 1 {
                                Some(Keyword::Template)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "spec".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "spec".len() - 1 {
                                Some(Keyword::Spec)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "true".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "true".len() - 1 {
                                Some(Keyword::True)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "false".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "false".len() - 1 {
                                Some(Keyword::False)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "import".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "import".len() - 1 {
                                Some(Keyword::Import)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "let".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "let".len() - 1 {
                                Some(Keyword::Let)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "persist".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "persist".len() - 1 {
                                Some(Keyword::Persist)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "or".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "or".len() - 1 {
                                Some(Keyword::Or)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "and".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "and".len() - 1 {
                                Some(Keyword::And)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "null".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "null".len() - 1 {
                                Some(Keyword::Null)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "module".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "module".len() - 1 {
                                Some(Keyword::Module)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "function".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "function".len() - 1 {
                                Some(Keyword::Function)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "type".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "type".len() - 1 {
                                Some(Keyword::Type)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "export".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "export".len() - 1 {
                                Some(Keyword::Export)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "yield".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "yield".len() - 1 {
                                Some(Keyword::Yield)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "as".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "as".len() - 1 {
                                Some(Keyword::As)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "const".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "const".len() - 1 {
                                Some(Keyword::Const)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "action".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "action".len() - 1 {
                                Some(Keyword::Action)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    {
                        let mut current_node = &mut root_node;
                        for (i, c) in "enum".as_bytes().iter().enumerate() {
                            let endpoint: Option<Keyword> = if i == "enum".len() - 1 {
                                Some(Keyword::Enum)
                            } else {
                                None
                            };
                            current_node = current_node.insert(*c, endpoint).as_mut();
                        }
                    };
                    root_node
                }
            }
            #[inline(always)]
            fn __stability() -> &'static trie::TrieNode<Keyword> {
                static LAZY: ::lazy_static::lazy::Lazy<trie::TrieNode<Keyword>> =
                    ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for KEYWORD_TRIE {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub enum Keyword {
        Match,
        When,
        In,
        If,
        Elif,
        Else,
        Loop,
        Return,
        Int,
        Uint,
        Float,
        Char,
        Bool,
        Template,
        Spec,
        True,
        False,
        Import,
        Let,
        Persist,
        Or,
        And,
        Null,
        Module,
        Function,
        Type,
        Export,
        Yield,
        As,
        Const,
        Action,
        Enum,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Keyword {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Keyword::Match,) => ::core::fmt::Formatter::write_str(f, "Match"),
                (&Keyword::When,) => ::core::fmt::Formatter::write_str(f, "When"),
                (&Keyword::In,) => ::core::fmt::Formatter::write_str(f, "In"),
                (&Keyword::If,) => ::core::fmt::Formatter::write_str(f, "If"),
                (&Keyword::Elif,) => ::core::fmt::Formatter::write_str(f, "Elif"),
                (&Keyword::Else,) => ::core::fmt::Formatter::write_str(f, "Else"),
                (&Keyword::Loop,) => ::core::fmt::Formatter::write_str(f, "Loop"),
                (&Keyword::Return,) => ::core::fmt::Formatter::write_str(f, "Return"),
                (&Keyword::Int,) => ::core::fmt::Formatter::write_str(f, "Int"),
                (&Keyword::Uint,) => ::core::fmt::Formatter::write_str(f, "Uint"),
                (&Keyword::Float,) => ::core::fmt::Formatter::write_str(f, "Float"),
                (&Keyword::Char,) => ::core::fmt::Formatter::write_str(f, "Char"),
                (&Keyword::Bool,) => ::core::fmt::Formatter::write_str(f, "Bool"),
                (&Keyword::Template,) => ::core::fmt::Formatter::write_str(f, "Template"),
                (&Keyword::Spec,) => ::core::fmt::Formatter::write_str(f, "Spec"),
                (&Keyword::True,) => ::core::fmt::Formatter::write_str(f, "True"),
                (&Keyword::False,) => ::core::fmt::Formatter::write_str(f, "False"),
                (&Keyword::Import,) => ::core::fmt::Formatter::write_str(f, "Import"),
                (&Keyword::Let,) => ::core::fmt::Formatter::write_str(f, "Let"),
                (&Keyword::Persist,) => ::core::fmt::Formatter::write_str(f, "Persist"),
                (&Keyword::Or,) => ::core::fmt::Formatter::write_str(f, "Or"),
                (&Keyword::And,) => ::core::fmt::Formatter::write_str(f, "And"),
                (&Keyword::Null,) => ::core::fmt::Formatter::write_str(f, "Null"),
                (&Keyword::Module,) => ::core::fmt::Formatter::write_str(f, "Module"),
                (&Keyword::Function,) => ::core::fmt::Formatter::write_str(f, "Function"),
                (&Keyword::Type,) => ::core::fmt::Formatter::write_str(f, "Type"),
                (&Keyword::Export,) => ::core::fmt::Formatter::write_str(f, "Export"),
                (&Keyword::Yield,) => ::core::fmt::Formatter::write_str(f, "Yield"),
                (&Keyword::As,) => ::core::fmt::Formatter::write_str(f, "As"),
                (&Keyword::Const,) => ::core::fmt::Formatter::write_str(f, "Const"),
                (&Keyword::Action,) => ::core::fmt::Formatter::write_str(f, "Action"),
                (&Keyword::Enum,) => ::core::fmt::Formatter::write_str(f, "Enum"),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Keyword {
        #[inline]
        fn clone(&self) -> Keyword {
            match (&*self,) {
                (&Keyword::Match,) => Keyword::Match,
                (&Keyword::When,) => Keyword::When,
                (&Keyword::In,) => Keyword::In,
                (&Keyword::If,) => Keyword::If,
                (&Keyword::Elif,) => Keyword::Elif,
                (&Keyword::Else,) => Keyword::Else,
                (&Keyword::Loop,) => Keyword::Loop,
                (&Keyword::Return,) => Keyword::Return,
                (&Keyword::Int,) => Keyword::Int,
                (&Keyword::Uint,) => Keyword::Uint,
                (&Keyword::Float,) => Keyword::Float,
                (&Keyword::Char,) => Keyword::Char,
                (&Keyword::Bool,) => Keyword::Bool,
                (&Keyword::Template,) => Keyword::Template,
                (&Keyword::Spec,) => Keyword::Spec,
                (&Keyword::True,) => Keyword::True,
                (&Keyword::False,) => Keyword::False,
                (&Keyword::Import,) => Keyword::Import,
                (&Keyword::Let,) => Keyword::Let,
                (&Keyword::Persist,) => Keyword::Persist,
                (&Keyword::Or,) => Keyword::Or,
                (&Keyword::And,) => Keyword::And,
                (&Keyword::Null,) => Keyword::Null,
                (&Keyword::Module,) => Keyword::Module,
                (&Keyword::Function,) => Keyword::Function,
                (&Keyword::Type,) => Keyword::Type,
                (&Keyword::Export,) => Keyword::Export,
                (&Keyword::Yield,) => Keyword::Yield,
                (&Keyword::As,) => Keyword::As,
                (&Keyword::Const,) => Keyword::Const,
                (&Keyword::Action,) => Keyword::Action,
                (&Keyword::Enum,) => Keyword::Enum,
            }
        }
    }
    pub enum TokenKind {
        Literal(Literal),
        Ident(String),
        /// ";"
        Semi,
        /// ","
        Comma,
        /// "."
        Dot,
        /// "("
        OpenParen,
        /// ")"
        CloseParen,
        /// "{"
        OpenBrace,
        /// "}"
        CloseBrace,
        /// "["
        OpenBracket,
        /// "]"
        CloseBracket,
        /// "@"
        At,
        /// "#"
        Pound,
        /// "~"
        Tilde,
        /// "?"
        Question,
        /// ":"
        Colon,
        /// "$"
        Dollar,
        /// "="
        Eq,
        /// "!"
        Bang,
        /// "<"
        Lt,
        /// ">"
        Gt,
        /// "-"
        Minus,
        /// "&"
        And,
        /// "|"
        Or,
        /// "+"
        Plus,
        /// "*"
        Star,
        /// "/"
        Slash,
        /// "^"
        Caret,
        /// "%"
        Percent,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TokenKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&TokenKind::Literal(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Literal");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&TokenKind::Ident(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Ident");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&TokenKind::Semi,) => ::core::fmt::Formatter::write_str(f, "Semi"),
                (&TokenKind::Comma,) => ::core::fmt::Formatter::write_str(f, "Comma"),
                (&TokenKind::Dot,) => ::core::fmt::Formatter::write_str(f, "Dot"),
                (&TokenKind::OpenParen,) => ::core::fmt::Formatter::write_str(f, "OpenParen"),
                (&TokenKind::CloseParen,) => ::core::fmt::Formatter::write_str(f, "CloseParen"),
                (&TokenKind::OpenBrace,) => ::core::fmt::Formatter::write_str(f, "OpenBrace"),
                (&TokenKind::CloseBrace,) => ::core::fmt::Formatter::write_str(f, "CloseBrace"),
                (&TokenKind::OpenBracket,) => ::core::fmt::Formatter::write_str(f, "OpenBracket"),
                (&TokenKind::CloseBracket,) => ::core::fmt::Formatter::write_str(f, "CloseBracket"),
                (&TokenKind::At,) => ::core::fmt::Formatter::write_str(f, "At"),
                (&TokenKind::Pound,) => ::core::fmt::Formatter::write_str(f, "Pound"),
                (&TokenKind::Tilde,) => ::core::fmt::Formatter::write_str(f, "Tilde"),
                (&TokenKind::Question,) => ::core::fmt::Formatter::write_str(f, "Question"),
                (&TokenKind::Colon,) => ::core::fmt::Formatter::write_str(f, "Colon"),
                (&TokenKind::Dollar,) => ::core::fmt::Formatter::write_str(f, "Dollar"),
                (&TokenKind::Eq,) => ::core::fmt::Formatter::write_str(f, "Eq"),
                (&TokenKind::Bang,) => ::core::fmt::Formatter::write_str(f, "Bang"),
                (&TokenKind::Lt,) => ::core::fmt::Formatter::write_str(f, "Lt"),
                (&TokenKind::Gt,) => ::core::fmt::Formatter::write_str(f, "Gt"),
                (&TokenKind::Minus,) => ::core::fmt::Formatter::write_str(f, "Minus"),
                (&TokenKind::And,) => ::core::fmt::Formatter::write_str(f, "And"),
                (&TokenKind::Or,) => ::core::fmt::Formatter::write_str(f, "Or"),
                (&TokenKind::Plus,) => ::core::fmt::Formatter::write_str(f, "Plus"),
                (&TokenKind::Star,) => ::core::fmt::Formatter::write_str(f, "Star"),
                (&TokenKind::Slash,) => ::core::fmt::Formatter::write_str(f, "Slash"),
                (&TokenKind::Caret,) => ::core::fmt::Formatter::write_str(f, "Caret"),
                (&TokenKind::Percent,) => ::core::fmt::Formatter::write_str(f, "Percent"),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TokenKind {
        #[inline]
        fn clone(&self) -> TokenKind {
            match (&*self,) {
                (&TokenKind::Literal(ref __self_0),) => {
                    TokenKind::Literal(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&TokenKind::Ident(ref __self_0),) => {
                    TokenKind::Ident(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&TokenKind::Semi,) => TokenKind::Semi,
                (&TokenKind::Comma,) => TokenKind::Comma,
                (&TokenKind::Dot,) => TokenKind::Dot,
                (&TokenKind::OpenParen,) => TokenKind::OpenParen,
                (&TokenKind::CloseParen,) => TokenKind::CloseParen,
                (&TokenKind::OpenBrace,) => TokenKind::OpenBrace,
                (&TokenKind::CloseBrace,) => TokenKind::CloseBrace,
                (&TokenKind::OpenBracket,) => TokenKind::OpenBracket,
                (&TokenKind::CloseBracket,) => TokenKind::CloseBracket,
                (&TokenKind::At,) => TokenKind::At,
                (&TokenKind::Pound,) => TokenKind::Pound,
                (&TokenKind::Tilde,) => TokenKind::Tilde,
                (&TokenKind::Question,) => TokenKind::Question,
                (&TokenKind::Colon,) => TokenKind::Colon,
                (&TokenKind::Dollar,) => TokenKind::Dollar,
                (&TokenKind::Eq,) => TokenKind::Eq,
                (&TokenKind::Bang,) => TokenKind::Bang,
                (&TokenKind::Lt,) => TokenKind::Lt,
                (&TokenKind::Gt,) => TokenKind::Gt,
                (&TokenKind::Minus,) => TokenKind::Minus,
                (&TokenKind::And,) => TokenKind::And,
                (&TokenKind::Or,) => TokenKind::Or,
                (&TokenKind::Plus,) => TokenKind::Plus,
                (&TokenKind::Star,) => TokenKind::Star,
                (&TokenKind::Slash,) => TokenKind::Slash,
                (&TokenKind::Caret,) => TokenKind::Caret,
                (&TokenKind::Percent,) => TokenKind::Percent,
            }
        }
    }
    pub struct Token {
        token_type: TokenKind,
        range: Range,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Token {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Token {
                    token_type: ref __self_0_0,
                    range: ref __self_0_1,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Token");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "token_type",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "range",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    pub struct Position {
        line: usize,
        character: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Position {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Position {
                    line: ref __self_0_0,
                    character: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Position");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "line",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "character",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    pub type Range = (Position, Position);
    struct TokenIterator<'a> {
        iter: Chars<'a>,
        peeked: Option<Option<<Chars<'a> as Iterator>::Item>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::fmt::Debug for TokenIterator<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TokenIterator {
                    iter: ref __self_0_0,
                    peeked: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TokenIterator");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "iter",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "peeked",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'a> Iterator for TokenIterator<'a> {
        type Item = <Chars<'a> as Iterator>::Item;
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            match self.peeked.take() {
                Some(v) => v,
                None => self.iter.next(),
            }
        }
        #[inline]
        fn count(mut self) -> usize {
            match self.peeked.take() {
                Some(None) => 0,
                Some(Some(_)) => 1 + self.iter.count(),
                None => self.iter.count(),
            }
        }
        #[inline]
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            match self.peeked.take() {
                Some(None) => None,
                Some(v @ Some(_)) if n == 0 => v,
                Some(Some(_)) => self.iter.nth(n - 1),
                None => self.iter.nth(n),
            }
        }
        #[inline]
        fn last(mut self) -> Option<Self::Item> {
            let peek_opt = match self.peeked.take() {
                Some(None) => return None,
                Some(v) => v,
                None => None,
            };
            self.iter.last().or(peek_opt)
        }
        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let peek_len = match self.peeked {
                Some(None) => return (0, Some(0)),
                Some(Some(_)) => 1,
                None => 0,
            };
            let (lo, hi) = self.iter.size_hint();
            let lo = lo.saturating_add(peek_len);
            let hi = match hi {
                Some(x) => x.checked_add(peek_len),
                None => None,
            };
            (lo, hi)
        }
    }
    impl<'a> From<Chars<'a>> for TokenIterator<'a> {
        fn from(s: Chars<'a>) -> TokenIterator<'a> {
            TokenIterator {
                iter: s,
                peeked: None,
            }
        }
    }
    impl<'a> TokenIterator<'a> {
        pub fn peek(&mut self) -> Option<&<TokenIterator<'a> as Iterator>::Item> {
            let iter = &mut self.iter;
            self.peeked.get_or_insert_with(|| iter.next()).as_ref()
        }
        pub fn peek_mut(&mut self) -> Option<&mut <TokenIterator<'a> as Iterator>::Item> {
            let iter = &mut self.iter;
            self.peeked.get_or_insert_with(|| iter.next()).as_mut()
        }
        pub fn next_if(
            &mut self,
            func: impl FnOnce(&<TokenIterator<'a> as Iterator>::Item) -> bool,
        ) -> Option<<TokenIterator<'a> as Iterator>::Item> {
            match self.next() {
                Some(matched) if func(&matched) => Some(matched),
                other => {
                    if !self.peeked.is_none() {
                        ::core::panicking::panic("assertion failed: self.peeked.is_none()")
                    };
                    self.peeked = Some(other);
                    None
                }
            }
        }
        pub fn next_if_eq<T>(
            &mut self,
            expected: &T,
        ) -> Option<<TokenIterator<'a> as Iterator>::Item>
        where
            T: ?Sized,
            <TokenIterator<'a> as Iterator>::Item: PartialEq<T>,
        {
            self.next_if(|next| next == expected)
        }
    }
    pub fn lex(input: &String) -> Result<Vec<Token>, LexError> {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &match (&*KEYWORD_TRIE,) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Debug::fmt,
                    )],
                },
            ));
        };
        let mut result = Vec::new();
        let mut it = TokenIterator::from(input.chars());
        let mut character = 0;
        let mut line = 0;
        while let Some(&c) = it.peek() {
            let token = match c {
                '0'..='9' => {
                    let start_pos = Position { line, character };
                    let value = get_number(&mut it)?;
                    character += value.1;
                    let end_pos = Position { line, character };
                    Token {
                        token_type: TokenKind::Literal(value.0),
                        range: (start_pos, end_pos),
                    }
                }
                '+' | '-' | ';' | ',' | '.' | '(' | ')' | '{' | '}' | '[' | ']' | '@' | '#'
                | '~' | '?' | ':' | '$' | '=' | '!' | '<' | '>' | '&' | '|' | '*' | '/' | '^'
                | '%' => {
                    let start_position = Position { line, character };
                    character += 1;
                    let end_position = Position { line, character };
                    Token {
                        token_type: match it.next() {
                            Some(c) => match c {
                                ';' => TokenKind::Semi,
                                ',' => TokenKind::Comma,
                                '.' => TokenKind::Dot,
                                '(' => TokenKind::OpenParen,
                                ')' => TokenKind::CloseParen,
                                '{' => TokenKind::OpenBrace,
                                '}' => TokenKind::CloseBrace,
                                '[' => TokenKind::OpenBracket,
                                ']' => TokenKind::CloseBracket,
                                '@' => TokenKind::At,
                                '#' => TokenKind::Pound,
                                '~' => TokenKind::Tilde,
                                '?' => TokenKind::Question,
                                ':' => TokenKind::Colon,
                                '$' => TokenKind::Dollar,
                                '=' => TokenKind::Eq,
                                '!' => TokenKind::Bang,
                                '<' => TokenKind::Lt,
                                '>' => TokenKind::Gt,
                                '-' => TokenKind::Minus,
                                '&' => TokenKind::And,
                                '|' => TokenKind::Or,
                                '+' => TokenKind::Plus,
                                '*' => TokenKind::Star,
                                '/' => TokenKind::Slash,
                                '^' => TokenKind::Caret,
                                '%' => TokenKind::Percent,
                                _ => continue,
                            },
                            None => break,
                        },
                        range: (start_position, end_position),
                    }
                }
                ' ' => {
                    it.next();
                    character += 1;
                    continue;
                }
                '\r' | '\n' => {
                    it.next();
                    line += 1;
                    character = 0;
                    continue;
                }
                'a'..='z' | 'A'..='Z' => {
                    let start_pos = Position { line, character };
                    it.next();
                    let ident = get_ident(c, &mut it);
                    character += ident.len();
                    let end_pos = Position { line, character };
                    Token {
                        token_type: TokenKind::Ident(ident),
                        range: (start_pos, end_pos),
                    }
                }
                _ => {
                    return Err(LexError::new(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Unexpected character "],
                            &match (&c,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    }));
                }
            };
            result.push(token);
        }
        Ok(result)
    }
    fn get_number(iter: &mut TokenIterator) -> Result<(Literal, usize), LexError> {
        let mut dec = false;
        let mut base = 10u8;
        let mut number = 0u64;
        let mut decimal = 0.0;
        let mut index = 0;
        let mut size = 0usize;
        let mut err = Ok(Literal::Empty);
        while let Some(Ok(digit)) = iter.peek().map(|c| match *c {
            '.' if !dec => {
                dec = true;
                Ok(0)
            }
            '#' => {
                base = number as u8;
                number = 0;
                Ok(0)
            }
            _ => match u64::from_str_radix(&c.to_string(), base as u32) {
                Ok(d) => Ok(d),
                Err(e) => match c {
                    '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                        err = Err(LexError::new(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["", " "],
                                &match (&e, &e.kind()) {
                                    _args => [
                                        ::core::fmt::ArgumentV1::new(
                                            _args.0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            _args.1,
                                            ::core::fmt::Debug::fmt,
                                        ),
                                    ],
                                },
                            ));
                            res
                        }));
                        Err(e)
                    }
                    _ => Err(e),
                },
            },
        }) {
            if dec {
                decimal = decimal + digit as f64 / (10i32.pow(index) as f64);
                index = index + 1;
            } else {
                number = number * base as u64 + digit;
            }
            size += 1;
            iter.next();
        }
        match err {
            Err(e) => return Err(e),
            _ => (),
        }
        if dec {
            if base != 10 {
                Err(LexError::new(&String::from(
                    "Unable to create floating point value with non decimal base!",
                )))
            } else {
                Ok((Literal::Float(number as f64 + decimal), size))
            }
        } else {
            Ok((Literal::Integer(number, base), size))
        }
    }
    fn get_ident(c: char, iter: &mut TokenIterator) -> String {
        let mut ident = c.to_string();
        while let Some(nc) = iter.peek() {
            match nc {
                'a'..='z' | 'A'..='Z' => {
                    ident.push(*nc);
                }
                _ => {
                    return ident;
                }
            }
            iter.next();
        }
        ident
    }
}
mod trie {
    use std::fmt::Debug;
    use colored::Colorize;
    pub struct TrieNode<T>
    where
        T: Debug,
    {
        children: [Option<Box<TrieNode<T>>>; 26],
        keyword: Option<T>,
    }
    impl<T> TrieNode<T>
    where
        T: Debug,
    {
        pub fn new() -> Self {
            TrieNode {
                children: Default::default(),
                keyword: None,
            }
        }
        fn num_children(&self) -> u8 {
            self.children.iter().fold(0, |acc, x| match x {
                Some(_) => acc + 1,
                None => acc,
            })
        }
        pub fn insert(&mut self, value: u8, keyword: Option<T>) -> &mut Box<TrieNode<T>> {
            let index = value as usize - 'a' as usize;
            let found = &mut self.children[index];
            match found {
                Some(b) => b.keyword = keyword,
                None => {
                    let b = Box::new(TrieNode {
                        children: Default::default(),
                        keyword,
                    });
                    self.children[index] = Some(b);
                }
            }
            match &mut self.children[index] {
                Some(b) => b,
                None => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["Shouldn\'t be here!"],
                    &match () {
                        _args => [],
                    },
                )),
            }
        }
        pub fn next(&self, c: char) -> Option<&Box<TrieNode<T>>> {
            match &self.children[c as usize - 'a' as usize] {
                Some(s) => Some(s),
                None => None,
            }
        }
        fn output(
            &self,
            f: &mut std::fmt::Formatter<'_>,
            index: u32,
            indent: &String,
            last: bool,
            value: i8,
        ) -> std::fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &match (&indent,) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ))?;
            if index != 0 {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&if last { "└──" } else { "├──" },) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))?;
            }
            if let Some(k) = &self.keyword {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &match (&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[""],
                            &match (&((value as u8 + 'a' as u8) as char),) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    }
                    .bright_green(),)
                    {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))?;
                k.fmt(f)?;
            } else if value != -1 {
                f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", "\n"],
                    &match (&((value as u8 + 'a' as u8) as char),) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))?;
            }
            let nindent = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (
                        &indent,
                        &if index == 0 {
                            ""
                        } else if last {
                            "    "
                        } else {
                            "│   "
                        },
                    ) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            };
            let mut num = 0;
            self.children.iter().enumerate().for_each(|(i, v)| match v {
                Some(v) => {
                    v.output(
                        f,
                        index + 1,
                        &nindent,
                        num == self.num_children() - 1,
                        i as i8,
                    )
                    .unwrap();
                    num += 1;
                }
                None => (),
            });
            Ok(())
        }
    }
    impl<T> Debug for TrieNode<T>
    where
        T: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.output(f, 0, &"".to_string(), false, -1)
        }
    }
}
use trielib::*;
fn main() {
    let tokens = lexer::lex(&String::from("5 + Hello")).unwrap();
}
