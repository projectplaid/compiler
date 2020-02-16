use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

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
        Token {
            symbol: Symbol::Identifier,
            value: "foo".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = LexerInstance::new("tests/foo.st".to_string());
        assert!(result.is_ok());

        let mut instance = result.unwrap();

        instance.next();
    }
}
