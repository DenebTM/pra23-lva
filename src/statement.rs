use std::fmt::Display;

use crate::block::{Block, TestBlock};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Statement {
    // Assignment(Assignment),
    // Skip(Skip),
    // Test(Test),
    /// \[X := a\], \[skip\], \[b\]
    Atom(Block),

    /// S1; S2
    Sequence(Box<Statement>, Box<Statement>),

    /// if \[b\] then S1 else S2
    IfThenElse(TestBlock, Box<Statement>, Box<Statement>),

    /// while \[b\] do S
    While(TestBlock, Box<Statement>),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Atom(block) => block.to_string(),

                Self::Sequence(stmt1, stmt2) => {
                    format!("{}; {}", stmt1, stmt2)
                }

                Self::IfThenElse(test, stmt1, stmt2) => {
                    format!(
                        "if {} then {} else {} endif",
                        Block::Test(test.clone()),
                        stmt1,
                        stmt2
                    )
                }

                Self::While(test, stmt1) => {
                    format!("while {} do {} enddo", Block::Test(test.clone()), stmt1,)
                }
            }
        )
    }
}
