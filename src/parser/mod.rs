mod expression;
mod helpers;
mod tokens;

use crate::{
    block::AssignmentBlock,
    parser::tokens::{next_token, Token},
    program::Program,
    statement::builder::StatementBuilder,
};

pub fn parse(input: &str) /*  -> Statement */
{
    let mut builder: StatementBuilder = StatementBuilder::new(1);

    let mut input = input;
    while input.len() > 0 {
        match next_token(input) {
            Ok((s, token)) => {
                builder = match &token {
                    Token::Skip => builder.skip(),
                    Token::If(test) => builder.begin_if(test.clone()),
                    Token::Then => builder,
                    Token::Else => builder.else_(),
                    Token::EndIf => builder.end_if(),
                    Token::While(test) => builder.begin_while(test.clone()),
                    Token::Do => builder,
                    Token::EndDo => builder.end_while(),
                    Token::Assignment(AssignmentBlock { var, expr, .. }) => {
                        builder.assignment(var.clone(), expr.clone())
                    }

                    Token::AExp(expr) => panic!("Stray arithmetic expression {expr}"),
                    Token::BExp(expr) => panic!("Stray boolean expression {expr}"),
                };

                println!("Processed token {:?}", token);
                input = s;
            }

            Err(e) => {
                println!("Failed to process token: {e}");
                break;
            }
        }
    }

    let program = Program::new(builder.end());
    println!("{program}")
}
