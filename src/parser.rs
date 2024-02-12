use peg::{self, error::ParseError, str::LineCol};

use crate::{
    block::{Block, TestBlock},
    expression::{AExp, BExp, Value, Variable},
    program::Program,
    statement::Statement,
};

peg::parser!(grammar while_() for str {
    rule __ = quiet!{ [' ' | '\n']+ }
    rule _  = quiet!{ [' ' | '\n']* }
    rule ws_or_eof() = &(_ / ![_])
    rule alpha() -> char = quiet!{ ['a'..='z' | 'A'..='Z'] }
    rule digit() -> char = quiet!{ ['0'..='9'] }
    rule neg() -> char = quiet!{ ['-'] }
    rule keyword() = quiet! { "if" / "then" / "else" / "endif" / "while" / "do" / "enddo" }

    rule constant() -> Value
        = n:$(neg()? digit()+) {? n.parse().or(Err("i32")) }
        / expected!("constant")

    rule variable() -> Variable
        = !keyword() x:alpha() ws_or_eof() { x }
        / expected!("variable")

    rule bexp() -> BExp
        = t:precedence!{
            x:(@) _ op:$("||")  _ y:@ { BExp::BooleanOp(Box::new(x), op.to_string(), Box::new(y)) }
            --
            x:(@) _ op:$("&&")  _ y:@ { BExp::BooleanOp(Box::new(x), op.to_string(), Box::new(y)) }
            --
            x:aexp() _ op:$("<=" / "==" / "!=" / ">=" / "<" / ">")  _ y:aexp() { BExp::RelationalOp(x, op.to_string(), y) }
            --
            "true" { BExp::True }
            "false" { BExp::False }
            --
            "(" _ t:bexp() _ ")" { t }
        }

    rule aexp() -> AExp
        = t:precedence!{
            x:(@) _ op:$("+" / "-") _ y:@ { AExp::ArithmeticOp(Box::new(x), op.to_string(), Box::new(y)) }
            --
            x:(@) _ op:$("*" / "/") _ y:@ { AExp::ArithmeticOp(Box::new(x), op.to_string(), Box::new(y)) }
            --
            n:constant() { AExp::Number(n) }
            v:variable() { AExp::Variable(v) }
            --
            "(" _ t:aexp() _ ")" { t }
        }

    rule if_then_else() -> Statement
        = "if" __ t0:bexp() __ "then" __ t1:stmt() __ "else" __ t2:stmt() __ "endif" {
            Statement::IfThenElse(TestBlock { label: 0, expr: t0 }, Box::new(t1), Box::new(t2))
        }

    rule while() -> Statement
        = "while" __ t0:bexp() __ "do" __ t1:stmt() __ "enddo" {
            Statement::While(TestBlock { label: 0, expr: t0 }, Box::new(t1))
        }

    rule atom() -> Statement
        = b:(
            x:variable() _ ":=" _ e:aexp() { Block::assignment(0, x, e) }
            / "skip" { Block::skip(0) }
            / e:bexp() { Block::test(0, e) }
        ) { Statement::Atom(b) }

    rule stmt() -> Statement
        = _ s:precedence!{
            s1:(@) _ ";" _ s2:@ { Statement::Sequence(Box::new(s1), Box::new(s2))}
            --
            s:if_then_else() { s }
            --
            s:while() { s }
            --
            s:atom() { s }
            --
            _ { Statement::Empty }
        } { s }

        pub rule program() -> Program = s:stmt() _ { Program::new(s) }
});

pub fn parse(input: &str) -> Result<Program, ParseError<LineCol>> {
    while_::program(input)
}
