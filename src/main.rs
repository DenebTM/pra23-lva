mod algorithm;
mod analysis;
mod block;
mod expression;
mod functions;
mod program;
mod statement;

use expression::{AExp::*, BExp::*};

use crate::{program::Program, statement::builder::StatementBuilder};

fn main() {
    let program = Program::new(
        StatementBuilder::new(1)
            .assignment(0, Number(2))
            .assignment(1, Number(4))
            .assignment(0, Number(1))
            .if_then(RelationalOp(&Variable(1), ">", &Variable(0)))
            .assignment(2, Variable(1))
            .else_()
            .assignment(2, ArithmeticOp(&Variable(1), "*", &Variable(1)))
            .end()
            .assignment(0, Variable(2))
            .end(),
    );

    println!("{:?}", program.flow_r());

    for block in program.blocks() {
        print!("{}, ", block);
    }
    println!();

    // let lva = algorithm::chaotic_iter::run(&program);
    let lva = algorithm::mfp::run(&program);

    for label in 1..=program.len {
        println!(
            "{label}: entry={:?}, exit={:?}",
            lva.entry[&label], lva.exit[&label],
        )
    }
}
