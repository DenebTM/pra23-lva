use crate::{
    block::{Block, TestBlock},
    expression::{AExp, BExp, Label, Variable},
    statement::Statement,
};

#[derive(Clone, Debug)]
pub struct StatementBuilder<'a> {
    contents: Statement<'a>,
    next_label: Label,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(first_label: Label) -> Self {
        Self {
            contents: Statement::Empty,
            next_label: first_label,
        }
    }

    pub fn assignment(self, var: Variable, expr: AExp<'a>) -> Self {
        let new_stmt = Statement::Atom(Block::assignment(self.next_label, var, expr));
        Self {
            contents: append(self.contents, new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn skip(self) -> Self {
        let new_stmt = Statement::Atom(Block::skip(self.next_label));
        Self {
            contents: append(self.contents, new_stmt),
            next_label: self.next_label + 1,
        }
    }

    pub fn test(self, expr: BExp<'a>) -> Self {
        let new_stmt = Statement::Atom(Block::test(self.next_label, expr));
        Self {
            contents: append(self.contents, new_stmt),
            next_label: self.next_label + 1,
        }
    }
}

fn append<'a>(stmt: Statement<'a>, next: Statement<'a>) -> Statement<'a> {
    match stmt {
        Statement::Empty => next,
        Statement::Composition(stmt1, stmt2) => {
            Statement::Composition(stmt1, Box::new(append(*stmt2, next)))
        }
        other_first => Statement::Composition(Box::new(other_first), Box::new(next)),
    }
}
