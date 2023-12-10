use nom::bytes::complete::tag_no_case as tag;
use nom::bytes::complete::take;
use nom::combinator::fail;
use nom::sequence::{delimited, terminated};
use nom::IResult;

use crate::block::AssignmentBlock;
use crate::expression::{AExp, BExp};

use super::expression::{aexp, bexp, var};
use super::helpers::{space, till_delims1};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    Skip,
    If(BExp),
    Then,
    Else,
    While(BExp),
    Do,
    EndIf,
    EndDo,
    Assignment(AssignmentBlock),
    AExp(AExp),
    BExp(BExp),
}

pub fn next_token(s: &str) -> IResult<&str, Token> {
    let s = s.trim_start();

    if let Ok((o, _)) = tag::<&str, &str, ()>("skip")(s) {
        Ok((o, Token::Skip))
    } else if let Ok((s, _)) = tag::<&str, &str, ()>("if")(s) {
        let (o, test_token) = next_token(s)?;
        if let Token::BExp(test) = test_token {
            Ok((o, Token::If(test)))
        } else {
            fail(s)
        }
    } else if let Ok((o, _)) = tag::<&str, &str, ()>("then")(s) {
        Ok((o, Token::Then))
    } else if let Ok((o, _)) = tag::<&str, &str, ()>("else")(s) {
        Ok((o, Token::Else))
    } else if let Ok((o, _)) = tag::<&str, &str, ()>("while")(s) {
        let (o, test_token) = next_token(o)?;
        if let Token::BExp(test) = test_token {
            Ok((o, Token::While(test)))
        } else {
            fail(s)
        }
    } else if let Ok((o, _)) = tag::<&str, &str, ()>("do")(s) {
        Ok((o, Token::Do))
    } else if let Ok((o, _)) = tag::<&str, &str, ()>("endif")(s) {
        Ok((o, Token::EndIf))
    } else if let Ok((o, _)) = tag::<&str, &str, ()>("enddo")(s) {
        Ok((o, Token::EndDo))
    } else if let Ok((o, other)) = till_delims1(s) {
        if let Ok((_, ass)) = assignment(other) {
            Ok((o, Token::Assignment(ass)))
        } else if let Ok((_, expr)) = aexp(other) {
            Ok((o, Token::AExp(expr)))
        } else if let Ok((_, expr)) = bexp(other) {
            Ok((o, Token::BExp(expr)))
        } else {
            fail(s)
        }
    } else {
        let (s, _) = tag(";")(s)?;
        next_token(s)
    }
}

pub fn assignment(s: &str) -> IResult<&str, AssignmentBlock> {
    let (s, v_str) = terminated(delimited(space, take(1usize), space), tag(":="))(s)?;

    let (_, v) = var(v_str)?;
    let (_, expr) = aexp(s)?;

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
