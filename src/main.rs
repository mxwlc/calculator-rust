//TODO: More Error Handling
//      User Interface




#[macro_use]
extern crate lalrpop_util;
use ast::{Expr, Opcode};
use std::io::{self, Write};
pub mod ast;


lalrpop_mod!(pub calculator);

#[derive(Clone,Debug)]






pub enum Errors {
    DivByZero,
    SyntaxError,
    Exit,
    Undefined,
}

type MathResult = Result<f64, Errors>;



fn evaluate(expr: &ast::Expr) -> MathResult {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Op(left, opcode, right) => match opcode {
            Opcode::Add => {
                Ok(evaluate(&left)? + evaluate(&right)?)
            }
            Opcode::Div => {
                if evaluate(&right)? == 0 as f64{
                    return Err(Errors::DivByZero);
                }
                return Ok(evaluate(&left)? / evaluate(&right)?);
            }
            Opcode::Mul => {
                Ok(evaluate(&left)? * evaluate(&right)?)
            }
            Opcode::Sub => {
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
    assert_eq!(&format!("{:?}", evaluate(&expr).unwrap()), "0.0");
}

#[test]
fn calculator2() {
    let expr = calculator::ExprParser::new().parse("3*4/6").unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", evaluate(&expr).unwrap()), "2.0");
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
fn main() -> io::Result<()>{
    println!("Welcome To the Calculator\n Input exit to terminate the program");
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "exit"{
            std::process::exit(0);
        }
        let result = calculator::ExprParser::new().parse(&input);

        match &result {
            Ok(expr) => {
                print!("{:?}", expr);
                let value = evaluate(&expr);
                match value {
                    Ok(result) => println!("={:?}", result),
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(err) => {println!("{:?}", err);}
        }
    }
}