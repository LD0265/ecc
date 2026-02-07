use core::num;
use std::collections::HashMap;
use std::fmt::{self, Display};

use crate::parser::ast::Statement;

#[derive(PartialEq, Clone)]
pub enum Register {
    Zero,
    AT,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    GP,
    SP,
    FP,
    RA,
}

#[derive(PartialEq)]
pub enum VariableLocation {
    Stack,
    ArgumentRegister,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Register::Zero => "$zero",
            Register::AT => "$at",
            Register::V0 => "$v0",
            Register::V1 => "$v1",
            Register::A0 => "$a0",
            Register::A1 => "$a1",
            Register::A2 => "$a2",
            Register::A3 => "$a3",
            Register::T0 => "$t0",
            Register::T1 => "$t1",
            Register::T2 => "$t2",
            Register::T3 => "$t3",
            Register::T4 => "$t4",
            Register::T5 => "$t5",
            Register::T6 => "$t6",
            Register::T7 => "$t7",
            Register::S0 => "$s0",
            Register::S1 => "$s1",
            Register::S2 => "$s2",
            Register::S3 => "$s3",
            Register::S4 => "$s4",
            Register::S5 => "$s5",
            Register::S6 => "$s6",
            Register::S7 => "$s7",
            Register::T8 => "$t8",
            Register::T9 => "$t9",
            Register::K0 => "$k0",
            Register::K1 => "$k1",
            Register::GP => "$gp",
            Register::SP => "$sp",
            Register::FP => "$fp",
            Register::RA => "$ra",
        };
        write!(f, "{}", s)
    }
}

pub struct Allocator {
    used_registers: Vec<Register>,
    argument_registers: HashMap<String, Register>,
    stack_variables: HashMap<String, usize>,
    stack_size: usize,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator {
            used_registers: Vec::new(),
            argument_registers: HashMap::new(),
            stack_variables: HashMap::new(),
            stack_size: 0,
        }
    }

    pub fn allocate_temp(&mut self) -> Option<Register> {
        let temp_registers = vec![
            Register::T0,
            Register::T1,
            Register::T2,
            Register::T3,
            Register::T4,
            Register::T5,
            Register::T6,
            Register::T7,
        ];

        for reg in temp_registers {
            if !self.used_registers.contains(&reg) {
                self.used_registers.push(reg.clone());
                return Some(reg);
            }
        }

        None
    }

    pub fn free_temp(&mut self, reg: Register) {
        if let Some(pos) = self.used_registers.iter().position(|x| *x == reg) {
            self.used_registers.remove(pos);
        }
    }

    pub fn calculate_needed_stack_space(
        &mut self,
        body: &Vec<Statement>,
        num_params: usize,
    ) -> usize {
        self.stack_size += 4 * (num_params + 1);

        self.calculate_needed_stack_space_helper(body);

        self.stack_size
    }

    fn calculate_needed_stack_space_helper(&mut self, statements: &[Statement]) {
        for stmt in statements {
            match stmt {
                Statement::VariableDeclaration { .. } => {
                    self.stack_size += 4;
                }

                Statement::While { body, .. } | Statement::If { body, .. } => {
                    self.calculate_needed_stack_space_helper(body);
                }

                Statement::For { body, .. } => {
                    // Add 4 because of init var decleration in parens
                    self.stack_size += 4;
                    self.calculate_needed_stack_space_helper(body);
                }

                _ => {}
            }
        }
    }

    pub fn add_argument(&mut self, name: &str) {
        let arg_registers = vec![Register::A0, Register::A1, Register::A2, Register::A3];

        let mut reg_to_use = Register::Zero;

        for arg_reg in arg_registers {
            if !self.used_registers.contains(&arg_reg) {
                reg_to_use = arg_reg;
                break;
            }
        }

        if reg_to_use == Register::Zero {
            panic!("Out of Argument registers");
        }

        self.argument_registers.insert(name.to_string(), reg_to_use);
    }

    pub fn get_argument_register(&self, name: &str) -> Option<String> {
        let reg = match self.argument_registers.get(name) {
            Some(r) => r,
            None => {
                panic!("{} not found in argument registers", name);
            }
        };

        Some(reg.to_string())
    }

    pub fn add_stack_variable(&mut self, name: &str) {
        let offset = self.stack_size - (4 * (self.stack_variables.len() + 1));
        self.stack_variables.insert(name.to_string(), offset);
    }

    pub fn get_stack_size(&self) -> &usize {
        &self.stack_size
    }

    pub fn get_stack_variable_offset(&self, name: &str) -> Option<usize> {
        self.stack_variables.get(name).cloned()
    }

    pub fn get_variable_location(&self, name: &str) -> VariableLocation {
        if self.stack_variables.contains_key(name) {
            return VariableLocation::Stack;
        } else {
            return VariableLocation::ArgumentRegister;
        }
    }

    // I should refactor mips.rs to use this more
    pub fn get_variable_register(&self, name: &str) -> Option<String> {
        if self.get_variable_location(name) == VariableLocation::Stack {
            let offset = match self.get_stack_variable_offset(name) {
                Some(n) => n,
                None => {
                    return None;
                }
            };
            return Some(format!("{}($sp)", offset));
        } else {
            let reg = match self.get_argument_register(name) {
                Some(r) => r,
                None => {
                    return None;
                }
            };
            return Some(reg);
        }
    }
}
