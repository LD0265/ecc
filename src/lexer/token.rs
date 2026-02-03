#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Void,
    Int32,
    String,
    If,
    Else,
    While,
    Return,
    
    // Literals
    Integer(i32),
    StringLiteral(String),
    
    // Identifier (this comment is somewhat redundant)
    Identifier(String),

    // Operators
    Equal,
    EqualEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    
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