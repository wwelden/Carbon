use clap::{Arg, Command};
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;


mod ast;
mod evaluator;
mod lexer;
mod parser;
mod value;
mod error;

use crate::evaluator::{Context, Evaluator};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;
use crate::error::CarbonError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("carbon")
        .version("0.1.0")
        .about("Carbon Programming Language Interpreter")
        .arg(
            Arg::new("file")
                .help("Carbon source file to execute")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Start interactive mode after executing file")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(file_path) = matches.get_one::<String>("file") {
        // Execute file
        let source = std::fs::read_to_string(file_path)?;
        let mut context = Context::new();

        if let Err(e) = execute_source(&source, &mut context) {
            eprintln!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }

        if matches.get_flag("interactive") {
            start_repl(context)?;
        }
    } else {
        // Start REPL
        let context = Context::new();
        start_repl(context)?;
    }

    Ok(())
}

fn execute_source(source: &str, context: &mut Context) -> Result<(), CarbonError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut evaluator = Evaluator::new();

    for statement in statements {
        match evaluator.eval_statement(context, statement)? {
            Some(Value::Print(output)) => {
                println!("{}", output);
            }
            Some(value) if !matches!(value, Value::Null) => {
                println!("{}", value.to_string());
            }
            _ => {}
        }
    }

    Ok(())
}

fn start_repl(mut context: Context) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Carbon Programming Language Interpreter".bright_blue());
    println!("Type '{}' to exit", "quit".yellow());

    let mut rl = Editor::<(), _>::new()?;

    // Load history if it exists
    let _ = rl.load_history(".carbon_history");

    loop {
        match rl.readline(&format!("{} ", ">".bright_green())) {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                if line == "quit" || line == "exit" {
                    println!("Goodbye! 👋");
                    break;
                }

                let _ = rl.add_history_entry(line);

                match execute_line(line, &mut context) {
                    Ok(Some(Value::Print(output))) => {
                        println!("{}", output);
                    }
                    Ok(Some(value)) if !matches!(value, Value::Null) => {
                        println!("{}", value.to_string().bright_cyan());
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}: {}", "Error".red(), e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C pressed, exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D pressed, exiting...");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    let _ = rl.save_history(".carbon_history");

    Ok(())
}

fn execute_line(line: &str, context: &mut Context) -> Result<Option<Value>, CarbonError> {
    let mut lexer = Lexer::new(line);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut evaluator = Evaluator::new();
    let mut last_value = None;

    for statement in statements {
        last_value = evaluator.eval_statement(context, statement)?;
    }

    Ok(last_value)
}
