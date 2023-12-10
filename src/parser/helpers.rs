use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space0},
    combinator::fail,
    IResult,
};

pub fn surrounded_by_parens(s: &str) -> bool {
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

pub fn space(s: &str) -> IResult<&str, &str> {
    alt((space0, line_ending))(s)
}

pub fn delims(s: &str) -> IResult<&str, &str> {
    alt((tag(";"), tag("then"), tag("else"), tag("do"), tag("end")))(s)
}

pub fn till_delims1(s: &str) -> IResult<&str, &str> {
    let mut index = 0;

    while index < s.len() && delims(&s[index..]).is_err() {
        index += 1;
    }

    match index {
        0 => fail(s),
        _ => Ok((&s[index..], &s[..index])),
    }
}
