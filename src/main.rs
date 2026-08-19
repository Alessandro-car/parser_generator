mod meta_parser;
mod automaton;
mod emitter;
use crate::automaton::first::FirstSets;
use crate::automaton::follow::FollowSets;
use crate::meta_parser::lexer::Lexer;
use crate::meta_parser::lexer::TokenType;
use crate::meta_parser::parser::Parser;
use crate::automaton::augment::augment;
use crate::automaton::lr0::LR0Automaton;
use crate::automaton::table::ParseTables;
use crate::emitter::symbol_gen::extract_identifiers;
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

    let (augmented_rules, start_idx) = augment(ast.get_rules(), ast.get_start_sym());
    let lr0 = LR0Automaton::build(&augmented_rules, start_idx, ast.get_token_set());
    println!("{:#?}", ParseTables::build(&lr0, &augmented_rules, ast.get_token_set(), &follow_set, start_idx));
    println!("{:#?}", extract_identifiers(ast));
    Ok(())
}
