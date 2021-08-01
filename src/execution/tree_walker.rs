use crate::parsing::ast::{Expr, Op, Program, Stmt};
use std::collections::HashMap;

type State = HashMap<String, i64>;

pub fn clear_run(program: Program) -> Result<(), String> {
    run_with_state(program, &mut HashMap::new())
}

pub fn run_keep_state(program: Program, state: &mut State) -> Result<(), String> {
    let state_backup = state.clone();
    match run_with_state(program, state) {
        Err(e) => {
            *state = state_backup;
            Err(e)
        }
        ok => ok
    }
}

fn run_with_state(program: Program, state: &mut State) -> Result<(), String> {
    for stmt in program {
        visit_stmt(stmt, state)?;
    }
    Ok(())
}

fn visit_stmt(stmt:Stmt, state: &mut State) -> Result<(), String> {
    match stmt {
        Stmt::AssignStmt { target, assignee } => {
            let assignee = visit_expr(assignee, state)?;
            state.insert(target, assignee);
        }
        Stmt::PrintStmt(target) => {
            match target {
                None => {println!()}
                Some(argument) => {
                    let argument = visit_expr(argument, state)?;
                    println!("{}", argument);
                }
            }
        }
    }
    Ok(())
}

fn visit_expr(expr:Box<Expr>, state:&mut State) -> Result<i64, String> {
    match *expr {
        Expr::Number(n) => {Ok(n)}
        Expr::BinaryOp { left, operator, right} => {
            let left = visit_expr(left, state)?;
            let right = visit_expr(right, state)?;

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
                    let operand = visit_expr(operand, state)?;
                    Ok(-operand)
                }
                _ => { panic!() /*should never happen*/ }
            }
        }
        Expr::Variable(varname) => {
            state.get(&varname)
                .map(|e| {*e})
                .ok_or(format!("unknown variable '{}'", varname))
        }
    }
}