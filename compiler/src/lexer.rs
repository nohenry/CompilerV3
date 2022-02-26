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
    Match,    // match
    When,     // when
    In,       // in
    If,       // if
    Elif,     // elif
    Else,     // else
    Loop,     // loop
    Return,   // return
    Int,      // int
    Uint,     // uint
    Float,    // float
    Char,     // char
    Bool,     // bool
    Template, // template
    Spec,     // spec
    True,     // true
    False,    // false
    Import,   // import
    Let,      // let
    Persist,  // persist
    Or,       // or
    And,      // and
    Xor,      // xor
    Null,     // null
    Module,   // module
    Function, // function
    Type,     // type
    Export,   // export
    Yield,    // yield
    As,       // as
    Const,    // const
    Action,   // action
    Enum,     // enum
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keyword {
    keyword: KeywordKind,
    range: Range,
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorKind {
    Assignment,
    As,
    Plus,
    Minus,
    Mult,
    Divide,
    Percent,
    Dot,
    BitOr,
    BitAnd,
    BitXor,
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
    LogicalNot,
    BitLeft,
    BitRight,
    Spread,
    Arrow,
    Wildcard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operator {
    operator: OperatorKind,
    range: Range,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenKind,
    range: Range,
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

fn default_range() -> Range {
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
    println!("{:?}", *KEYWORD_TRIE);
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
            '\'' => {
                let start_pos = Position { line, character };
                it.next();
                character += 1;
                let mut text = String::new();
                while let Some(c) = it.next() {
                    match c {
                        '\'' => break,
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
                                    '~' => OperatorKind::BitNot,
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
                                        Some('*') => {
                                            it.next();
                                            OperatorKind::Wildcard
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
                                                OperatorKind::BitLeft
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
                                                OperatorKind::BitRight
                                            }
                                            '=' => {
                                                it.next();
                                                OperatorKind::GtEq
                                            }
                                            _ => OperatorKind::Gt,
                                        },
                                        _ => OperatorKind::Gt,
                                    },
                                    '-' => OperatorKind::Minus,
                                    '&' => OperatorKind::BitAnd,
                                    '|' => OperatorKind::BitOr,
                                    '+' => OperatorKind::Plus,
                                    '*' => OperatorKind::Mult,
                                    '/' => OperatorKind::Divide,
                                    '^' => OperatorKind::BitXor,
                                    '%' => OperatorKind::Percent,
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
            'a'..='z' => {
                ident.push(*nc);
                match keyword {
                    Some(trie) => {
                        println!("Here");
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
