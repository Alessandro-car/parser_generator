use crate::meta_parser::parser::TokenSet;
use std::collections::HashMap;

fn generate_regex_token(token_set: &TokenSet, sanitized_ids: &HashMap<String, String>) -> String {
    let mut code: String = String::new();

    for token in token_set.get_defs() {
        let id = token.get_id();
        let safe_id = sanitized_ids.get(id).unwrap();

        let pattern = format!(
            "static {}_REGEXPR: OnceLock<Regex> = OnceLock::new();
            pub fn get_{}_regex() -> &'static Regex {{
                {}_REGEXPR.get_or_init(|| {{
                    Regex::new(r#\"^{}\"#).expect(\"Invalid regex pattern\")
                }})
            }}
        ",
            safe_id, safe_id, safe_id, token.get_pattern()
        );
        code.push_str(&pattern);
    }
    code
}

fn maximal_munch_scanner(token_set: &TokenSet, sanitized_ids: &HashMap<String, String>) -> String {
    let mut code: String = String::new();
    code.push_str(
    r#"
    pub struct Lexer<'a> {
        input: &'a str,
        position: usize,
        line: usize,
        column: usize,
    }

    impl<'a> Lexer<'a> {
        pub fn new(input: &'a str) -> Self {
            Lexer {
                input,
                position: 0,
                line: 1,
                column: 1,
            }
        }

        pub fn next_token(&mut self) -> Token {
            loop {
                if self.position >= self.input.len() {
                    return Token::Eof;
                }

                let slice = &self.input[self.position..];
                let mut best_len = 0;
                let mut best_token = None;
                let mut is_ignored = false;
    "#);

    for token in token_set.get_defs() {
        let id = token.get_id();
        let safe_id = sanitized_ids.get(id).unwrap();

        let is_skip = token.is_skip();

        let action_assignment = if is_skip {
            "is_ignored = true;".to_string()
        } else {
            format!("best_token = Some(Token::{}(mat.as_str().to_string()));\n
            \t\tis_ignored = false;
            ", safe_id)
        };

        let block = format!(
        r#"
            if let Some(mat) = get_{}_regex().find(slice) {{
                if mat.len() > best_len {{
                    best_len = mat.len();
                    {}
                }}
            }}
        "#,
        safe_id, action_assignment
        );

        code.push_str(&block);
    }

    code.push_str(
    r#"
            if best_len > 0 {
                self.position += best_len;
                let matched_string = &slice[..best_len];
                let newline_count = matched_string.matches('\n').count();
                if newline_count > 0 {
                    self.line += newline_count;
                    let last_newline_idx = matched_string.rfind('\n').unwrap();
                    let after_newline = &matched_string[last_newline_idx + 1..];
                    self.column = after_newline.chars().count() + 1;
                } else {
                    self.column += matched_string.chars().count();
                }
                if is_ignored {
                    continue;
                }
                return best_token.unwrap();
            }

            let bad_char = slice.chars().next().unwrap();
            self.position += bad_char.len_utf8();
            if bad_char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            return Token::Error(format!("Unrecognized character: {} at line {} column {}", bad_char, self.line, self.column));
            }
        }
    }
    "#
    );
    code
}

pub fn emit_lexer(token_set: &TokenSet, sanitized_ids: &HashMap<String, String>) -> String {
    let mut code = String::new();
    code.push_str(generate_regex_token(token_set, sanitized_ids).as_str());
    code.push_str(maximal_munch_scanner(token_set, sanitized_ids).as_str());

    code
}

