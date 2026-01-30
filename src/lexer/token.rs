#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Void,
    Int32,
    If,
    Else,
    While,
    Return,
    
    // Literals
    Integer(i32),
    
    // Identifier (this comment is somewhat redundant)
    Identifier(String),

    // Operators
    Equal,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    
    // Other
    Eof,
}