use std::fmt;

pub type Result<T> = std::result::Result<T, CompileError>;

#[derive(Debug)]
pub enum CompileError {
    LexError { message: String, line: usize },
    ParseError { message: String, line: usize },
    CodeGenError { message: String, line: usize },
    TypeError { message: String, line: usize },
    UndefinedVariableError {message: String, line: usize},
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

            CompileError::TypeError { message, line } => {
                write!(f, "Type error at line {}: {}", line, message)
            }

            CompileError::CodeGenError { message, line } => {
                write!(f, "Code generation error at line {}: {}", line, message)
            }

            CompileError::UndefinedVariableError { message, line } => {
                write!(f, "Undefined Variable Error at line {}: {}", line, message)
            }
        }
    }
}

impl std::error::Error for CompileError {}
