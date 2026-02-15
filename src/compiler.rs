use crate::error;
use crate::error::CompileError;
use crate::lexer;
use crate::mips;
use crate::parser;

pub struct Compiler {
    source: String,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        Compiler {
            source: source.to_string(),
        }
    }

    pub fn compile(&mut self, output_file: &str, emit_comments: bool) -> Result<(), error::CompileError> {
        let mut l = lexer::Lexer::new(&self.source);
        let tokens = l.tokenize()?;
        let mut p = parser::Parser::new(tokens);
        let program = p.parse()?;

        let mut mips_gen = mips::MipsGenerator::new(program, emit_comments);
        let mips_code = mips_gen.generate()?;

        let res = std::fs::write(output_file, mips_code.to_string());

        match res {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                return Err(CompileError::GenericError {
                    message: format!("Failed to write to output file: {}", e),
                });
            }
        }
    }

    pub fn get_ast(&mut self) -> Result<parser::ast::Program, error::CompileError> {
        let mut l = lexer::Lexer::new(&self.source);
        let tokens = l.tokenize()?;
        let mut p = parser::Parser::new(tokens);
        let program = p.parse()?;

        Ok(program)
    }

    pub fn get_tokens(&mut self) -> Result<Vec<lexer::Token>, error::CompileError> {
        let mut l = lexer::Lexer::new(&self.source);
        let tokens = l.tokenize()?;

        Ok(tokens)
    }
}
