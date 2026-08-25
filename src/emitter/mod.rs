pub mod symbol_gen;
pub mod lexer_gen;
pub mod dependency;
pub mod tables_gen;
pub mod driver_gen;

use crate::meta_parser::parser::ASTNode;
use crate::automaton::table::ParseTables;
use crate::automaton::first::FirstSets;
use crate::automaton::follow::FollowSets;
use crate::automaton::augment::augment;
use crate::automaton::lr0::LR0Automaton;

use crate::emitter::dependency::insert_dependency;
use crate::emitter::symbol_gen::Symbols;
use crate::emitter::lexer_gen::emit_lexer;
use crate::emitter::tables_gen::generate_tables;
use crate::emitter::driver_gen::generate_driver_code;

fn format_generated_code(code: &str) -> Result<String, syn::Error> {
    let ast: syn::File = syn::parse_file(code)?;
    let formatted_code = prettyplease::unparse(&ast);

    Ok(formatted_code)
}

pub fn generate(ast: &ASTNode) -> String {
    let first_set = FirstSets::build(ast.get_rules(), ast.get_token_set());
    let follow_set = FollowSets::build(ast.get_rules(), ast.get_token_set(), ast.get_start_sym().clone(), first_set);

    let (augmented_rules, start_idx) = augment(ast.get_rules(), ast.get_start_sym());
    let lr0 = LR0Automaton::build(&augmented_rules, start_idx, ast.get_token_set());
    let parse_tables = match ParseTables::build(&lr0, &augmented_rules, ast.get_token_set(), &follow_set, start_idx) {
        Ok(tables) => tables,
        Err(conflicts) => {
            eprintln!("Error: Could not build parse tables due to grammar conflicts:");
            for conflict in conflicts {
                eprintln!("- {}", conflict);
            }
            std::process::exit(1);
        }
    };

    let mut code = String::new();

    code.push_str(&insert_dependency());
    code.push_str("\n\n");

    let prologue = ast.get_prologue()
        .as_ref()
        .map(|p| p.get_prologue().clone())
        .unwrap_or_default();
    code.push_str(&prologue);
    code.push_str("\n\n");

    let symbols = Symbols::new(ast);
    code.push_str(&symbols.generate_token_enum_code(ast));
    code.push_str("\n\n");

    code.push_str(&symbols.generate_rule_table_code());
    code.push_str("\n\n");

    code.push_str(&emit_lexer(ast.get_token_set(), symbols.get_sanitized_ids()));
    code.push_str("\n\n");

    code.push_str(&generate_tables(&parse_tables));
    code.push_str("\n\n");

    code.push_str(&generate_driver_code());
    code.push_str("\n\n");

    let epilogue = ast.get_epilogue()
        .as_ref()
        .map(|p| p.get_epilogue().clone())
        .unwrap_or_default();
    code.push_str(&epilogue);
    code.push_str("\n\n");

    match format_generated_code(code.as_str()) {
        Ok(formatted) => formatted,
        Err(e) => {
            eprintln!("Failed to parse/format generated code: {}", e);
            eprintln!("Returning raw unformatted code so you can debug the syntax error.");
            code
        }
    }
}
