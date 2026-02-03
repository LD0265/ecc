mod cli;
mod error;
mod lexer;
mod parser;
mod mips;
mod compiler;

use std::path::Path;

use clap::{CommandFactory, Parser, error::ErrorKind};
use cli::Args;

use crate::compiler::Compiler;

fn main() {
    let args = Args::parse();

    let path = Path::new(args.input_file.as_str());

    if !path.exists() {
        Args::command()
                .error(ErrorKind::ArgumentConflict, format!("{:?} is not a valid path", path))
                .exit();
    }

    if !path.is_file() {
        Args::command()
                .error(ErrorKind::ArgumentConflict, format!("{:?} is not a file", path))
                .exit();
    }

    if let Some(extension) = path.extension() {
        if extension != "ec" {
            // I really don't like using unwrap, but this is okay for now
            let file_name = path.file_name().unwrap();

            Args::command()
                .error(ErrorKind::ArgumentConflict, format!("{} must be an ec file", file_name.display()))
                .exit();
        }
    }

    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => String::from("")
    };

    let mut compiler = Compiler::new(&source);

    let output = args.output.as_str();

    match compiler.compile(output) {
        Ok(_) => {
            println!("Compilation successful!");
        }

        Err(e) => {
            eprintln!("Error compiling: {}", e);
        }
    }

    if args.ast {
        match compiler.get_ast() {
            Ok(ast) => {
                println!("{:#?}", ast);
            }

            Err(e) => {
                eprintln!("Error generating AST: {}", e);
            }
        }
    }

    if args.tokens {
        match compiler.get_tokens() {
            Ok(tokens) => {
                println!("{:#?}", tokens);
            }

            Err(e) => {
                eprintln!("Error generating tokens: {}", e);
            }
        }
    }
}
