#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Int32,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i32),
    Identifier(String),
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
    pub statements: Vec<Statement>,
}
