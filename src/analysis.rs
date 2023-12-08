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

pub type LVExitAtLabel = HashSet<Variable>;
pub type LVExit = Vec<HashSet<Variable>>;
pub type LVEntryAtLabel = HashSet<Variable>;
pub type LVEntry = Vec<HashSet<Variable>>;

pub fn lv_exit<'a>(program: &'a Program<'a>, lv_entry: &LVEntry, label: Label) -> LVExitAtLabel {
    assert!(
        program.at(label) != None,
        "Label '{}' does not exist in program",
        label
    );
    assert!(
        label < lv_entry.len(),
        "Not enough entries in passed `lv_entry`"
    );

    if program.final_labels().contains(&label) {
        HashSet::new()
    } else {
        program
            .flow_r()
            .iter()
            .map(|(l_prime, _)| lv_entry.get(*l_prime).unwrap())
            .flatten()
            .cloned()
            .collect()
    }
}

pub fn lv_entry<'a>(program: &'a Program<'a>, lv_exit: &LVExit, label: Label) -> LVEntryAtLabel {
    let block = program.at(label).unwrap();

    lv_exit[label]
        .sub(&kill_lv(block.clone()))
        .union(&gen_lv(block))
        .cloned()
        .collect()
}
