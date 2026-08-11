use std::collections::HashSet;
use std::collections::HashMap;
use crate::meta_parser::parser::GrammarRuleSet;
use crate::meta_parser::parser::TokenSet;
use crate::automaton::first::FirstSets;

fn is_token(symbol: String, token_set: TokenSet) -> bool {
    for token_def in token_set.get_defs() {
        if symbol == token_def.get_id().clone() {
            return true;
        }
    }
    return false;
}
fn appears_before_terminal(symbol: &str, rules: &GrammarRuleSet, token_set: &TokenSet) -> Vec<String> {
    let mut results = Vec::new();
    for rule in rules.get_rules() {
        for alternative in rule.get_alternatives() {
            for pair in alternative.windows(2) {
                let cur_sym = &pair[0];
                let next_sym = &pair[1];

                if cur_sym == symbol && is_token(next_sym.to_string(), token_set.clone()) {
                    results.push(next_sym.to_string());
                }
            }
        }
    }
    results
}

fn appears_before_non_terminal(symbol: &str, rules: &GrammarRuleSet, token_set: &TokenSet) -> Vec<String> {
    let mut results = Vec::new();
    for rule in rules.get_rules() {
        for alternative in rule.get_alternatives() {
            for pair in alternative.windows(2) {
                let cur_sym = &pair[0];
                let next_sym = &pair[1];

                if cur_sym == symbol && !is_token(next_sym.to_string(), token_set.clone()) {
                    results.push(next_sym.to_string());
                }
            }
        }
    }
    results
}

fn appears_at_the_end(symbol: &str, rules: &GrammarRuleSet) -> Vec<String> {
    let mut results = Vec::new();
    for rule in rules.get_rules() {
        let lhs = rule.get_lhs();
        for alternative in rule.get_alternatives() {
            if let Some(last_sym) = alternative.last() {
                if last_sym == symbol {
                    results.push(lhs.to_string());
                }
            }
        }
    }
    results
}

pub struct FollowSets {
    sets: HashMap<String, HashSet<String>>
}

impl FollowSets {
    pub fn build(rules: &GrammarRuleSet, token_set: &TokenSet, start_sym: String, first: FirstSets) -> Self {
        let first_sets = first.get_set();
        let mut sets = HashMap::new();
        for rule in rules.get_rules() {
            sets.insert(rule.get_lhs().clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;

            for rule in rules.get_rules() {
                let current_symbol = rule.get_lhs();

                if *current_symbol == start_sym {
                    let current_set = sets.get_mut(current_symbol).unwrap();
                    if current_set.insert(String::from("$")) {
                        changed = true;
                    }
                }

                let terminals = appears_before_terminal(current_symbol, rules, token_set);
                let current_set = sets.get_mut(current_symbol).unwrap();
                for terminal in terminals {
                    if current_set.insert(terminal) {
                        changed = true;
                    }
                }

                let non_terminals = appears_before_non_terminal(current_symbol, rules, token_set);
                for nt in non_terminals {
                    let symbols_to_add: Vec<String> = first_sets.get(&nt)
                        .map(|s| s.iter().cloned().collect())
                        .unwrap_or_default();

                    let current_set = sets.get_mut(current_symbol).unwrap();
                    for sym in symbols_to_add {
                        if current_set.insert(sym) {
                            changed = true;
                        }
                    }
                }

                let parent_lhss = appears_at_the_end(current_symbol, rules);
                for parent_lhs in parent_lhss {
                    if &parent_lhs != current_symbol {
                        let symbols_to_add: Vec<String> = sets.get(&parent_lhs)
                            .map(|s| s.iter().cloned().collect())
                            .unwrap_or_default();

                        let current_set = sets.get_mut(current_symbol).unwrap();
                        for sym in symbols_to_add {
                            if current_set.insert(sym) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        FollowSets { sets }
    }

    pub fn get_set(&self) -> &HashMap<String, HashSet<String>> {
        &self.sets
    }
}
