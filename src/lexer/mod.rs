use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

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
    reader: BufReader<std::fs::File>,
}

fn generate_token(symbol: Symbol, value: String) -> Token {
    return Token {
        symbol: symbol,
        value: value,
    };
}

impl LexerInstance {
    pub fn new(filename: String) -> io::Result<LexerInstance> {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);

        Ok(LexerInstance { reader: reader })
    }

    fn skip_whitespace(&mut self) {
        let mut buffer = [0; 1];
        loop {
            let cur_pos = self.reader.seek(SeekFrom::Current(0)).unwrap();
            let result = self.reader.read(&mut buffer);
            let _bytes_read: usize = 0;
            match result {
                Ok(_bytes_read) => match buffer[0] {
                    9 | 10 | 13 | 32 => (),
                    _ => {
                        let _ = self.reader.seek(SeekFrom::Start(cur_pos)).unwrap();
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let mut value = String::new();

        loop {
            let mut buffer = [0; 1];
            let result = self.reader.read(&mut buffer).unwrap();
            if result == 0 {
                return generate_token(Symbol::EndOfFile, value);
            }

            value.push(buffer[0] as char);
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
}
