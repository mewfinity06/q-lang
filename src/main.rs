// File attrimutes
#![allow(unused_imports)]

// Rust imports

// Third party imports
use anyhow::{Result, bail};
use load_file::load_str;

// Q-lang imports
use lexer::Lexer;

// Q-lang mods
mod ast;
mod lexer;
mod parser;
mod utils;

fn main() -> Result<()> {
    let args = std::env::args();
    if args.len() < 2 {
        log!(ERROR, "No input file provided.");
        return Ok(());
    }

    let file_path = args.into_iter().nth(1).unwrap();
    let path_absolute = std::path::absolute(&file_path)?;
    let file_contents = load_str!(&path_absolute.to_str().unwrap());

    let lexer = Lexer::new(file_contents);
    let program = match parser::parse(lexer) {
        Ok(p) => p,
        Err((Some((token, span)), message)) => {
            log!(
                ERROR,
                "found {:?} @ lo: {}, hi: {}",
                token,
                span.lo,
                span.hi
            );
            log!(CONTEXT, "{}", message);
            return Ok(());
        }
        Err((None, message)) => {
            log!(ERROR, "{}", message);
            return Ok(());
        }
    };

    log!(DEBUG, "Program: {:#?}", program);

    Ok(())
}
