#[derive(Debug)]
pub enum Token {
    // Keywords
    Void,
    Int16,
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
    Semicolon,
    
    // Other
    Eof,
}