use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(PartialEq)]
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

    fn handle_number(&mut self) -> Result<Token, LexerError> {
        Ok(generate_token(Symbol::EndOfFile, "".to_string()))
    }

    fn handle_string(&mut self) -> Result<Token, LexerError> {
        Ok(generate_token(Symbol::EndOfFile, "".to_string()))
    }

    fn handle_alpha(&mut self) -> Result<Token, LexerError> {
        let first_ch = self
            .reader_iter
            .next()
            .expect("first character should be available");
        let mut value = String::new();
        value.push(first_ch);

        while let Some(&c) = self.reader_iter.peek() {
            match c {
                '\t' | '\r' | '\n' | ' ' => {
                    return Ok(generate_token(Symbol::Identifier, value));
                }
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
                    let _ = self.reader_iter.next();
                    value.push(c);
                }
                ':' => {
                    let _ = self.reader_iter.next();
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

    pub fn next(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        if let Some(&c) = self.reader_iter.peek() {
            match c {
                '0'..='9' => return self.handle_number(),
                'A'..='Z' | 'a'..='z' => return self.handle_alpha(),
                '.' => return Ok(generate_token(Symbol::Period, ".".to_string())),
                '\'' => return self.handle_string(),
                '"' => return self.handle_comment(),
                _ => {
                    return Err(LexerError {
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
