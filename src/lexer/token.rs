#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Void,
    Int32,
    Bool,
    String,
    If,
    Else,
    While,
    For,
    Return,
    
    // Literals
    Integer(i32),
    StringLiteral(String),
    BoolLiteral(bool),
    
    // Identifier (this comment is somewhat redundant)
    Identifier(String),

    // Operators
    Equal,
    EqualEqual,
    Not,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Plus,
    Minus,
    Star,
    Ampersand,

    // Almost operators but not really
    PlusPlus,
    MinusMinus,
    RightShift,
    LeftShift,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    
    // Other
    Eof,
    //NewLine,
}