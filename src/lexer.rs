mod token;
pub use token::Token;
use crate::error::{CompileError, Result};

pub struct Lexer {
    source: Vec<char>,
    line: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            line: 1,
            current: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            tokens.push(self.next_token()?);
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        let ch = self.peek();

        match ch {
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            '{' => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(Token::RightBrace)
            }
            ';' => {
                self.advance();
                Ok(Token::Semicolon)
            }
            '=' => {
                self.advance();
                Ok(Token::Equal)
            }
            '0'..='9' => self.scan_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),
            
            // Unknown character
            _ => Err(CompileError::LexError {
                message: format!("Unexpected character '{}'", ch),
                line: self.line,
            }),
        }
    }

    fn scan_identifier(&mut self) -> Result<Token> {
        let start = self.current;
        self.advance();

        while !self.is_at_end() {
            let ch = self.peek();
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text: String = self.source[start..self.current].iter().collect();

        let token = match text.as_str() {
            "void" => Token::Void,
            "int16" => Token::Int16,
            "int32" => Token::Int32,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "return" => Token::Return,
            _ => Token::Identifier(text),
        };

        Ok(token)
    }

    fn scan_number(&mut self) -> Result<Token> {
        let start = self.current;

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        let text: String = self.source[start..self.current].iter().collect();

        let value = text.parse::<i32>().map_err(|_| CompileError::LexError {
            message: format!("Invalid number: {}", text),
            line: self.line,
        })?;

        Ok(Token::Integer(value))
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.peek();
        self.current += 1;
        ch
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}