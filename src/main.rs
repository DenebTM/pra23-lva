mod algorithm;
mod analysis;
mod block;
mod expression;
mod functions;
mod parser;
mod program;
mod statement;

use rustyline::{config::Configurer, DefaultEditor};
use std::{
    env,
    io::{self, IsTerminal},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let is_terminal = io::stdin().is_terminal();
    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);

    if is_terminal {
        println!("Enter statements here! Examples can be found in ./example_program.");
        println!("To finish the program, press Ctrl+D or submit a blank line.");
        println!("To use an input file, run: {} < (path/to/file)", args[0]);
        println!("To exit, press Ctrl+C or submit a blank program.")
    }

    loop {
        let mut rl_prompt = ">>> ";

        let mut input = String::new();
        while let Ok(line) = rl.readline(rl_prompt) {
            if line.len() == 0 && is_terminal {
                break;
            }

            input.push_str(&line);
            input.push('\n');
            rl_prompt = "... ";
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

            continue;
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

        println!();
    }
}
