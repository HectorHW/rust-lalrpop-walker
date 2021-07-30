use super::calculator1::ExprParser;
use super::ast::{Expr, Op};

#[test]
fn test_wrong_token(){
    let parser = ExprParser::new();
    let str = "12#";
    let res = parser.parse(str);
    assert!(res.is_err());
}

#[test]
fn test_eof(){
    let parser = ExprParser::new();
    let str = "12+";
    let res = parser.parse(str);
    assert!(res.is_err());
}

#[test]
fn test_single(){
    let parser = ExprParser::new();
    let str = "12";
    let expected = Box::new(Expr::Number(12));
    let res = parser.parse(str);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
}

#[test]
fn test_paren(){
    let parser = ExprParser::new();
    let str = "(12)";
    let expected = Box::new(Expr::Number(12));
    let res = parser.parse(str);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
}

#[test]
fn test_priority(){
    let parser = ExprParser::new();
    let str = "1+2*-3";

    let neg = Box::new(Expr::UnaryOp {operator:Op::Sub,
        operand: Box::new(Expr::Number(3))});

    let mul = Box::new(
        Expr::BinaryOp {
            left: Box::new(Expr::Number(2)), operator: Op::Mul, right: neg
        });

    let expected = Box::new(Expr::BinaryOp {
        left: Box::new(Expr::Number(1)),
        operator: Op::Add,
        right: mul
    });
    let res = parser.parse(str);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
}