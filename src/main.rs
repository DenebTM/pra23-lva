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
        assignment(1, 2, Number(1)),
        while_(
            TestBlock {
                label: 2,
                expr: RelationalOp(&Variable(0), ">", &Number(0)),
            },
            composition(
                assignment(3, 2, ArithmeticOp(&Variable(2), "*", &Variable(1))),
                assignment(4, 0, ArithmeticOp(&Variable(0), "-", &Number(1))),
            ),
        ),
    ));

    println!("{:?}", program.flow_r());

    for block in program.blocks() {
        print!("{}, ", block);
    }
    println!();
}
