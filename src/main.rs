mod cli;
mod error;
mod lexer;
mod parser;

use std::path::Path;

use clap::{CommandFactory, Parser, error::ErrorKind};
use cli::Args;

use crate::lexer::Lexer;

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

    // TODO: everything below this comment is VERY temporary
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => String::from("")
    };

    let mut l = Lexer::new(&source);
    let tokens = l.tokenize();
    let mut p = parser::Parser::new(tokens.unwrap());

    println!("{:#?}", p.parse());
}
