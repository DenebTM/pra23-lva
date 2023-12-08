use std::collections::HashSet;

use crate::{
    block::{AssignmentBlock, Block, TestBlock},
    expression::Variable,
};

pub fn gen(block: Block) -> HashSet<Variable> {
    match block {
        Block::Assignment(AssignmentBlock { expr, .. }) => expr.free_vars(),
        Block::Test(TestBlock { expr, .. }) => expr.free_vars(),
        Block::Skip(_) => [].into(),
    }
}

pub fn kill(block: Block) -> HashSet<Variable> {
    match block {
        Block::Assignment(AssignmentBlock { var, .. }) => [var].into(),
        Block::Test(TestBlock { .. }) => [].into(),
        Block::Skip(_) => [].into(),
    }
}
