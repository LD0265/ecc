#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Int32,
    Int32Pointer,
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
    IntegerRead,
    StringRead,
    IntegerRandomRange,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitwiseShiftType {
    LeftShift,
    RightShift,
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
    Multiply,
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i32),
    Identifier(String),
    IdentifierReference(String),
    IdentifierDereference(String),
    StringLiteral(String),
    BoolLiteral(bool),
    Empty,

    BinaryOp {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
        is_not: bool
    },

    ArrayInitializer {
        body: Vec<Box<Expr>>,
        size: usize,
    },

    ArrayIndex {
        array_name: String,
        indexer: Box<Expr>,
    },

    BitwiseShift {
        identifier: Box<Expr>,
        shift_type: BitwiseShiftType,
    },

    FunctionCall {
        function_name: String,
        arguments: Vec<Argument>,
        is_builtin_function: bool,
        builtin_function_type: Option<BuiltinFunctionType>,
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
    pub typ: Type,
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
        
        // This doesn't feel right but it's easy
        is_dereference: bool,
        is_array_index: bool,
        indexer: Expr, 
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
        is_builtin_function: bool,
        builtin_function_type: Option<BuiltinFunctionType>,
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

    // NewLine,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub segments: Segments,
}
