extern crate syn;
extern crate prettyplease;

mod meta_parser;
mod automaton;
mod emitter;
use crate::meta_parser::lexer::Lexer;
use crate::meta_parser::lexer::TokenType;
use crate::meta_parser::parser::Parser;
use std::env;
use std::io::Result;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).cloned().unwrap_or_default();
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let lex = Lexer::new(contents);
    let mut parser = Parser::new(lex).expect("Failed to construct parser");
    let ast = parser.parse();
    let generated_code = emitter::generate(&ast);
    let file_name = "gen_parser.rs";
    let _file = fs::File::create(file_name)?;
    fs::write(file_name, generated_code)?;
    Ok(())
}
