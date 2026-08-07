mod meta_parser;
use crate::meta_parser::lexer::Lexer;

use std::env;
use std::io::Result;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).cloned().unwrap_or_default();

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let char_vec: Vec<char> = contents.chars().collect();
    let lex = Lexer::new(char_vec);
    Ok(())
}
