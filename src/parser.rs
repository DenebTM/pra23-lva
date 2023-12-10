use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{take, take_till, take_until, take_while1, take_while_m_n};
use nom::character::complete::{alpha1, anychar, digit1, line_ending, one_of, space0};
use nom::combinator::{map_res, verify};
use nom::sequence::{delimited, separated_pair, terminated, Tuple};
use nom::{bytes::complete::tag, character::complete::char};
use nom::{AsChar, IResult};

use crate::block::AssignmentBlock;
use crate::expression::{AExp, Value, Variable};

fn alpha_single(s: &str) -> IResult<&str, char> {
    verify(anychar, |c| c.is_alpha())(s)
}

fn space(s: &str) -> IResult<&str, &str> {
    alt((space0, line_ending))(s)
}

fn var(s: &str) -> IResult<&str, AExp> {
    let (s, var) = delimited(space, alpha1, space)(s)?;
    let c = s.chars().nth(0).unwrap();

    Ok((s, AExp::Variable((c as u8) - ('x' as u8))))
}

fn val(s: &str) -> IResult<&str, AExp> {
    let (s, val) = map_res(delimited(space, digit1, space), FromStr::from_str)(s)?;

    Ok((s, AExp::Number(val)))
}

fn is_a_op(c: char) -> bool {
    "+-*/".contains(c)
}
fn till_a_op(s: &str) -> IResult<&str, &str> {
    take_till(is_a_op)(s)
}
fn get_a_op(s: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, is_a_op)(s)
}

fn aexp(s: &str) -> IResult<&str, AExp> {
    let (s, lhs_str) = delimited(space, till_a_op, space)(s)?;
    let (_, lhs) = alt((var, val))(lhs_str)?;

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

fn assignment(s: &str) -> IResult<&str, AssignmentBlock> {
    let (s, var) = terminated(alpha1, tag(":="))(s)?;
    let (s, expr) = alpha1(s)?;

    todo!()
}

fn statement(s: &str) -> IResult<&str, &str> {
    take_until(";")(s)
}
