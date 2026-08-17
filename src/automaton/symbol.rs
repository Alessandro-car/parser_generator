use crate::meta_parser::parser::TokenSet;

pub fn is_token(symbol: &str, token_set: &TokenSet) -> bool {
    for token_def in token_set.get_defs() {
        if symbol == token_def.get_id().clone() {
            return true;
        }
    }
    return false;
}
