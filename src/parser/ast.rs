#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Int32,
    String,
}

#[derive(Debug, Clone)]
pub enum DataStorageType {
    // I'll think about adding more later
    Asciiz,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i32),
    Identifier(String),
    StringLiteral(String),
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

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        var_type: Type,
        identifier: String,

        // I'll add more to this later
        init: Option<Expr>,
    },

    Function {
        name: String,
        params: Vec<Parameter>,
        return_type: Type,
        body: Vec<Statement>,
        use_stack: bool,
    },

    DataDeclaration {
        label: String,
        storage_type: DataStorageType,
        value: String,
    },

    Instruction {
        opcode: String,
        operands: Vec<String>,
    },

    // Return {
    //     value: Option<Expr>,
    // },
    ExprStatement(Expr),
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub segments: Segments,
}
