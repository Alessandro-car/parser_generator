mod meta_parser;
mod automaton;
use crate::automaton::first::FirstSets;
use crate::automaton::follow::FollowSets;
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
    let first_set = FirstSets::build(ast.get_rules(), ast.get_token_set());
    let follow_set = FollowSets::build(ast.get_rules(), ast.get_token_set(), ast.get_start_sym().clone(), first_set);
    Ok(())
}
