use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag_no_case as tag;
use nom::bytes::complete::{take, take_till};
use nom::character::complete::digit1;
use nom::combinator::{fail, map_res, verify};
use nom::sequence::delimited;
use nom::IResult;

use crate::expression::{AExp, BExp};

use super::helpers::{space, surrounded_by_parens};

pub fn var(s: &str) -> IResult<&str, AExp> {
    let (s_2, var) = delimited(space, take(1usize), space)(s)?;
    let c = var.chars().nth(0).unwrap();

    if (c as u8) < ('x' as u8) {
        return fail(s);
    }

    Ok((s_2, AExp::Variable((c as u8) - ('x' as u8))))
}

pub fn nval(s: &str) -> IResult<&str, AExp> {
    let (s, val) = map_res(delimited(space, digit1, space), FromStr::from_str)(s)?;

    Ok((s, AExp::Number(val)))
}

pub fn bval(s: &str) -> IResult<&str, BExp> {
    let (s, val) = delimited(space, alt((tag("true"), tag("false"))), space)(s)?;

    Ok((
        s,
        match val {
            "true" => BExp::True,
            "false" => BExp::False,
            _ => panic!(),
        },
    ))
}

fn is_a_op(c: char) -> bool {
    "+-*/".contains(c)
}
fn till_a_op0(s: &str) -> IResult<&str, &str> {
    take_till(is_a_op)(s)
}
fn get_a_op(s: &str) -> IResult<&str, &str> {
    verify(take(1usize), |s: &str| s.chars().all(|c| is_a_op(c)))(s)
}

fn is_b_op(c: char) -> bool {
    "|&".contains(c)
}
fn till_b_op(s: &str) -> IResult<&str, &str> {
    take_till(is_b_op)(s)
}
fn get_b_op(s: &str) -> IResult<&str, &str> {
    verify(take(1usize), |s: &str| s.chars().all(|c| is_b_op(c)))(s)
}

fn is_r_op(c: char) -> bool {
    ['=', '<', '>'].contains(&c)
}
fn till_r_op(s: &str) -> IResult<&str, &str> {
    take_till(is_r_op)(s)
    // verify(take_till(is_r_op), |s_2: &str| s_2.len() < s.len())(s)
}
fn get_r_op(s: &str) -> IResult<&str, &str> {
    verify(take(1usize), |s: &str| s.chars().all(|c| is_r_op(c)))(s)
}

/**
 * parse an arithmetic operation
 *
 * order of operations is not respected
 */
pub fn aexp(s: &str) -> IResult<&str, AExp> {
    let s = s.trim();

    if surrounded_by_parens(s) {
        let subexpr_str = &s[1..s.len() - 1];
        let (s, subexpr) = aexp(subexpr_str)?;
        return Ok((s, subexpr));
    }

    let (s, lhs_str) = delimited(space, till_a_op0, space)(s)?;
    let (s_2, lhs) = alt((var, nval))(lhs_str)?;

    if s.len() == 0 && s_2.len() == 0 {
        return Ok((s, lhs));
    }

    let (s, op) = get_a_op(s)?;
    let (s, rhs) = aexp(s)?;

    Ok((
        s,
        AExp::ArithmeticOp(Box::new(lhs), op.to_string(), Box::new(rhs)),
    ))
}

/**
 * parse a boolean operation
 *
 * order of operations is not respected
 */
pub fn bexp(s: &str) -> IResult<&str, BExp> {
    let s = s.trim();

    if let Ok((s, lhs)) = bval(s) {
        if s.len() == 0 {
            return Ok((s, lhs));
        }
    }

    if let Ok((subexpr_str, _)) = tag::<&str, &str, ()>("!")(s) {
        let (s, subexpr) = bexp(subexpr_str)?;
        return Ok((s, BExp::Not(Box::new(subexpr))));
    }

    if surrounded_by_parens(s) {
        let subexpr_str = &s[1..s.len() - 1];
        let (s, subexpr) = bexp(subexpr_str)?;
        return Ok((s, subexpr));
    }

    let (s, lhs_str): (&str, &str) = {
        let res_r = delimited(space, till_r_op, space)(s);
        let res_b = delimited(space, till_b_op, space)(s);

        if res_r.is_ok() && res_b.is_ok() {
            let (s_r, lhs_r) = res_r.unwrap();
            let (s_b, lhs_b) = res_b.unwrap();

            if s_r.len() > s_b.len() && !surrounded_by_parens(lhs_b.trim()) {
                (s_r, lhs_r)
            } else {
                (s_b, lhs_b)
            }
        } else if res_r.is_ok() {
            res_r.unwrap()
        } else if res_b.is_ok() {
            res_b.unwrap()
        } else {
            fail(s)?
        }
    };

    if let Ok((_, lhs)) = aexp(lhs_str) {
        let (s, r_op) = get_r_op(s)?;
        let (s, rhs) = aexp(s)?;
        return Ok((s, BExp::RelationalOp(lhs, r_op.to_string(), rhs)));
    }

    let (_, lhs) = bexp(lhs_str)?;
    let (s, b_op) = get_b_op(s)?;
    let (s, rhs) = bexp(s)?;

    Ok((
        s,
        BExp::BooleanOp(Box::new(lhs), b_op.to_string(), Box::new(rhs)),
    ))
}
