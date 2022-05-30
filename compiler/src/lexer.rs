use std::{collections::hash_map::RandomState, fmt, iter::Peekable, str::Chars};
use trielib::*;

use crate::ast::Literal;

#[derive(Debug, Clone)]
pub struct LexError {
    error: String,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl LexError {
    fn new(error: &String) -> Self {
        LexError {
            error: error.clone(),
        }
    }
}

#[make_keywords]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordKind {
    As,       // as
    Spec,     //spec
    Switch,   // switch
    If,       // if
    Else,     // else
    Loop,     // loop
    Let,      // let
    Return,   // return
    Int,      // int
    Int8,     // int
    Int16,    // int
    Int32,    // int
    Int64,    // int
    Int128,   // int
    Uint,     // uint
    Uint8,    // uint
    Uint16,   // uint
    Uint32,   // uint
    Uint64,   // uint
    Uint128,  // uint
    Float,    // float
    Float32,  // float
    Float64,  // float
    Char,     // char
    Bool,     // bool
    Template, // template
    Action,   // action
    True,     // true
    False,    // false
    Import,   // import
    Persist,  // persist
    Or,       // or
    And,      // and
    Xor,      // xor
    Null,     // null
    Type,     // type
    Export,   // export
    Const,    // const
    Enum,     // enum
    Yield,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keyword {
    pub keyword: KeywordKind,
    pub range: Range,
}

impl Keyword {
    fn translate(&self) -> TokenKind {
        match self.keyword {
            KeywordKind::As => TokenKind::Operator(Operator {
                operator: OperatorKind::As,
                range: self.range,
            }),
            KeywordKind::Or => TokenKind::Operator(Operator {
                operator: OperatorKind::LogicalOr,
                range: self.range,
            }),
            KeywordKind::And => TokenKind::Operator(Operator {
                operator: OperatorKind::LogicalAnd,
                range: self.range,
            }),
            KeywordKind::Xor => TokenKind::Operator(Operator {
                operator: OperatorKind::LogicalXor,
                range: self.range,
            }),
            _ => TokenKind::Keyword(*self),
        }
    }

    pub fn create_expect(kind: KeywordKind) -> TokenKind {
        TokenKind::Keyword(Keyword {
            keyword: kind,
            range: default_range(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorKind {
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
    LogicalAnd,
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
    BitLeft,
    BitLeftEqual,
    BitRight,
    BitRightEqual,
    Spread,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operator {
    pub operator: OperatorKind,
    pub range: Range,
}

impl Operator {
    pub const fn create_expect(kind: OperatorKind) -> TokenKind {
        TokenKind::Operator(Operator {
            operator: kind,
            range: default_range(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Literal(Literal),

    Ident(String),
    Keyword(Keyword),
    Operator(Operator),

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

#[macro_export]
macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
    ($target: expr, $pat: path, 2) => {{
        if let $pat(a, b) = $target {
            // #1
            (a, b)
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
    ($target: expr, $pat: path, 3) => {{
        if let $pat(a, b, c) = $target {
            // #1
            (a, b, c)
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenKind,
    pub range: Range,
}

impl Token {
    pub fn token_type(&self) -> TokenKind {
        self.token_type.clone()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    line: usize,
    character: usize,
}

pub type Range = (Position, Position);

pub const fn default_range() -> Range {
    (
        Position {
            character: 0,
            line: 0,
        },
        Position {
            character: 0,
            line: 0,
        },
    )
}

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
                // Since we called `self.next()`, we consumed `self.peeked`.
                assert!(self.peeked.is_none());
                self.peeked = Some(other);
                None
            }
        }
    }

    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<<TokenIterator<'a> as Iterator>::Item>
    where
        T: ?Sized,
        <TokenIterator<'a> as Iterator>::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
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

                let value = match value.0 {
                    Literal::Integer(v, b, _) => Literal::Integer(v, b, (start_pos, end_pos)),
                    Literal::Float(v, _) => Literal::Float(v, (start_pos, end_pos)),
                    v => v,
                };
                Token {
                    token_type: TokenKind::Literal(value),
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
                    token_type: TokenKind::Literal(Literal::String(text, (start_pos, end_pos))),
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
                return Err(LexError::new(&format!("Unexpected character {}", c)));
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

    let mut err = Ok(Literal::Empty);
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
                    err = Err(LexError::new(&format!("{} {:?}", e, e.kind())));
                    Err(e)
                }
                _ => Err(e),
            },
        },
    }) {
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
            Err(LexError::new(&String::from(
                "Unable to create floating point value with non decimal base!",
            )))
        } else {
            Ok((
                Literal::Float(number as f64 + decimal, default_range()),
                size,
            ))
        }
    } else {
        Ok((Literal::Integer(number, base, default_range()), size))
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
