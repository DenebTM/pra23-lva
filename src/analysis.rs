use std::{collections::HashSet, ops::Sub};

use crate::{
    block::{AssignmentBlock, Block, TestBlock},
    expression::{Label, Variable},
    program::Program,
};

pub fn gen_lv(block: Block) -> HashSet<Variable> {
    match block {
        Block::Assignment(AssignmentBlock { expr, .. }) => expr.free_vars(),
        Block::Test(TestBlock { expr, .. }) => expr.free_vars(),
        Block::Skip(_) => [].into(),
    }
}

pub fn kill_lv(block: Block) -> HashSet<Variable> {
    match block {
        Block::Assignment(AssignmentBlock { var, .. }) => [var].into(),
        Block::Test(TestBlock { .. }) => [].into(),
        Block::Skip(_) => [].into(),
    }
}

pub fn lv_exit<'a>(program: &'a Program<'a>, label: Label) -> HashSet<Variable> {
    assert!(program.at(label) != None);

    if program.final_labels().contains(&label) {
        HashSet::new()
    } else {
        program
            .flow_r()
            .iter()
            .map(|(l_prime, _)| lv_entry(program, *l_prime))
            .flatten()
            .collect()
    }
}

pub fn lv_entry<'a>(program: &'a Program<'a>, label: Label) -> HashSet<Variable> {
    let block = program.at(label).unwrap();

    lv_exit(program, label)
        .sub(&kill(block.clone()))
        .union(&gen(block))
        .cloned()
        .collect()
}
