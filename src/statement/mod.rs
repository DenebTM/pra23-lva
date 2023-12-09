#![allow(dead_code)]

pub mod builder;

use std::fmt::Display;

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

    // represents an empty program
    Empty,
}

impl<'a> Statement<'a> {
    pub fn get_label(&self) -> Label {
        match self {
            Self::Atom(block) => block.get_label(),

            Self::Composition(stmt1, _) => stmt1.get_label(),

            Self::IfThenElse(test, _, _) => test.label,

            Self::While(test, _) => test.label,

            // TODO: this isn't great
            Self::Empty => panic!("An empty statement has no label"),
        }
    }

    pub fn append(self, next: Statement<'a>) -> Statement<'a> {
        match self {
            Statement::Empty => next,
            Statement::Composition(stmt1, stmt2) => {
                Statement::Composition(stmt1, Box::new(stmt2.append(next)))
            }
            other_first => Statement::Composition(Box::new(other_first), Box::new(next)),
        }
    }
}

impl<'a> Display for Statement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Atom(block) => block.to_string(),

                Self::Composition(stmt1, stmt2) => {
                    format!("{}; {}", stmt1, stmt2)
                }

                Self::IfThenElse(test, stmt1, stmt2) => {
                    format!(
                        "(if {} then {} else {})",
                        Block::Test(test.clone()),
                        stmt1,
                        stmt2
                    )
                }

                Self::While(test, stmt1) => {
                    format!("while ({}) do ({})", Block::Test(test.clone()), stmt1,)
                }

                Self::Empty => "".to_string(),
            }
        )
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

    pub fn if_then_else<'a>(
        test: TestBlock<'a>,
        stmt1: Box<Statement<'a>>,
        stmt2: Box<Statement<'a>>,
    ) -> Box<Statement<'a>> {
        Box::new(Statement::IfThenElse(test, stmt1, stmt2))
    }

    pub fn while_<'a>(test: TestBlock<'a>, stmt1: Box<Statement<'a>>) -> Box<Statement<'a>> {
        Box::new(Statement::While(test, stmt1))
    }
}
