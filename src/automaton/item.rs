use std::collections::BTreeSet;
use crate::meta_parser::parser::GrammarRuleSet;
use crate::meta_parser::parser::TokenSet;
use crate::automaton::symbol::is_token;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Item {
    rule_idx: usize,
    alt_idx: usize,
    dot: usize
}

impl Item {
    pub fn new(rule_idx: usize, alt_idx: usize, dot: usize) -> Self {
        Item { rule_idx, alt_idx, dot }
    }

    pub fn symbol_at_dot<'a>(&self, rules: &'a GrammarRuleSet) -> Option<&'a String> {
        rules.get_rules()[self.rule_idx]
            .get_alternatives()[self.alt_idx]
            .get(self.dot)
    }

    pub fn is_complete(&self, rules: &GrammarRuleSet) -> bool {
        self.symbol_at_dot(rules).is_none()
    }

    pub fn advanced(&self) -> Item {
        Item::new(self.rule_idx, self.alt_idx, self.dot + 1)
    }
}

pub fn closure(items: BTreeSet<Item>, rules: &GrammarRuleSet, token_set: &TokenSet) -> BTreeSet<Item> {
    let mut result = items;
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot: Vec<Item> = result.iter().cloned().collect();
        for item in snapshot {
            let Some(symbol) = item.symbol_at_dot(rules) else {
                continue;
            };

            if is_token(symbol, token_set) {
                continue;
            }

            for (rule_idx, rule) in rules.get_rules().iter().enumerate() {
                if rule.get_lhs() != symbol {
                    continue;
                }

                for alt_idx in 0..rule.get_alternatives().len() {
                    if result.insert(Item::new(rule_idx, alt_idx, 0)) {
                        changed = true;
                    }
                }
            }
        }
    }
    result
}

pub fn goto(items: &BTreeSet<Item>, rules: &GrammarRuleSet, symbol: &str, token_set: &TokenSet) -> BTreeSet<Item> {
    let mut result = BTreeSet::new();
    for item in items {
        if let Some(sym) = item.symbol_at_dot(rules) {
            if sym == symbol {
                result.insert(item.advanced());
            }
        };
    }

    if result.is_empty() {
        result
    } else {
        closure(result, rules, token_set)
    }
}

pub fn format_item(item: &Item, rules: &GrammarRuleSet) -> String {
    let rule = &rules.get_rules()[item.rule_idx];
    let alt = &rule.get_alternatives()[item.alt_idx];

    let mut parts: Vec<String> = alt.clone();
    parts.insert(item.dot, ".".to_string());

    format!("{} -> {}", rule.get_lhs(), parts.join(" "))
}
