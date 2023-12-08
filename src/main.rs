mod analysis;
mod block;
mod expression;
mod functions;
mod statement;

use block::TestBlock;
use expression::{AExp::*, BExp::*};
use functions::{blocks, flow_r};
use statement::{boxed::*, Statement::Composition};

fn main() {
    let stmt = Composition(
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
    );

    println!("{:?}", flow_r(&stmt));

    for block in blocks(&stmt) {
        print!("{}, ", block);
    }
    println!();
}
