pub(crate) mod ast;

use crate::error::{CompileError, Result};
use crate::lexer::Token;
use crate::parser::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,

    num_while: usize,
    num_for: usize,
    num_if: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            num_while: 0,
            num_for: 0,
            num_if: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let data_body: Vec<Statement>;
        let mut text_body: Vec<Statement> = Vec::new();

        text_body.push(Statement::Function {
            name: "_start".to_string(),
            params: Vec::new(),
            return_type: Type::Void,
            body: {
                vec![
                    Statement::Instruction {
                        opcode: "jal".to_string(),
                        operands: vec!["main".to_string()],
                    },
                    Statement::Instruction {
                        opcode: "li".to_string(),
                        operands: vec!["$v0, 10".to_string()],
                    },
                    Statement::Instruction {
                        opcode: "syscall".to_string(),
                        operands: vec![],
                    },
                ]
            },
            use_stack: false,
        });

        while !self.is_at_end() {
            text_body.push(self.parse_statement()?);
        }

        data_body = self.populate_data_segment(&text_body);

        Ok(Program {
            segments: Segments {
                data: Segment { body: data_body },
                text: Segment { body: text_body },
            },
        })
    }

    fn parse_function(&mut self) -> Result<Statement> {
        let return_type = self.parse_type()?;

        let name = self.parse_identifier()?;

        self.expect(Token::LeftParen, "parse_function")?;
        let params = self.parse_parameters()?;
        self.expect(Token::RightParen, "parse_function")?;

        self.expect(Token::LeftBrace, "parse_function")?;
        let mut body = self.parse_block()?;

        if return_type == Type::Void {
            body.push(Statement::Return { value: Expr::Empty })
        }

        self.expect(Token::RightBrace, "parse_function")?;

        Ok(Statement::Function {
            name,
            params,
            return_type,
            body,
            use_stack: true,
        })
    }

    fn parse_type(&mut self) -> Result<Type> {
        let typ = match self.peek() {
            Token::Void => Type::Void,
            Token::Int32 => Type::Int32,
            Token::Bool => Type::Bool,
            Token::String => Type::String,

            _ => {
                return Err(CompileError::ParseError {
                    message: format!("Expected type, found {:?}", self.peek()),
                    line: 0,
                });
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
            params.push(Parameter {
                name,
                param_type: typ,
            });

            if !matches!(self.peek(), Token::Comma) {
                break;
            }
            self.advance();
        }

        Ok(params)
    }

    fn parse_return(&mut self) -> Result<Statement> {
        self.advance();
        let e = self.parse_expression()?;
        self.expect(Token::Semicolon, "parse_return")?;
        Ok(Statement::Return { value: e })
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
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),

            Token::IPrint | Token::SPrint => self.parse_builtin_function(),

            Token::Int32 | Token::String | Token::Bool | Token::Void => {
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

            Token::Identifier(name) => {
                let is_function = match self.peek_ahead(1) {
                    Some(Token::LeftParen) => true,
                    _ => false,
                };

                if is_function {
                    self.parse_function_call(name.clone())
                } else {
                    self.parse_expression_statement()
                }
            }

            _ => self.parse_expression_statement(),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement> {
        let var_type = self.parse_type()?;
        let identifier = self.parse_identifier()?;

        let res_statement: Statement;

        let mut should_expect_semicolon = true;

        if matches!(self.peek(), Token::Equal) {
            self.advance();

            let mut is_not = false;
            if matches!(self.peek(), Token::Not) {
                is_not = true;
                self.advance();
            }

            let left = self.parse_expression()?;

            // This is so terrible you have no idea
            if let Expr::FunctionCall { .. } = left {
                should_expect_semicolon = false;
            }

            let operator = match self.peek() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => BinaryOperator::Empty,
            };

            if operator != BinaryOperator::Empty {
                self.advance();
            }

            let mut right = Expr::Empty;
            if operator != BinaryOperator::Empty {
                right = self.parse_expression()?;
            }

            res_statement = Statement::VariableDeclaration {
                var_type,
                identifier,
                operation: Expr::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                    is_not,
                },
            }
        } else {
            res_statement = Statement::VariableDeclaration {
                var_type,
                identifier,
                operation: Expr::BinaryOp {
                    left: Box::new(Expr::Empty),
                    operator: BinaryOperator::Empty,
                    right: Box::new(Expr::Empty),
                    is_not: false,
                },
            }
        }

        if should_expect_semicolon {
            self.expect(Token::Semicolon, "parse_variable_declaration")?;
        }

        Ok(res_statement)
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        match self.peek() {
            Token::Integer(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::Integer(n))
            }

            Token::Identifier(name) => {
                let n = name.clone();

                if matches!(self.peek_ahead(1), Some(Token::LeftParen)) {
                    let call = self.parse_function_call(n.clone())?;

                    if let Statement::FunctionCall {
                        function_name,
                        arguments,
                    } = call
                    {
                        let call_expr = Expr::FunctionCall {
                            function_name,
                            arguments,
                        };
                        return Ok(call_expr);
                    }
                }

                self.advance();
                Ok(Expr::Identifier(n))
            }

            Token::StringLiteral(str) => {
                let str = str.clone();
                self.advance();
                Ok(Expr::StringLiteral(str))
            }

            Token::BoolLiteral(b) => {
                let bool = *b;
                self.advance();
                Ok(Expr::BoolLiteral(bool))
            }

            _ => Err(CompileError::ParseError {
                message: format!("Expected expression, found {:?}", self.peek()),
                line: 0,
            }),
        }
    }

    fn parse_unary(&mut self) -> Result<Statement> {
        self.backtrack();

        let identifier = self.parse_expression()?;

        let name = if let Expr::Identifier(n) = &identifier {
            n.clone()
        } else {
            return Err(CompileError::ParseError {
                message: "Left side of unary must be an identifier".to_string(),
                line: 0,
            });
        };

        let operation = match self.peek() {
            Token::PlusPlus => Expr::BinaryOp {
                left: Box::new(identifier),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Integer(1)),
                is_not: false,
            },

            Token::MinusMinus => Expr::BinaryOp {
                left: Box::new(identifier),
                operator: BinaryOperator::Subtract,
                right: Box::new(Expr::Integer(1)),
                is_not: false,
            },

            _ => Expr::Empty,
        };

        self.advance();

        Ok(Statement::VariableAssignment {
            identifier: name,
            operation,
        })
    }

    fn parse_assignment(&mut self, identifier: Expr) -> Result<Statement> {
        self.advance();

        let mut is_not = false;
        if matches!(self.peek(), Token::Not) {
            is_not = true;
            self.advance();
        }

        let left = self.parse_expression()?;

        let value = match self.peek() {
            Token::Plus | Token::Minus => {
                let operator = match self.peek() {
                    Token::Plus => BinaryOperator::Add,
                    Token::Minus => BinaryOperator::Subtract,
                    _ => unreachable!(),
                };
                self.advance();

                let right = self.parse_expression()?;

                Expr::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                    is_not,
                }
            }
            _ => left,
        };

        self.expect(Token::Semicolon, "parse_assignment")?;

        let name = if let Expr::Identifier(name) = identifier {
            name
        } else {
            return Err(CompileError::ParseError {
                message: "Left side of assignment must be an identifier".to_string(),
                line: 0,
            });
        };

        Ok(Statement::VariableAssignment {
            identifier: name,
            operation: value,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;

        if matches!(self.peek(), Token::Equal) {
            return self.parse_assignment(expr);
        } else if matches!(self.peek(), Token::PlusPlus | Token::MinusMinus) {
            return self.parse_unary();
        }

        self.expect(Token::Semicolon, "parse_expression_statement")?;
        Ok(Statement::ExprStatement(expr))
    }

    fn parse_conditional(&mut self, default_is_not: bool) -> Result<Expr> {
        let mut is_not = default_is_not;
        if matches!(self.peek(), Token::Not) {
            is_not = !is_not;
            self.advance();
        }

        let left = self.parse_expression()?;

        if matches!(self.peek(), Token::RightParen) {
            return Ok(Expr::BinaryOp {
                left: Box::new(left),
                operator: BinaryOperator::Equal,
                right: Box::new(Expr::Empty),
                is_not,
            });
        }

        let operator = match self.peek() {
            Token::LessThan => BinaryOperator::LessThan,
            Token::GreaterThan => BinaryOperator::GreaterThan,
            Token::LessThanEqual => BinaryOperator::LessEqual,
            Token::GreaterThanEqual => BinaryOperator::GreaterEqual,
            Token::EqualEqual => BinaryOperator::Equal,
            Token::NotEqual => BinaryOperator::NotEqual,
            _ => panic!("{:?} not implemented in parse_conditional", self.peek()),
        };

        self.advance();

        let right = self.parse_expression()?;

        Ok(Expr::BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            is_not,
        })
    }

    fn parse_while(&mut self) -> Result<Statement> {
        self.advance();

        self.expect(Token::LeftParen, "parse_while")?;
        let condition = self.parse_conditional(false)?;

        self.expect(Token::RightParen, "parse_while")?;

        self.expect(Token::LeftBrace, "parse_while")?;
        let body = self.parse_block()?;
        self.expect(Token::RightBrace, "parse_while")?;

        let body_label = format!("while_{}_body", self.num_while);
        let end_label = format!("while_{}_end", self.num_while);
        self.num_while += 1;

        Ok(Statement::While {
            body_label,
            end_label,
            condition,
            body,
        })
    }

    fn parse_for(&mut self) -> Result<Statement> {
        self.advance();
        self.expect(Token::LeftParen, "parse_for")?;

        let init = self.parse_variable_declaration()?;
        let condition = self.parse_conditional(false)?;

        self.expect(Token::Semicolon, "parse_for")?;

        let var_change = self.parse_expression_statement()?;

        self.expect(Token::RightParen, "parse_for")?;

        self.expect(Token::LeftBrace, "parse_for")?;
        let body = self.parse_block()?;
        self.expect(Token::RightBrace, "parse_for")?;

        let body_label = format!("for_{}_body", self.num_for);
        let end_label = format!("for_{}_end", self.num_for);
        self.num_for += 1;

        Ok(Statement::For {
            init: Box::new(init),
            body_label,
            end_label,
            condition,
            body,
            var_change: Box::new(var_change),
        })
    }

    fn parse_if(&mut self) -> Result<Statement> {
        self.advance();

        self.expect(Token::LeftParen, "parse_if")?;
        let condition = self.parse_conditional(true)?;

        self.expect(Token::RightParen, "parse_if")?;

        self.expect(Token::LeftBrace, "parse_if")?;
        let body = self.parse_block()?;
        self.expect(Token::RightBrace, "parse_if")?;

        let label = format!("if_{}", self.num_if);
        self.num_if += 1;

        Ok(Statement::If {
            label,
            condition,
            body,
        })
    }

    fn parse_function_call(&mut self, function_name: String) -> Result<Statement> {
        self.advance();
        self.expect(Token::LeftParen, "parse_function_call")?;

        let mut arguments: Vec<Argument> = Vec::new();

        loop {
            let expr = self.parse_expression()?;
            arguments.push(Argument { expr });

            if !matches!(self.peek(), Token::Comma) {
                break;
            }
            self.advance();
        }

        self.expect(Token::RightParen, "parse_builtin_function")?;
        self.expect(Token::Semicolon, "parse_builtin_function")?;

        Ok(Statement::FunctionCall {
            function_name,
            arguments,
        })
    }

    fn parse_builtin_function(&mut self) -> Result<Statement> {
        let function_type = match self.peek() {
            Token::IPrint => BuiltinFunctionType::IntegerPrint,
            Token::SPrint => BuiltinFunctionType::StringPrint,

            _ => {
                panic!(
                    "Builtin function type {:?} not implemented in parse_builtin_function",
                    self.peek()
                );
            }
        };

        self.advance();

        self.expect(Token::LeftParen, "parse_builtin_function")?;

        let mut args: Vec<Argument> = Vec::new();

        loop {
            let expr = self.parse_expression()?;
            args.push(Argument { expr });

            if !matches!(self.peek(), Token::Comma) {
                break;
            }
            self.advance();
        }

        self.expect(Token::RightParen, "parse_builtin_function")?;
        self.expect(Token::Semicolon, "parse_builtin_function")?;

        Ok(Statement::BuiltinFunctionCall {
            function_type,
            arguments: args,
        })
    }

    fn populate_data_segment(&mut self, text: &[Statement]) -> Vec<Statement> {
        let mut strings = Vec::new();

        for stmt in text {
            if let Statement::Function { body, .. } = stmt {
                self.collect_string_literals(body, &mut strings);
            }
        }

        let mut seen = std::collections::HashSet::new();
        strings.retain(|s| seen.insert(s.clone()));

        strings
            .into_iter()
            .enumerate()
            .map(|(i, value)| Statement::DataDeclaration {
                label: format!("str_{}", i),
                storage_type: DataStorageType::Asciiz,
                value,
            })
            .collect()
    }

    fn collect_string_literals(&self, statements: &[Statement], strings: &mut Vec<String>) {
        for stmt in statements {
            match stmt {
                Statement::VariableDeclaration {
                    var_type: Type::String,
                    operation: Expr::BinaryOp { left, .. },
                    ..
                } => {
                    if let Expr::StringLiteral(value) = &**left {
                        strings.push(value.clone());
                    }
                }

                Statement::BuiltinFunctionCall { arguments, .. } => {
                    for arg in arguments {
                        if let Expr::StringLiteral(s) = &arg.expr {
                            strings.push(s.clone());
                        }
                    }
                }

                Statement::While { body, .. }
                | Statement::If { body, .. }
                | Statement::For { body, .. } => {
                    self.collect_string_literals(body, strings);
                }

                _ => {}
            }
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
    }

    fn backtrack(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn expect(&mut self, expected: Token, caller: &str) -> Result<()> {
        if self.peek() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(CompileError::ParseError {
                message: format!(
                    "Expected {:?}, found {:?} in {}",
                    expected,
                    self.peek(),
                    caller
                ),
                line: 0,
            })
        }
    }

    fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current + n)
    }

    fn peek_behind(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current - n)
    }
}
