#![allow(unused_imports)]
use load_file::load_str;

use lexer::{self, Lexer, token::Token};
use utils::{self, log};

fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    if args.len() < 2 {
        log!(ERROR, "No input file provided.");
        return Ok(());
    }

    let file_path = args.into_iter().nth(1).unwrap();
    let path_absolute = std::path::absolute(&file_path)?;
    let file_contents = load_str!(&path_absolute.to_str().unwrap());

    let lexer = Lexer::new(&file_path, &file_contents);

    for token in lexer {
        if token == Token::EOF {
            break;
        }
        log!(DEBUG, "{:?}", token);
    }

    Ok(())
}
