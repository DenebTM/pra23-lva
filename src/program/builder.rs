use crate::{
    block::{Block, SkipBlock},
    expression::Label,
    statement::{boxed, Statement},
};

use super::Program;

#[derive(Clone, Debug)]
pub struct ProgramBuilder<'a> {
    contents: Statement<'a>,
    next_label: Label,
}

impl<'a> ProgramBuilder<'a> {
    pub fn new() -> Self {
        Self {
            contents: Statement::Empty,
            next_label: 1,
        }
    }

    pub fn skip(self) -> Self {
        let new_stmt = Statement::Atom(Block::skip(self.next_label));
        Self {
            contents: Self::append(self.contents, new_stmt),
            next_label: self.next_label + 1,
        }
    }

    fn append(stmt: Statement<'a>, next: Statement<'a>) -> Statement<'a> {
        match stmt {
            Statement::Empty => next,
            Statement::Composition(stmt1, stmt2) => {
                Statement::Composition(stmt1, Box::new(Self::append(*stmt2, next)))
            }
            other_first => Statement::Composition(Box::new(other_first), Box::new(next)),
        }
    }
}
