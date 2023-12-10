use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{
    is_not, take, take_till, take_until, take_until1, take_while1, take_while_m_n,
};
use nom::character::complete::{alpha1, anychar, digit1, line_ending, one_of, space0};
use nom::combinator::{consumed, fail, map_res, not, verify};
use nom::sequence::{delimited, preceded, separated_pair, terminated, Tuple};
use nom::{bytes::complete::tag, character::complete::char};
use nom::{AsChar, IResult};

use crate::block::AssignmentBlock;
use crate::expression::{AExp, BExp, Value, Variable};
use crate::statement::Statement;

fn surrounded_by_parens(s: &str) -> bool {
    let mut nesting_level = 0;

    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == '(' {
            nesting_level += 1;
        } else if s.chars().nth(i).unwrap() == ')' {
            nesting_level -= 1;
        }
        if (i == 0 || i != (s.len() - 1)) && nesting_level <= 0 {
            return false;
        }
    }

    true
}

fn space(s: &str) -> IResult<&str, &str> {
    alt((space0, line_ending))(s)
}

fn delims(s: &str) -> IResult<&str, &str> {
    alt((
        //
        tag(";"),
        tag("then"),
        tag("else"),
        tag("do"),
        tag("end"),
    ))(s)
}

fn till_delims1(s: &str) -> IResult<&str, &str> {
    let mut index = 0;

    while index < s.len() && delims(&s[index..]).is_err() {
        index += 1;
    }

    match index {
        0 => fail(s),
        _ => Ok((&s[index..], &s[..index])),
    }
}

pub fn next_token(s: &str) -> IResult<&str, &str> {
    let (s, _) = space(s)?;

    let any = alt((
        tag/* ::<&str, &str, ()> */("if"),
        tag("then"),
        tag("else"),
        tag("while"),
        tag("do"),
        tag("end"),
        till_delims1,
    ))(s);

    if any.is_ok() {
        any
    } else {
        let (s, _) = tag(";")(s)?;
        next_token(s)
    }
}

fn var(s: &str) -> IResult<&str, AExp> {
    let (s_2, var) = delimited(space, take(1usize), space)(s)?;
    let c = var.chars().nth(0).unwrap();

    if (c as u8) < ('x' as u8) {
        return fail(s);
    }

    Ok((s_2, AExp::Variable((c as u8) - ('x' as u8))))
}

fn nval(s: &str) -> IResult<&str, AExp> {
    let (s, val) = map_res(delimited(space, digit1, space), FromStr::from_str)(s)?;

    Ok((s, AExp::Number(val)))
}

fn bval(s: &str) -> IResult<&str, BExp> {
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

pub fn aexp(s: &str) -> IResult<&str, AExp> {
    let s = s.trim();

    if surrounded_by_parens(s) {
        let subexpr_str = &s[1..s.len() - 1];
        let (s, subexpr) = aexp(subexpr_str)?;
        return Ok((s, subexpr));
    }

    let (s, lhs_str) = delimited(space, till_a_op0, space)(s)?;
    let (_, lhs) = alt((var, nval))(lhs_str)?;

    if s.len() == 0 {
        return Ok((s, lhs));
    }

    let (s, op) = get_a_op(s)?;
    let (s, rhs) = aexp(s)?;

    Ok((
        s,
        AExp::ArithmeticOp(Box::new(lhs), op.to_string(), Box::new(rhs)),
    ))
}

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

pub fn assignment(s: &str) -> IResult<&str, AssignmentBlock> {
    let (s, v_str) = terminated(alpha1, tag(":="))(s)?;
    let (s, expr_str) = alpha1(s)?;

    let (_, v) = var(v_str)?;
    let (_, expr) = aexp(expr_str)?;

    let var = match v {
        AExp::Variable(index) => index,
        _ => unreachable!(),
    };

    Ok((
        s,
        AssignmentBlock {
            var,
            expr,
            label: 0,
        },
    ))
}
