#[macro_use]
extern crate lalrpop_util;
use ast::{Expr, Opcode};
pub mod ast;
lalrpop_mod!(pub calculator);

fn evaluate(expr: &ast::Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Op(left, opcode, right) => match opcode {
            Opcode::Add => {
                println!("add");
                evaluate(&left) + evaluate(&right)
            }
            Opcode::Div => {
                println!("div");
                evaluate(&left) / evaluate(&right)
            }
            Opcode::Mul => {
                println!("mul");
                evaluate(&left) * evaluate(&right)
            }
            Opcode::Sub => {
                println!("sub");
                evaluate(&left) - evaluate(&right)
            }
        },
        Expr::Error => 0,
    }
}

#[test]
fn calculator1() {
    let expr = calculator::ExprParser::new().parse("1+2-3").unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", evaluate(&expr)), "0");
}

#[test]
fn calculator2() {
    let expr = calculator::ExprParser::new().parse("3*4/6").unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", evaluate(&expr)), "2");
}

#[test]
fn calculator3() {
    let expr = calculator::ExprParser::new()
        .parse("8/0")
        .unwrap();

    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", evaluate(&expr)), "0");
}

#[cfg(not(test))]
fn main() {
    let expr = calculator::ExprParser::new().parse("1+2-7").unwrap();

    println!("{:?}", evaluate(&expr));
}
