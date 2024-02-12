mod algorithm;
mod analysis;
mod block;
mod expression;
mod functions;
mod parser;
mod program;
mod statement;

use std::{env, io};

// needed by the example program
// use crate::{
//     expression::{AExp::*, BExp::*},
//     program::Program,
//     statement::builder::StatementBuilder,
// };

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Enter statements here! An example program can be found at ./example_program");
    println!("To finish the program, press Ctrl+D");
    println!("To use an input file: Run {} < (input file name)", args[0]);

    let stdin = io::stdin();
    let mut input = String::new();
    let mut buf = String::new();
    while let Ok(count) = stdin.read_line(&mut buf) {
        if count == 0 {
            break;
        }

        input.push_str(&buf);
        buf.clear();
    }

    input = input.trim_end().to_string();
    if input.len() == 0 {
        return;
    }
    input.push(' ');
    let program = parser::parse(&input);

    // parse error -> print location of the error
    if let Err(err) = program {
        println!(
            "\nError parsing program at line {}, column {}:",
            err.location.line, err.location.column
        );

        let line = input.split('\n').take(err.location.line).last().unwrap();
        println!("{line}");
        println!("{:>col$}", "^", col = err.location.column);

        println!("Expected {}", err.expected);

        return;
    }

    // parsed successfully -> proceed with analysis
    let program = program.unwrap();

    println!("Program: {}", program);
    println!("Flow: {:?}", program.flow_r());
    println!();

    // let lva = algorithm::chaotic_iter::run(&program);
    let lva = algorithm::mfp::run(&program);

    for label in 1..=program.len {
        println!(
            "{label}: entry={:?}, exit={:?}",
            lva.entry[&label], lva.exit[&label],
        )
    }
}
