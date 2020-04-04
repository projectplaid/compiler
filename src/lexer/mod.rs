use std::fs;
use std::io;
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

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
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
        while let Some(&c) = self.reader_iter.peek() {
            match c {
                '\t' | '\r' | '\n' | ' ' => {
                    let _ = self.reader_iter.next();
                }
                _ => {
                    return;
                }
            }
        }
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let mut value = String::new();

        while let Some(&c) = self.reader_iter.peek() {
            match c {
                '0'..='9' => {
                    // parse number
                }
                'A'..='Z' | 'a'..='z' => {
                    // parse string
                }
                '.' => {
                    return Ok(generate_token(Symbol::Period, value));
                }
                _ => {
                    return Err(LexerError {
                        message: format!("unexpected character {}", c),
                    });
                }
            }
        }

        Ok(generate_token(Symbol::EndOfFile, value))
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

        let token = instance.next().expect("unable to get token");
        assert!(token.symbol == Symbol::EndOfFile);
    }

    #[test]
    fn test_identifier() {
        let result = LexerInstance::new("tests/identifier.st".to_string());
        assert!(result.is_ok());

        let mut instance = result.unwrap();

        let token = instance.next().expect("unable to get token");
        assert!(token.symbol == Symbol::EndOfFile);
        println!("{}", token.value);
        assert!(token.value == "Foobar");
    }
}
