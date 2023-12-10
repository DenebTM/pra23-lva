use crate::{
    expression::{BExp, Variable},
    parser::{bexp, next_token},
    statement::Statement,
};

pub fn parse(input: &str) /*  -> Statement */
{
    let (s, token) = next_token(input).unwrap();

    println!("{}", token);
    if s.len() > 0 {
        parse(s)
    }

    println!("{:?}", bexp("x := 1"));
}
