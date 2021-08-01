#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Number(i64),
    Variable(String),
    BinaryOp{
        left: Box<Expr>,
        operator: Op,
        right: Box<Expr>,
    },

    UnaryOp{
        operator: Op,
        operand: Box<Expr>,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    AssignStmt{
        target: String,
        assignee: Box<Expr>
    },
    PrintStmt(Option<Box<Expr>>)
}

pub type  Program = Vec<Stmt>;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub
}