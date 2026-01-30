use std::fmt;

pub type Result<T> = std::result::Result<T, CompileError>;

#[derive(Debug)]
pub enum CompileError {
    LexError { message: String, line: usize },
    ParseError { message: String, line: usize },
    TypeError { message: String },
    UndefinedVariable { name: String },
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileError::LexError { message, line } => {
                write!(f, "Lexical error at line {}: {}", line, message)
            }
            CompileError::ParseError { message, line } => {
                write!(f, "Parse error at line {}: {}", line, message)
            }
            CompileError::TypeError { message } => {
                write!(f, "Type error: {}", message)
            }
            CompileError::UndefinedVariable { name } => {
                write!(f, "Undefined variable: {}", name)
            }
        }
    }
}

impl std::error::Error for CompileError {}
