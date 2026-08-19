use crate::meta_parser::parser::ASTNode;
use crate::automaton::symbol::is_token;
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

fn extract_identifiers(ast: &ASTNode) -> HashMap<String, String> {
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

pub struct Symbols {
    identifiers: HashMap<String, String>,
    rule_table: HashMap<(usize, usize), (String, usize)>
}

impl Symbols {
    pub fn new(ast: &ASTNode) -> Self {
        let identifiers = extract_identifiers(ast);
        let mut rule_table = HashMap::new();

        for (rule_idx, rule) in ast.get_rules().get_rules().iter().enumerate() {
            for (alt_idx, alternative) in rule.get_alternatives().iter().enumerate() {
                rule_table.insert((rule_idx, alt_idx), (rule.get_lhs().clone(), alternative.len()));
            }
        }
        Symbols { identifiers, rule_table }
    }

    pub fn generate_token_enum_code(&self, ast: &ASTNode) -> String {
        let mut code = String::from("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str("pub enum Token {\n");

        //Sort the map to guarantee deterministic output order
        let mut sorted_identifiers: Vec<_> = self.identifiers.iter().collect();
        sorted_identifiers.sort_by(|a, b| a.0.cmp(b.0));

        for (id, old_id) in &self.identifiers {
            if is_token(old_id, ast.get_token_set()) {
                let line = format!("\t{}(String),\n", id);
                code.push_str(line.as_str());
            }
        }

        code.push_str("\tEof,\n");
        code.push_str("\tError(String),\n");
        code.push_str("}\n");
        code
    }

    pub fn generate_rule_table_code(&self) -> String {
        let mut code = String::from("pub static RULES: &[(&str, usize)] = &[\n");

        //Collect the entries and sort them by their indices (rule_idx, alt_idx)
        let mut sorted_rules: Vec<_> = self.rule_table.iter().collect();
        sorted_rules.sort_by_key(|(indices, _val)| **indices);

        for (_indices, (lhs, rhs_len)) in sorted_rules {
            let line = format!("\t(\"{}\", {}),\n", lhs, rhs_len);
            code.push_str(line.as_str());
        }

        code.push_str("];\n");
        code
    }
}

