use std::{ops::Range, rc::Rc};

use ts_errors::LexError;

use crate::{Position, Token, TokenKind, TokenRange};

pub struct Lexer<'a> {
    input: &'a str,

    errors: Vec<LexError>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input,
            errors: Vec::new(),
        }
    }

    pub fn lex(&self) -> Vec<Rc<Token>> {
        let mut tokens = Vec::new();

        let mut char_iter = self.input.chars().chain(['\0'].into_iter()).enumerate();

        let current_keyword: Range<usize> = 0..0;
        let mut current_range: Range<usize> = 0..0;
        // let file_position = Position::default();

        let mut file_range = TokenRange::default();

        while let Some((index, current_char)) = char_iter.next() {
            let current_str = &self.input[current_range.clone()];

            if current_str.len() > 0 {
                match current_char {
                    ' ' => (),
                    _ => match TokenKind::from(current_str) {
                        TokenKind::Unkown => {
                            let current_str =
                                &self.input[current_range.start..current_range.end - 1];

                            file_range.end.prev_char();

                            tokens.push(Rc::new(Token::new(
                                self.input,
                                current_range.clone(),
                                file_range,
                            )));
                            file_range.set_start(file_range.end);
                            current_range.start = current_range.end
                        }
                        _ => (),
                    },
                }
            } else {
                match current_char {
                    '\n' => file_range.end.next_line(),
                    _ => (),
                }
            }

            file_range.end.next_char();
            current_range.end += 1
        }

        tokens
    }
}
