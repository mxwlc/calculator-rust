#[macro_use]
extern crate lalrpop_util;
use ast::{Expr, Opcode};
pub mod ast;
lalrpop_mod!(pub calculator);

#[derive(Clone,Debug)]
pub enum Errors {
    DivByZero,
    SyntaxError,
}

type MathResult = Result<i32, Errors>;

fn evaluate(expr: &ast::Expr) -> MathResult {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Op(left, opcode, right) => match opcode {
            Opcode::Add => {
                println!("add");
                Ok(evaluate(&left)? + evaluate(&right)?)
            }
            Opcode::Div => {
                println!("div");
                if evaluate(&right)? == 0 {
                    return Err(Errors::DivByZero);
                }
                return Ok(evaluate(&left)? / evaluate(&right)?);
            }
            Opcode::Mul => {
                println!("mul");
                Ok(evaluate(&left)? * evaluate(&right)?)
            }
            Opcode::Sub => {
                println!("sub");
                Ok(evaluate(&left)? - evaluate(&right)?)
            }
        },
        Expr::Error => Err(Errors::SyntaxError),
    }
}

#[test]
fn calculator1() {
    let expr = calculator::ExprParser::new().parse("1+2-3").unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", evaluate(&expr).unwrap()), "0");
}

#[test]
fn calculator2() {
    let expr = calculator::ExprParser::new().parse("3*4/6").unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", evaluate(&expr).unwrap()), "2");
}

#[test]
fn calculator3() {
    let expr = calculator::ExprParser::new()
        .parse("8/0")
        .unwrap();

    println!("{:?}", expr);
    assert!(evaluate(&expr).is_err());
}

#[cfg(not(test))]
fn main() {
    let expr = calculator::ExprParser::new().parse("1+2-7").unwrap();

    println!("{:?}", evaluate(&expr));
}
