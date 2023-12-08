use crate::{
    block::{AssignmentBlock, Block, SkipBlock, TestBlock},
    expression::Label,
    statement::Statement,
};

pub struct Program<'a> {
    pub contents: Statement<'a>,
    pub len: i32,
}

impl<'a> Program<'a> {
    /// relabels a statement and returns it together with a following label
    fn relabel(stmt: &'a Statement<'a>, start: Label) -> (Statement<'a>, Label) {
        match stmt {
            Statement::Atom(block) => (
                Statement::Atom(match block {
                    Block::Assignment(AssignmentBlock { var, expr, .. }) => {
                        Block::assignment(start.clone(), *var, expr.clone())
                    }
                    Block::Skip(SkipBlock { .. }) => Block::skip(start.clone()),
                    Block::Test(TestBlock { expr, .. }) => Block::test(start.clone(), expr.clone()),

                    _ => todo!(),
                }),
                start + 1,
            ),

            Statement::Composition(stmt1, stmt2) => {
                let (new_stmt1, stmt2_start) = Program::relabel(&stmt1, start);
                let (new_stmt2, next) = Program::relabel(&stmt2, stmt2_start);

                (
                    Statement::Composition(Box::new(new_stmt1), Box::new(new_stmt2)),
                    next,
                )
            }

            Statement::IfThenElse(test, stmt1, stmt2) => {
                let (new_test, stmt1_start) = (
                    TestBlock {
                        label: start,
                        expr: test.expr.clone(),
                    },
                    start + 1,
                );
                let (new_stmt1, stmt2_start) = Program::relabel(&stmt1, stmt1_start);
                let (new_stmt2, next) = Program::relabel(&stmt2, stmt2_start);

                (
                    Statement::IfThenElse(new_test, Box::new(new_stmt1), Box::new(new_stmt2)),
                    next,
                )
            }

            Statement::While(test, stmt1) => {
                let (new_test, stmt1_start) = (
                    TestBlock {
                        label: start,
                        expr: test.expr.clone(),
                    },
                    start + 1,
                );
                let (new_stmt1, next) = Program::relabel(&stmt1, stmt1_start);

                (Statement::While(new_test, Box::new(new_stmt1)), next)
            }
        }
    }

    /// creates a new program, labelling all its statements sequentially
    pub fn new(contents: &'a mut Statement<'a>) -> Self {
        let (contents, len) = Program::relabel(contents, 1);
        Self { contents, len }
    }
}
