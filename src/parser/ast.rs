#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Int32,
    Bool,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataStorageType {
    // I'll think about adding more later
    Asciiz,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFunctionType {
    IntegerPrint,
    StringPrint,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    Add,
    Subtract,
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i32),
    Identifier(String),
    StringLiteral(String),
    BoolLiteral(bool),
    Empty,

    BinaryOp {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
        is_not: bool
    },

    FunctionCall {
        function_name: String,
        arguments: Vec<Argument>,
    },
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Segments {
    pub data: Segment,
    pub text: Segment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    DataDeclaration {
        label: String,
        storage_type: DataStorageType,
        value: String,
    },

    Instruction {
        opcode: String,
        operands: Vec<String>,
    },

    VariableDeclaration {
        var_type: Type,
        identifier: String,
        operation: Expr,
    },

    VariableAssignment {
        identifier: String,
        operation: Expr,
    },

    Function {
        name: String,
        params: Vec<Parameter>,
        return_type: Type,
        body: Vec<Statement>,
        use_stack: bool,
    },

    FunctionCall {
        function_name: String,
        arguments: Vec<Argument>,
    },

    BuiltinFunctionCall {
        function_type: BuiltinFunctionType,
        arguments: Vec<Argument>,
    },

    While {
        body_label: String,
        end_label: String,
        condition: Expr,
        body: Vec<Statement>,
    },

    For {
        init: Box<Statement>,
        body_label: String,
        end_label: String,
        condition: Expr,
        var_change: Box<Statement>,
        body: Vec<Statement>,
    },

    If {
        label: String,
        condition: Expr,
        body: Vec<Statement>,
    },

    Return {
        value: Expr,
    },

    ExprStatement(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub segments: Segments,
}
