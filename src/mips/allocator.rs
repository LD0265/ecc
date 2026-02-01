use std::collections::HashMap;

use crate::parser::ast::Statement;

#[derive(PartialEq, Clone)]
pub enum Registers {
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

pub struct Allocator {
    used_registers: Vec<Registers>,
    stack_variables: HashMap<String, usize>,
    stack_size: usize
}

impl Registers {
    pub fn to_string(&self) -> &'static str {
        match self {
            Registers::Zero => "$zero",
            Registers::AT => "$at",
            Registers::V0 => "$v0",
            Registers::V1 => "$v1",
            Registers::A0 => "$a0",
            Registers::A1 => "$a1",
            Registers::A2 => "$a2",
            Registers::A3 => "$a3",
            Registers::T0 => "$t0",
            Registers::T1 => "$t1",
            Registers::T2 => "$t2",
            Registers::T3 => "$t3",
            Registers::T4 => "$t4",
            Registers::T5 => "$t5",
            Registers::T6 => "$t6",
            Registers::T7 => "$t7",
            Registers::S0 => "$s0",
            Registers::S1 => "$s1",
            Registers::S2 => "$s2",
            Registers::S3 => "$s3",
            Registers::S4 => "$s4",
            Registers::S5 => "$s5",
            Registers::S6 => "$s6",
            Registers::S7 => "$s7",
            Registers::T8 => "$t8",
            Registers::T9 => "$t9",
            Registers::K0 => "$k0",
            Registers::K1 => "$k1",
            Registers::GP => "$gp",
            Registers::SP => "$sp",
            Registers::FP => "$fp",
            Registers::RA => "$ra",
        }
    }
}

impl Allocator {
    pub fn new() -> Self {
        Allocator {
            used_registers: Vec::new(),
            stack_variables: HashMap::new(),
            stack_size: 0,
        }
    }

    pub fn allocate_temp(&mut self) -> Option<Registers> {
        let temp_registers = vec![
            Registers::T0,
            Registers::T1,
            Registers::T2,
            Registers::T3,
            Registers::T4,
            Registers::T5,
            Registers::T6,
            Registers::T7,
        ];

        for reg in temp_registers {
            if !self.used_registers.contains(&reg) {
                self.used_registers.push(reg.clone());
                return Some(reg);
            }
        }

        None
    }

    pub fn free_temp(&mut self, reg: Registers) {
        if let Some(pos) = self.used_registers.iter().position(|x| *x == reg) {
            self.used_registers.remove(pos);
        }
    }

    pub fn calculate_needed_stack_space(&mut self, body: &Vec<Statement>) -> usize {
        let mut total = 4;

        for stmt in body {
            // We only need to worry about variable declarations for now
            if let Statement::VariableDeclaration { .. } = stmt {
                total += 4;
            }
        }

        self.stack_size = total;
        total
    }

    pub fn add_stack_variable(&mut self, name: &str) {
        let offset = self.stack_size - (4 * (self.stack_variables.len() + 1));
        self.stack_variables.insert(name.to_string(), offset);
    }

    pub fn get_stack_variable_offset(&self, name: &str) -> Option<usize> {
        self.stack_variables.get(name).cloned()
    }
}