use crate::error;
use crate::lexer;
use crate::parser;
use crate::mips;

pub struct Compiler {
    source: String,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        Compiler {
            source: source.to_string(),
        }
    }

    pub fn compile(&mut self) -> Result<String, error::CompileError> {
        let mut l = lexer::Lexer::new(&self.source);
        let tokens = l.tokenize()?;
        let mut p = parser::Parser::new(tokens);
        let program = p.parse()?;

        let mut mips_gen = mips::MipsGenerator::new(program);
        let mips_code = mips_gen.generate()?;

        Ok(mips_code.to_string())
    }

    pub fn get_ast(&mut self) -> Result<parser::ast::Program, error::CompileError> {
        let mut l = lexer::Lexer::new(&self.source);
        let tokens = l.tokenize()?;
        let mut p = parser::Parser::new(tokens);
        let program = p.parse()?;

        Ok(program)
    }
}