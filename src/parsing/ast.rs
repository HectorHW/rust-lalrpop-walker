#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Number(i64),
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
pub enum Op {
    Mul,
    Div,
    Add,
    Sub
}