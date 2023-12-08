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
    /// creates a new program, labelling all its statements sequentially
    pub fn new(contents: &'a Statement<'a>) -> Self {
        let (contents, len) = Program::relabel(contents, 1);
        Self { contents, len }
    }

    /// returns the block at a specified label in the program
    pub fn at(&'a self, label: Label) -> Option<Block<'a>> {
        Program::stmt_at(&self.contents, label)
    }

    /// relabels a statement and returns it together with a following label (internal use)
    fn relabel(stmt: &'a Statement<'a>, start: Label) -> (Statement<'a>, Label) {
        match stmt {
            Statement::Atom(block) => (
                Statement::Atom(match block {
                    Block::Assignment(AssignmentBlock { var, expr, .. }) => {
                        Block::assignment(start.clone(), *var, expr.clone())
                    }
                    Block::Skip(SkipBlock { .. }) => Block::skip(start.clone()),
                    Block::Test(TestBlock { expr, .. }) => Block::test(start.clone(), expr.clone()),
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

    /// returns the block at a specified label in the program (internal use)
    fn stmt_at(stmt: &'a Statement<'a>, label: Label) -> Option<Block<'a>> {
        match stmt {
            Statement::Atom(block) => {
                if block.get_label() == label {
                    return Some(block.clone());
                }

                None
            }

            Statement::Composition(stmt1, stmt2) => {
                if let Some(block) = Program::stmt_at(stmt1, label) {
                    return Some(block);
                }
                if let Some(block) = Program::stmt_at(stmt2, label) {
                    return Some(block);
                }

                None
            }

            Statement::IfThenElse(test, stmt1, stmt2) => {
                if test.label == label {
                    return Some(Block::Test(test.clone()));
                }

                if let Some(block) = Program::stmt_at(stmt1, label) {
                    return Some(block);
                }
                if let Some(block) = Program::stmt_at(stmt2, label) {
                    return Some(block);
                }

                None
            }

            Statement::While(test, stmt1) => {
                if test.label == label {
                    return Some(Block::Test(test.clone()));
                }

                if let Some(block) = Program::stmt_at(stmt1, label) {
                    return Some(block);
                }

                None
            }
        }
    }
}