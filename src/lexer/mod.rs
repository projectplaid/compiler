use std::fs;
use std::io;
use std::io::prelude::*;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(PartialEq)]
pub enum Symbol {
    Identifier,
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

pub struct LexerInstance {
    reader_iter: Peekable<IntoIter<char>>,
}

fn generate_token(symbol: Symbol, value: String) -> Token {
    return Token {
        symbol: symbol,
        value: value,
    };
}

impl LexerInstance {
    pub fn new(filename: String) -> io::Result<LexerInstance> {
        let s = fs::read_to_string(filename)?;

        Ok(LexerInstance {
            reader_iter: s.chars().collect::<Vec<_>>().into_iter().peekable(),
        })
    }

    fn skip_whitespace(&mut self) {
        loop {
            let result = self.reader_iter.peek();
            match result {
                Some(ch) => match ch {
                    '\t' | '\r' | '\n' | ' ' => {
                        let _ = self.reader_iter.next();
                    }
                    _ => {
                        return;
                    }
                },
                None => {
                    return;
                }
            }
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let mut value = String::new();
        let mut loop_count = 0;

        loop {
            let ch = self.reader_iter.peek();
            match ch {
                None => {
                    return generate_token(Symbol::EndOfFile, value);
                }
                Some(ch) => {
                    match ch {
                        '.' => {
                            return generate_token(Symbol::Period, value);
                        }
                        _ => {
                            value.push(*ch);
                        }
                    }
                    loop_count = loop_count + 1;
                }
            }
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

        let token = instance.next();
        assert!(token.symbol == Symbol::EndOfFile);
    }

    #[test]
    fn test_identifier() {
        let result = LexerInstance::new("tests/identifier.st".to_string());
        assert!(result.is_ok());

        let mut instance = result.unwrap();

        let token = instance.next();
        assert!(token.symbol == Symbol::EndOfFile);
        println!("{}", token.value);
        assert!(token.value == "Foobar");
    }
}
