#![allow(dead_code)]
use std::fmt::Display;

use crate::expression::{AExp, BExp, Label, Variable};

/// represents a single statement in a program
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Block<'a> {
    Assignment(AssignmentBlock<'a>),
    Skip(SkipBlock),
    Test(TestBlock<'a>),
}
impl<'a> Block<'a> {
    pub fn get_label(&self) -> Label {
        match self {
            Self::Assignment(b) => b.label,
            Self::Skip(b) => b.label,
            Self::Test(b) => b.label,
        }
    }

    pub fn assignment(label: Label, var: Variable, expr: AExp<'a>) -> Self {
        Self::Assignment(AssignmentBlock { label, var, expr })
    }
    pub fn skip(label: Label) -> Self {
        Self::Skip(SkipBlock { label })
    }
    pub fn test(label: Label, expr: BExp<'a>) -> Self {
        Self::Test(TestBlock { label, expr })
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AssignmentBlock<'a> {
    pub label: Label,
    pub var: Variable,
    pub expr: AExp<'a>,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct SkipBlock {
    pub label: Label,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TestBlock<'a> {
    pub label: Label,
    pub expr: BExp<'a>,
}

impl<'a> Display for Block<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]^{}",
            match self {
                Block::Assignment(AssignmentBlock { var, expr, .. }) => {
                    [
                        &(('x' as u8 + var) as char).to_string(),
                        " := ",
                        format!("{}", expr).as_str(),
                    ]
                    .concat()
                }

                Block::Skip(_) => "skip".to_string(),

                Block::Test(TestBlock { expr, .. }) => format!("{}", expr),
            },
            self.get_label()
        )
    }
}
