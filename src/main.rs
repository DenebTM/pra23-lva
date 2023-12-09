mod algorithm;
mod analysis;
mod block;
mod expression;
mod functions;
mod program;
mod statement;

use block::TestBlock;
use expression::{AExp::*, BExp::*};
use statement::{boxed::*, Statement::Composition};

use crate::program::Program;

fn main() {
    let program = Program::new(Composition(
        assignment(0, 0, Number(2)),
        composition(
            assignment(0, 1, Number(4)),
            composition(
                assignment(0, 0, Number(1)),
                composition(
                    if_then_else(
                        TestBlock {
                            label: 0,
                            expr: RelationalOp(&Variable(1), ">", &Variable(0)),
                        },
                        assignment(0, 2, Variable(1)),
                        assignment(0, 2, ArithmeticOp(&Variable(1), "*", &Variable(1))),
                    ),
                    assignment(0, 0, Variable(2)),
                ),
            ),
        ),
    ));

    println!("{:?}", program.flow_r());

    for block in program.blocks() {
        print!("{}, ", block);
    }
    println!();

    let lva = algorithm::chaotic_iter::run(&program);
    println!("{:#?}", lva);
}
