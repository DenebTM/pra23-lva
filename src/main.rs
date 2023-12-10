mod algorithm;
mod analysis;
mod block;
mod expression;
mod functions;
mod parser;
mod parser2;
mod program;
mod statement;

use expression::{AExp::*, BExp::*};

use crate::{parser2::parse, program::Program, statement::builder::StatementBuilder};

fn main() {
    let program = Program::new(
        StatementBuilder::new(1)
            .assignment(0, Number(2))
            .assignment(1, Number(4))
            .assignment(0, Number(1))
            .begin_if(RelationalOp(Variable(1), ">".to_string(), Variable(0)))
            .assignment(2, Variable(1))
            .else_()
            .assignment(
                2,
                ArithmeticOp(
                    Box::new(Variable(1)),
                    "*".to_string(),
                    Box::new(Variable(1)),
                ),
            )
            .end_if()
            .assignment(0, Variable(2))
            .end(),
    );

    println!("{}", program);
    println!("{:?}", program.flow_r());
    println!();

    // let lva = algorithm::chaotic_iter::run(&program);
    let lva = algorithm::mfp::run(&program);

    for label in 1..=program.len {
        println!(
            "{label}: entry={:?}, exit={:?}",
            lva.entry[&label], lva.exit[&label],
        )
    }

    parse("x := 2; y := 4; x := 1; if y > x then z := y else z := y*y end; x := z");
}
