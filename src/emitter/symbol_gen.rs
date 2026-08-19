use crate::meta_parser::parser::ASTNode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::OnceLock;

fn is_keyword(word: &str) -> bool {
    static KEYWORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();

    let keywords = KEYWORDS.get_or_init(|| {
        HashSet::from([
            "as", "break", "const", "continue", "crate", "else", "enum", "extern",
            "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
            "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
            "super", "trait", "true", "type", "unsafe", "use", "where", "while",

            "async", "await", "dyn",

            "abstract", "become", "box", "do", "final", "macro", "override",
            "priv", "typeof", "unsized", "virtual", "yield", "try",
        ])
    });

    keywords.contains(word)
}

fn convert_rust_keyword(word: &str) -> String {
    format!("r#{}", word)
}

//TODO: Convert dash into underscore or convert it into camel case to avoid rust warnings
fn convert_dash(word: &str) -> String {
    let converted: String = word
        .chars()
        .map(|c| if c == '-' { '_'} else { c })
        .collect();

    converted
}

fn check_for_typos(identifiers: &[String], ast: &ASTNode) -> Result<(), String> {
    //Create a HashSet of string slices for O(1) lookups without cloning data
    let id_set: HashSet<&str> = identifiers.iter().map(|s| s.as_str()).collect();

    for rule in ast.get_rules().get_rules() {
        for alternative in rule.get_alternatives() {
            for id in alternative {
                if !id_set.contains(id.as_str()) {
                    return Err(format!("Undefined symbol '{}' used in rule '{}'", id, rule.get_lhs()));
                }
            }
        }
    }

    Ok(())
}

pub fn extract_identifiers(ast: ASTNode) -> HashMap<String, String> {
    let mut identifiers: Vec<String> = Vec::from(
        ast.get_token_set()
        .get_defs()
        .iter()
        .map(|token| token.get_id().clone())
        .collect::<Vec<_>>()
    );

    for rule in ast.get_rules().get_rules() {
        identifiers.push(rule.get_lhs().clone());
    }
    let _ = check_for_typos(&identifiers, &ast);
    let mut sanitized_ids: HashMap<String, String> = HashMap::new();
    for id in identifiers {
        let mut converted = if is_keyword(&id) {
            convert_rust_keyword(&id)
        } else {
            id.clone()
        };
        converted = convert_dash(&converted);

        if converted.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            converted.insert(0, '_');
        }

        if let Some(previous_original) = sanitized_ids.get(&converted) {
            eprintln!(
                "ERROR: Collision detected! Both '{}' and '{}' resolve to the same Rust identifier: '{}'",
                previous_original, id, converted
            );
        } else {
            sanitized_ids.insert(converted, id);
        }
    }
    sanitized_ids
}





