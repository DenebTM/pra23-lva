#![allow(dead_code)]
use std::fmt::Display;

pub type Label = i32; // label index
pub type Variable = u8; // variable index
pub type Value = i32; // an actual numeric value (only for displaying)

/// represents an arithmetic expression as it may appear in an assignment to a variable
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum AExp<'a> {
    // the index of a variable
    Variable(Variable),

    // the value of the number is irrelevant
    Number(Value),

    // + - * /; operator is irrelevant
    ArithmeticOp(&'a AExp<'a>, &'a str, &'a AExp<'a>),
}

/// represents a boolean expression as it may appear (by itself) in a block
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum BExp<'a> {
    True,
    False,

    Not(Box<BExp<'a>>),

    // &&/||; operator is irrelevant
    BooleanOp(&'a BExp<'a>, &'a str, &'a BExp<'a>),

    // > < ==; operator is irrelevant
    RelationalOp(&'a AExp<'a>, &'a str, &'a AExp<'a>),
}

impl<'a> Display for AExp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AExp::Variable(var) => (('x' as u8 + var) as char).to_string(),

                AExp::Number(val) => val.to_string(),

                AExp::ArithmeticOp(lhs, op, rhs) =>
                    [lhs.to_string(), op.to_string(), rhs.to_string()].concat(),
            }
        )
    }
}

impl<'a> Display for BExp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BExp::True => "true".to_string(),

                BExp::False => "false".to_string(),

                BExp::Not(val) => ["!(", &val.to_string(), ")"].concat(),

                BExp::BooleanOp(lhs, op, rhs) =>
                    [lhs.to_string(), op.to_string(), rhs.to_string()].concat(),

                BExp::RelationalOp(lhs, op, rhs) =>
                    [lhs.to_string(), op.to_string(), rhs.to_string()].concat(),
            }
        )
    }
}
