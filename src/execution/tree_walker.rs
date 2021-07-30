use crate::parsing::ast::{Expr, Op};

pub fn visit_expr(expr:Box<Expr>) -> Result<i64, String> {
    match *expr {
        Expr::Number(n) => {Ok(n)}
        Expr::BinaryOp { left, operator, right} => {
            let left = visit_expr(left)?;
            let right = visit_expr(right)?;

            match operator {
                Op::Mul => {
                    Ok(left * right)
                }
                Op::Div => {
                    if right==0 {
                        Err("zero division".to_string())
                    }else{
                        Ok(left / right)
                    }
                }
                Op::Add => { Ok(left + right)}
                Op::Sub => {Ok(left - right)}
            }
        }

        Expr::UnaryOp { operator, operand } => {
            match operator {
                Op::Sub => {
                    let operand = visit_expr(operand)?;
                    Ok(-operand)
                }
                _ => { panic!() /*should never happen*/ }
            }
        }
    }
}