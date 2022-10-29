use std::{fmt::Display, str::Chars};

use ts_macros::*;

// pub mod ast;
pub mod new_ast;
pub mod new_lexer;
use new_ast::Literal;

use ts_errors::LexError;

#[derive(KeywordFromSlice, Debug, Clone, Copy, PartialEq)]
pub enum KeywordKind {
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    As,
    Implements,
    Interface,
    Let,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Yield,
    Any,
    Boolean,
    Constructor,
    Declare,
    Get,
    Module,
    Require,
    Number,
    Set,
    String,
    Symbol,
    Type,
    From,
    Of,
    Await,
    Undefined,

    __Unknown,
}

// impl From<&str> for KeywordKind {
//     fn from(input: &str) -> Self {
//         match input {

//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Keyword {
//     pub keyword: KeywordKind,
//     pub range: TokenRange,
// }

impl KeywordKind {
    fn translate(&self) -> TokenKind {
        match self {
            KeywordKind::As => TokenKind::Operator(OperatorKind::As),
            KeywordKind::Typeof => TokenKind::Operator(OperatorKind::Typeof),
            KeywordKind::Void => TokenKind::Operator(OperatorKind::Void),
            KeywordKind::Delete => TokenKind::Operator(OperatorKind::Delete),
            KeywordKind::Await => TokenKind::Operator(OperatorKind::Await),
            KeywordKind::In => TokenKind::Operator(OperatorKind::In),
            KeywordKind::Yield => TokenKind::Operator(OperatorKind::Yield),
            _ => TokenKind::Keyword(*self),
        }
    }

    // pub fn create_expect(kind: KeywordKind) -> TokenKind {
    //     TokenKind::Keyword(Keyword {
    //         keyword: kind,
    //         range: TokenRange::default(),
    //     })
    // }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorKind {
    Equality,
    NotEqual,
    Question,
    Assignment,
    As,
    Arrow,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Mult,
    MultEqual,
    Divide,
    DivideEqual,
    Percent,
    PercentEqual,
    Dot,
    BitOr,
    BitOrEqual,
    BitAnd,
    BitAndEqual,
    BitXor,
    BitXorEqual,
    LogicalOr,
    LogicalOrEqual,
    LogicalAnd,
    LogicalAndEqual,
    LogicalXor,
    Gt,
    Lt,
    Eq,
    GtEq,
    LtEq,
    NGt,
    NLt,
    NEq,
    BitNot,
    BitNotEqual,
    LogicalNot,
    LogicalNotEqual,
    BitLeft,
    BitLeftEqual,
    BitRight,
    BitRightEqual,
    BitRightUnsigned,
    BitRightUnsignedEqual,
    Spread,

    OptionalChain,
    Increment,
    Decrement,
    Typeof,
    Void,
    Delete,
    Await,
    Exponent,
    In,
    InstanceOf,
    NullishCoalescing,
    NullishCoalescingEqual,
    Yield,

    Unkown,
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assignment => write!(f, "="),
            Self::BitAnd => write!(f, "&"),
            Self::BitNot => write!(f, "~"),
            Self::BitOr => write!(f, "i"),
            Self::BitXor => write!(f, "^"),
            Self::NullishCoalescing => write!(f, "??"),
            Self::Delete => write!(f, "delete"),
            Self::Divide => write!(f, "/"),
            Self::Eq => write!(f, "=="),
            Self::Exponent => write!(f, "**"),
            Self::Gt => write!(f, ">"),
            Self::GtEq => write!(f, ">="),
            Self::In => write!(f, "in"),
            Self::InstanceOf => write!(f, "instanceof"),
            Self::Await => write!(f, "Await"),
            Self::In => write!(f, "In"),
            Self::Typeof => write!(f, "Typeof"),
            Self::LogicalAnd => write!(f, "&&"),
            Self::LogicalNot => write!(f, "~"),
            Self::LogicalOr => write!(f, "||"),
            Self::Lt => write!(f, "<"),
            Self::LtEq => write!(f, "<="),
            Self::Minus => write!(f, "-"),
            Self::Percent => write!(f, "%"),
            Self::Mult => write!(f, "*"),
            Self::NEq => write!(f, "!="),
            Self::Plus => write!(f, "+"),
            Self::BitLeft => write!(f, "<<"),
            Self::BitRight => write!(f, ">>"),
            Self::BitRightUnsigned => write!(f, ">>>"),
            Self::Typeof => write!(f, "typeof"),
            Self::Void => write!(f, "void"),
            Self::OptionalChain => write!(f, "?."),
            Self::Increment => write!(f, "++"),
            Self::Decrement => write!(f, "--"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<&str> for OperatorKind {
    fn from(content: &str) -> Self {
        use OperatorKind::*;
        let mut input = content.chars();
        match input.nth(1).unwrap() {
            '=' => match input.nth(0).unwrap() {
                '=' => Equality,
                '+' => PlusEqual,
                '-' => MinusEqual,
                '*' => MultEqual,
                '/' => DivideEqual,
                '%' => PercentEqual,
                '|' => BitOrEqual,
                '&' => BitAndEqual,
                '^' => BitXorEqual,
                '>' => GtEq,
                '<' => LtEq,
                '~' => BitNotEqual,
                '!' => NotEqual,
                _ => Unkown,
            },

            '>' => match input.nth(0).unwrap() {
                '>' => match content.len() {
                    2 => BitRight,
                    3 => match input.nth(2).unwrap() {
                        '=' => BitRightEqual,
                        '>' => BitRightUnsigned,
                        _ => Unkown,
                    },
                    4 => match input.nth(2).unwrap() {
                        '>' => match input.nth(3).unwrap() {
                            '=' => BitRightUnsignedEqual,
                            _ => Unkown,
                        },
                        _ => Unkown,
                    },
                    _ => Unkown,
                },
                _ => Unkown,
            },
            '<' => match input.nth(0).unwrap() {
                '<' => match content.len() {
                    2 => BitLeft,
                    3 => match input.nth(2).unwrap() {
                        '=' => BitLeftEqual,
                        _ => Unkown,
                    },
                    _ => Unkown,
                },
                _ => Unkown,
            },
            '|' => match input.nth(0).unwrap() {
                '|' => match content.len() {
                    2 => LogicalOr,
                    3 => match input.nth(2).unwrap() {
                        '=' => LogicalOrEqual,
                        _ => Unkown,
                    },
                    _ => Unkown,
                },
                _ => Unkown,
            },
            '&' => match input.nth(0).unwrap() {
                '&' => match content.len() {
                    2 => LogicalAnd,
                    3 => match input.nth(2).unwrap() {
                        '=' => LogicalAndEqual,
                        _ => Unkown,
                    },
                    _ => Unkown,
                },
                _ => Unkown,
            },
            '?' => match input.nth(0).unwrap() {
                '?' => match content.len() {
                    2 => NullishCoalescing,
                    3 => match input.nth(2).unwrap() {
                        '=' => NullishCoalescingEqual,
                        _ => Unkown,
                    },
                    _ => Unkown,
                },
                _ => Unkown,
            },
            '.' => match input.nth(0).unwrap() {
                '?' => OptionalChain,
                '.' => match content.len() {
                    3 => match input.nth(2).unwrap() {
                        '.' => Spread,
                        _ => Unkown,
                    },
                    _ => Unkown,
                },
                _ => Unkown,
            },
            '+' => match input.nth(0).unwrap() {
                '+' => Increment,
                _ => Unkown,
            },
            '*' => match input.nth(0).unwrap() {
                '*' => Exponent,
                _ => Unkown,
            },
            '-' => match input.nth(0).unwrap() {
                '-' => Decrement,
                _ => Unkown,
            },

            _ => Unkown,
        }
    }
}

impl From<char> for OperatorKind {
    fn from(input: char) -> Self {
        use OperatorKind::*;
        match input {
            '=' => Assignment,
            '+' => Plus,
            '-' => Minus,
            '*' => Mult,
            '/' => Divide,
            '%' => Percent,
            '.' => Dot,
            '|' => BitOr,
            '&' => BitAnd,
            '^' => BitXor,
            '>' => Gt,
            '<' => Lt,
            '~' => BitNot,
            '!' => LogicalNot,
            '|' => BitOr,
            '?' => Question,
            _ => Unkown,
        }
    }
}

// impl Display for OperatorKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Operator {
//     pub operator: OperatorKind,
//     pub range: TokenRange,
// }

// impl Operator {
//     pub const fn create_expect(kind: OperatorKind) -> TokenKind {
//         TokenKind::Operator(Operator {
//             operator: kind,
//             range: TokenRange::default(),
//         })
//     }
// }

#[derive(Debug)]
pub enum TokenKind {
    Literal(Literal),

    Ident,
    Keyword(KeywordKind),
    Operator(OperatorKind),

    /// ";"
    Semi,
    /// ":"
    Colon,
    /// ","
    Comma,
    /// "."
    /* Dot, */
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

    Unkown,
    /*
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
    Percent, */
}

impl From<&str> for TokenKind {
    fn from(input: &str) -> Self {
        if input.len() == 1 {
            return TokenKind::from(input.chars().nth(0).unwrap());
        }
        match KeywordKind::from(input) {
            KeywordKind::__Unknown => (),
            k => return TokenKind::Keyword(k),
        }

        TokenKind::Unkown
    }
}

impl From<char> for TokenKind {
    fn from(input: char) -> Self {
        use TokenKind::*;
        match input {
            ';' => Semi,
            ':' => Colon,
            ',' => Comma,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '@' => At,
            '#' => Pound,
            _ => {
                match OperatorKind::from(input) {
                    OperatorKind::Unkown => (),
                    o => return Operator(o),
                };

                if input.is_alphabetic() || input == '_' {
                    return Ident;
                }

                if input.is_numeric() {
                    return Literal(new_ast::Literal::Number(input.to_digit(10).unwrap().into()));
                }

                TokenKind::Unkown
            }
        }
    }
}

// #[derive(Debug)]
pub struct Token<'a> {
    pub source: &'a str,
    pub token_type: TokenKind,
    pub file_range: TokenRange,
    pub source_range: std::ops::Range<usize>,
}

impl<'a> Token<'a> {
    pub fn new(
        source: &'a str,
        source_range: std::ops::Range<usize>,
        token_range: TokenRange,
    ) -> Token {
        Token {
            source,
            token_type: TokenKind::from(&source[source_range.clone()]),
            file_range: token_range,
            source_range,
        }
    }

    pub fn token_type(&self) -> &TokenKind {
        &self.token_type
    }

    pub fn as_str(&self) -> &str {
        &self.source[self.source_range.clone()]
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    line: usize,
    character: usize,
}

impl Position {
    pub fn next_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }

    pub fn next_char(&mut self) {
        self.character += 1;
    }

    pub fn prev_char(&mut self) {
        self.character -= 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: 0,
            character: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TokenRange {
    pub start: Position,
    pub end: Position,
}

impl TokenRange {
    pub fn new(start: Position, end: Position) -> TokenRange {
        TokenRange { start, end }
    }

    pub fn one_line(&self) -> bool {
        self.start.line == self.end.line
    }

    pub fn set(&mut self, pos: Position) {
        self.start = pos;
        self.end = pos;
    }

    pub fn set_start(&mut self, pos: Position) {
        self.start = pos;
    }

    pub fn set_end(&mut self, pos: Position) {
        self.end = pos;
    }
}

impl Default for TokenRange {
    fn default() -> Self {
        Self {
            start: Position::default(),
            end: Position::default(),
        }
    }
}

/*
#[derive(Debug)]
struct TokenIterator<'a> {
    iter: Chars<'a>,
    peeked: Option<Option<<Chars<'a> as Iterator>::Item>>,
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
}

pub fn lex(input: &String) -> Result<Vec<Token>, LexError> {
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

                // let value = match value.0 {
                //     Literal::Nu(v) => Literal::Integer(v),
                //     Literal::Float(v) => Literal::Float(v),
                //     v => v,
                // };
                Token {
                    token_type: TokenKind::Literal(value.0),
                    range: (start_pos, end_pos),
                }
            }
            '\'' | '"' => {
                let start_pos = Position { line, character };
                it.next();
                character += 1;
                let mut text = String::new();
                while let Some(c) = it.next() {
                    match c {
                        '\'' | '"' => break,
                        _ => text.insert(text.len(), c),
                    }
                }
                character += text.len() + 1;
                let end_pos = Position { line, character };

                Token {
                    token_type: TokenKind::Literal(Literal::String(text)),
                    range: (start_pos, end_pos),
                }
            }
            '+' | '-' | ';' | ',' | '.' | '(' | ')' | '{' | '}' | '[' | ']' | '@' | '#' | '~'
            | '?' | ':' | '$' | '=' | '!' | '<' | '>' | '&' | '|' | '*' | '/' | '^' | '%' => {
                let start_position = Position { line, character };
                character += 1;
                let end_position = Position { line, character };
                Token {
                    token_type: match it.next() {
                        Some(c) => match c {
                            ';' => TokenKind::Semi,
                            ':' => TokenKind::Colon,
                            ',' => TokenKind::Comma,
                            '(' => TokenKind::OpenParen,
                            ')' => TokenKind::CloseParen,
                            '{' => TokenKind::OpenBrace,
                            '}' => TokenKind::CloseBrace,
                            '[' => TokenKind::OpenBracket,
                            ']' => TokenKind::CloseBracket,
                            '@' => TokenKind::At,
                            '#' => TokenKind::Pound,
                            _ => TokenKind::Operator(Operator {
                                operator: match c {
                                    '=' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::Eq
                                        }
                                        Some('>') => {
                                            it.next();
                                            OperatorKind::Arrow
                                        }
                                        _ => {
                                            it.next();
                                            OperatorKind::Assignment
                                        }
                                    },
                                    '.' => match it.peek() {
                                        Some('.') => {
                                            it.next();
                                            OperatorKind::Spread
                                        }
                                        _ => OperatorKind::Dot,
                                    },
                                    '?' => match it.peek() {
                                        Some(c) => match c {
                                            '.' => {
                                                it.next();
                                                OperatorKind::OptionalChain
                                            }
                                            _ => OperatorKind::LogicalNot,
                                        },
                                        _ => OperatorKind::LogicalNot,
                                    },
                                    '!' => match it.peek() {
                                        Some(c) => match c {
                                            '<' => {
                                                it.next();
                                                OperatorKind::NLt
                                            }
                                            '>' => {
                                                it.next();
                                                OperatorKind::NGt
                                            }
                                            '=' => {
                                                it.next();
                                                OperatorKind::NEq
                                            }
                                            _ => OperatorKind::LogicalNot,
                                        },
                                        _ => OperatorKind::LogicalNot,
                                    },
                                    '<' => match it.peek() {
                                        Some(c) => match c {
                                            '<' => {
                                                it.next();
                                                match it.peek() {
                                                    Some('=') => OperatorKind::BitLeftEqual,
                                                    _ => OperatorKind::BitLeft,
                                                }
                                            }
                                            '=' => {
                                                it.next();
                                                OperatorKind::LtEq
                                            }
                                            _ => OperatorKind::Lt,
                                        },
                                        _ => OperatorKind::Lt,
                                    },
                                    '>' => match it.peek() {
                                        Some(c) => match c {
                                            '>' => {
                                                it.next();
                                                match it.peek() {
                                                    Some('=') => OperatorKind::BitRightEqual,
                                                    _ => OperatorKind::BitRight,
                                                }
                                            }
                                            '=' => {
                                                it.next();
                                                OperatorKind::GtEq
                                            }
                                            _ => OperatorKind::Gt,
                                        },
                                        _ => OperatorKind::Gt,
                                    },
                                    '-' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::MinusEqual
                                        }
                                        _ => OperatorKind::Minus,
                                    },
                                    '&' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::BitAndEqual
                                        }
                                        _ => OperatorKind::BitAnd,
                                    },
                                    '~' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::BitNotEqual
                                        }
                                        _ => OperatorKind::BitNot,
                                    },
                                    '|' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::BitOrEqual
                                        }
                                        _ => OperatorKind::BitOr,
                                    },
                                    '^' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::BitXorEqual
                                        }
                                        _ => OperatorKind::BitXor,
                                    },
                                    '+' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::PlusEqual
                                        }
                                        _ => OperatorKind::Plus,
                                    },
                                    '*' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::MultEqual
                                        }
                                        _ => OperatorKind::Mult,
                                    },
                                    '/' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::DivideEqual
                                        }
                                        _ => OperatorKind::Divide,
                                    },
                                    '%' => match it.peek() {
                                        Some('=') => {
                                            it.next();
                                            OperatorKind::PercentEqual
                                        }
                                        _ => OperatorKind::Percent,
                                    },
                                    _ => continue,
                                },
                                range: (start_position, end_position),
                            }),
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
                character += ident.1;

                let end_pos = Position { line, character };
                let ident = match ident.0 {
                    TokenKind::Keyword(Keyword { keyword, .. }) => TokenKind::Keyword(Keyword {
                        keyword,
                        range: (start_pos, end_pos),
                    }),
                    t => t,
                };

                Token {
                    token_type: ident,
                    range: (start_pos, end_pos),
                }
            }
            _ => {
                return Err(LexError::new(format!("Unexpected character {}", c)));
            }
        };
        result.push(token);
    }
    Ok(result)
}

fn get_number(iter: &mut TokenIterator) -> Result<(Literal, usize), LexError> {
    let mut dec = false;
    let mut just_dec = false;

    let mut base = 10u8;
    let mut number = 0u64;
    let mut decimal = 0.0;
    let mut index = 0;
    let mut size = 0usize;

    let mut err = Ok(Literal::Null);
    while let Some(Ok(digit)) = iter.peek().map(|c| match *c {
        '.' if !dec => {
            dec = true;
            just_dec = true;
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
                    err = Err(LexError::new(format!("{} {:?}", e, e.kind())));
                    Err(e)
                }
                _ => Err(e),
            },
        },
    }) {
        if let Some(p) = iter.peek() {
            if p.to_digit(base.try_into().unwrap()).is_none() {
                break;
            }
        }
        if index == 0 && just_dec {
            just_dec = false;
            continue;
        }
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

    if dec && !just_dec {
        if base != 10 {
            Err(LexError::new(String::from(
                "Unable to create floating point value with non decimal base!",
            )))
        } else {
            Ok((Literal::Number(number as f64 + decimal), size))
        }
    } else {
        Ok((Literal::Number(number as f64), size))
    }
}

fn get_ident(c: char, iter: &mut TokenIterator) -> (TokenKind, usize) {
    let mut ident = c.to_string();
    let mut keyword = if c.is_lowercase() {
        (*KEYWORD_TRIE).next(c)
    } else {
        None
    };
    while let Some(nc) = iter.peek() {
        match nc {
            'a'..='z' | '0'..='9' => {
                ident.push(*nc);
                match keyword {
                    Some(trie) => {
                        keyword = trie.next(*nc);
                    }
                    _ => (),
                }
            }
            'A'..='Z' | '_' => {
                ident.push(*nc);
                keyword = None
            }
            _ => {
                if let Some(k) = keyword {
                    if let Some(k) = k.keyword() {
                        return (
                            Keyword {
                                keyword: k,
                                range: default_range(),
                            }
                            .translate(),
                            ident.len(),
                        );
                    }
                }
                let size = ident.len();
                return (TokenKind::Ident(ident), size);
            }
        }
        iter.next();
    }
    if let Some(k) = keyword {
        if let Some(k) = k.keyword() {
            return (
                Keyword {
                    keyword: k,
                    range: default_range(),
                }
                .translate(),
                ident.len(),
            );
        }
    }
    let size = ident.len();
    (TokenKind::Ident(ident), size)
}
*/
