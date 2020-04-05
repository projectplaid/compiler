#[allow(unused,dead_code,unused_imports)]

use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(PartialEq, Debug)]
pub enum Symbol {
    Identifier,
    Keyword,
    Number,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Period,
    Semicolon,
    Hash,
    LT,
    GT,
    StringLiteral,
    Comment,
    LBrace,
    RBrace,
    DollarSign,
    Bang,
    EndOfFile,
}

pub struct Token {
    pub symbol: Symbol,
    pub value: String,
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
}

pub struct LexerInstance {
    reader_iter: Peekable<IntoIter<char>>,
    column: u64,
    line: u64,
}

fn generate_token(symbol: Symbol, value: String) -> Token {
    Token {
        symbol,
        value,
    }
}

impl LexerInstance {
    pub fn new(filename: String) -> io::Result<LexerInstance> {
        let s = fs::read_to_string(filename)?;

        Ok(LexerInstance {
            reader_iter: s.chars().collect::<Vec<_>>().into_iter().peekable(),
            column: 1,
            line: 1,
        })
    }

    fn get_char(&mut self) -> Option<char> {
        if let Some(c) = self.reader_iter.next() {
            match c {
                '\n' => {
                    self.column = 1;
                    self.line += 1;
                    return Some(c);
                }
                '\r' => {
                    return Some(c);
                }
                _ => {
                    self.column += 1;
                    return Some(c);
                }
            }
        }

        None
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.reader_iter.peek() {
            match c {
                '\t' | '\r' | '\n' | ' ' => {
                    let _ = self.get_char();
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn handle_number(&mut self) -> Result<Token, LexerError> {
        Ok(generate_token(Symbol::EndOfFile, "".to_string()))
    }

    fn handle_string(&mut self) -> Result<Token, LexerError> {
        let _ = self
            .get_char()
            .expect("first character should be available");
        let mut value = String::new();
        // do NOT push the first ' into the result

        while let Some(&c) = self.reader_iter.peek() {
            match c {
                '\'' => {
                    // this is either the end of the string or an escaped '
                    let _ = self.get_char();

                    if let Some(&ch) = self.reader_iter.peek() {
                        match ch {
                            '\'' => {
                                value.push('\'');
                                let _ = self.get_char();
                            }
                            _ => {
                                return Ok(generate_token(Symbol::StringLiteral, value));
                            }
                        }
                    } else {
                        return Ok(generate_token(Symbol::StringLiteral, value));
                    }
                }
                _ => {
                    let _ = self.get_char();
                    if c != '\r' {
                        // strip any \r, we're all \n internally
                        value.push(c);
                    }
                }
            }
        }

        Err(LexerError {
            message: format!("unterminated string constant: {}", value),
        })
    }

    fn handle_alpha(&mut self) -> Result<Token, LexerError> {
        let first_ch = self
            .get_char()
            .expect("first character should be available");
        let mut value = String::new();
        value.push(first_ch);

        while let Some(&c) = self.reader_iter.peek() {
            match c {
                '\t' | '\r' | '\n' | ' ' => {
                    return Ok(generate_token(Symbol::Identifier, value));
                }
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
                    let _ = self.get_char();
                    value.push(c);
                }
                ':' => {
                    let _ = self.get_char();
                    return Ok(generate_token(Symbol::Keyword, value));
                }
                _ => {
                    return Err(LexerError {
                        message: format!("unexpected char {}", c),
                    });
                }
            }
        }

        Ok(generate_token(Symbol::EndOfFile, value))
    }

    fn handle_comment(&mut self) -> Result<Token, LexerError> {
        Ok(generate_token(Symbol::EndOfFile, "".to_string()))
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        if let Some(&c) = self.reader_iter.peek() {
            match c {
                '0'..='9' => self.handle_number(),
                'A'..='Z' | 'a'..='z' => self.handle_alpha(),
                '.' => {
                    let _ = self.get_char();
                    Ok(generate_token(Symbol::Period, ".".to_string()))
                }
                '\'' => self.handle_string(),
                '"' => self.handle_comment(),
                _ => {
                    let _ = self.get_char();
                    Err(LexerError {
                        message: format!("unexpected character {}", c),
                    })
                }
            }
        } else {
            Ok(generate_token(Symbol::EndOfFile, "".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source_file() {
        let result = LexerInstance::new("tests/empty.st".to_string());
        assert!(result.is_ok());

        let mut instance = result.unwrap();

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::EndOfFile, token.symbol);
    }

    #[test]
    fn test_identifier() {
        let result = LexerInstance::new("tests/identifier.st".to_string());
        assert!(result.is_ok());

        let mut instance = result.unwrap();

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::EndOfFile, token.symbol);
        assert_eq!("Foobar", token.value);
    }

    #[test]
    fn test_strings() {
        let result = LexerInstance::new("tests/strings.st".to_string());
        assert!(result.is_ok());

        let mut instance = result.unwrap();

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::StringLiteral, token.symbol);
        assert_eq!("This is a string", token.value);

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::Period, token.symbol);

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::StringLiteral, token.symbol);
        assert_eq!("This is a string 'inside a string'", token.value);

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::Period, token.symbol);

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::StringLiteral, token.symbol);
        assert_eq!("This is a\nmultiline\nstring", token.value);

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::Period, token.symbol);

        let token = instance.next_token().expect("unable to get token");
        assert_eq!(Symbol::EndOfFile, token.symbol);
    }
}
