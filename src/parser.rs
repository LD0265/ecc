pub(crate) mod ast;

use crate::error::{CompileError, Result};
use crate::lexer::Token;
use crate::parser::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program { statements })
    }
    
    fn parse_function(&mut self) -> Result<Statement> {
        let return_type = self.parse_type()?;
        
        let name = self.parse_identifier()?;
        
        self.expect(Token::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(Token::RightParen)?;
        
        self.expect(Token::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::RightBrace)?;
        
        Ok(Statement::Function {
            name,
            params,
            return_type,
            body,
        })
    }
    
    fn parse_type(&mut self) -> Result<Type> {
        let typ = match self.peek() {
            Token::Void => Type::Void,
            Token::Int32 => Type::Int32,
            _ => {
                return Err(CompileError::ParseError {
                    message: format!("Expected type, found {:?}", self.peek()),
                    line: 0,
                })
            }
        };
        self.advance();
        Ok(typ)
    }
    
    fn parse_identifier(&mut self) -> Result<String> {
        match self.peek() {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(CompileError::ParseError {
                message: format!("Expected identifier, found {:?}", self.peek()),
                line: 0,
            }),
        }
    }
    
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        let mut params = Vec::new();
        
        if matches!(self.peek(), Token::RightParen) {
            return Ok(params);
        }
        
        loop {
            let typ = self.parse_type()?;
            let name = self.parse_identifier()?;
            params.push(Parameter { name, param_type: typ });
            
            if !matches!(self.peek(), Token::Comma) {
                break;
            }
            self.advance();
        }
        
        Ok(params)
    }
    
    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        while !matches!(self.peek(), Token::RightBrace | Token::Eof) {
            statements.push(self.parse_statement()?);
        }
        
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Result<Statement> {
        match self.peek() {
            // We don't need these yet, but I put them here anyway
            // Token::Return => self.parse_return(),
            // Token::If => self.parse_if(),
            // Token::While => self.parse_while(),

            Token::Int32 | Token::Void => {
                let is_function = match self.peek_ahead(2) {
                    Some(Token::LeftParen) => true,
                    _ => false,
                };

                if is_function {
                    self.parse_function()
                } else {
                    self.parse_variable_declaration()
                }
            }
            _ => {
                self.parse_expression_statement()
            }
        }
    }
    
    fn parse_variable_declaration(&mut self) -> Result<Statement> {
        let var_type = self.parse_type()?;
        let identifier = self.parse_identifier()?;
        
        let init = if matches!(self.peek(), Token::Equal) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::VariableDeclaration {
            var_type,
            identifier,
            init,
        })
    }
    
    fn parse_expression(&mut self) -> Result<Expr> {
        match self.peek() {
            Token::Integer(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::Integer(n))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            _ => Err(CompileError::ParseError {
                message: format!("Expected expression, found {:?}", self.peek()),
                line: 0,
            }),
        }
    }
    
    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        Ok(Statement::ExprStatement(expr))
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn advance(&mut self) {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }
    
    fn expect(&mut self, expected: Token) -> Result<()> {
        if self.peek() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(CompileError::ParseError {
                message: format!("Expected {:?}, found {:?}", expected, self.peek()),
                line: 0,
            })
        }
    }
    
    fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current + n)
    }
}