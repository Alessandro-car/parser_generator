use std::collections::HashSet;
use std::collections::HashMap;
use crate::meta_parser::parser::TokenSet;
use crate::meta_parser::parser::GrammarRuleSet;

fn is_token(symbol: String, token_set: TokenSet) -> bool {
    for token_def in token_set.get_defs() {
        if symbol == token_def.get_id().clone() {
            return true;
        }
    }
    return false;
}

pub struct FirstSets {
    sets: HashMap<String, HashSet<String>>,
}

impl FirstSets {
    pub fn build(rules: &GrammarRuleSet, token_set: &TokenSet) -> Self {
        let mut sets = HashMap::new();
        for rule in rules.get_rules() {
            sets.insert(rule.get_lhs().clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;

            for rule in rules.get_rules() {
                let lhs = rule.get_lhs();

                for alternative in rule.get_alternatives() {
                    if alternative.is_empty() {
                        continue;
                    }

                    let first_symbol = &alternative[0];
                    if is_token(first_symbol.to_string(), token_set.clone()) {
                        let current_set = sets.get_mut(lhs).unwrap();
                        if current_set.insert(first_symbol.clone()) {
                            changed = true;
                        }
                    } else {
                        let symbols_to_add: Vec<String> = sets.get(first_symbol)
                                .map(|s| s.iter().cloned().collect())
                                .unwrap_or_default();

                        let current_set = sets.get_mut(lhs).unwrap();
                        for sym in symbols_to_add {
                            if current_set.insert(sym) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        FirstSets { sets }
    }
}
