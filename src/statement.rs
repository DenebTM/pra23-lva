#![allow(dead_code)]
use crate::{
    block::{Block, TestBlock},
    expression::Label,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Statement<'a> {
    // Assignment(Assignment),
    // Skip(Skip),
    // Test(Test),
    /// \[X := a\], \[skip\], \[b\]
    Atom(Block<'a>),

    /// S1; S2
    Composition(Box<Statement<'a>>, Box<Statement<'a>>),

    /// if \[b\] then S1 else S2
    IfThenElse(TestBlock<'a>, Box<Statement<'a>>, Box<Statement<'a>>),

    /// while \[b\] do S
    While(TestBlock<'a>, Box<Statement<'a>>),
}

impl<'a> Statement<'a> {
    pub fn get_label(&self) -> Label {
        match self {
            Self::Atom(block) => block.get_label(),

            Self::Composition(stmt1, _) => stmt1.get_label(),

            Self::IfThenElse(test, _, _) => test.label,

            Self::While(test, _) => test.label,
        }
    }
}

pub mod boxed {
    use super::Statement;
    use crate::{
        block::{Block, TestBlock},
        expression::{AExp, BExp, Label, Variable},
    };

    pub fn assignment<'a>(label: Label, var: Variable, expr: AExp<'a>) -> Box<Statement> {
        Box::new(Statement::Atom(Block::assignment(label, var, expr)))
    }
    pub fn skip<'a>(label: Label) -> Box<Statement<'a>> {
        Box::new(Statement::Atom(Block::skip(label)))
    }
    pub fn test<'a>(label: Label, expr: BExp<'a>) -> Box<Statement<'a>> {
        Box::new(Statement::Atom(Block::test(label, expr)))
    }

    pub fn composition<'a>(
        stmt1: Box<Statement<'a>>,
        stmt2: Box<Statement<'a>>,
    ) -> Box<Statement<'a>> {
        Box::new(Statement::Composition(stmt1, stmt2))
    }
    pub fn while_<'a>(test: TestBlock<'a>, stmt1: Box<Statement<'a>>) -> Box<Statement<'a>> {
        Box::new(Statement::While(test, stmt1))
    }
}
