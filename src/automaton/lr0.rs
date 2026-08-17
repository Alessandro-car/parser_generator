use crate::automaton::item::{closure, goto, Item};
use crate::meta_parser::parser::GrammarRuleSet;
use crate::meta_parser::parser::TokenSet;
use std::collections::BTreeSet;
use std::collections::HashMap;

pub struct LR0Automaton {
    states: Vec<BTreeSet<Item>>,
    transitions: HashMap<(usize, String), usize>
}

impl LR0Automaton {
    pub fn build(augmented_rules: &GrammarRuleSet, start_idx: usize, token_set: &TokenSet) -> Self {
        let mut states = Vec::new();
        let mut transitions = HashMap::new();
        let mut initial = BTreeSet::new();
        initial.insert(Item::new(start_idx, 0, 0));
        states.push(closure(initial, augmented_rules, token_set));
        let mut state_idx = 0;
        while state_idx < states.len() {
            let symbols: BTreeSet<String> = states[state_idx]
                .iter()
                .filter_map(|item| item.symbol_at_dot(augmented_rules).cloned())
                .collect();

            for symbol in symbols {
                let goto_set = goto(&states[state_idx], augmented_rules, &symbol, token_set);
                if goto_set.is_empty() {
                    continue;
                }

                let target_idx = match states.iter().position(|s| s == &goto_set) {
                    Some(idx) => idx,
                    None => {
                        states.push(goto_set);
                        states.len() - 1
                    }
                };

                transitions.insert((state_idx, symbol), target_idx);
            }
            state_idx += 1;
        }
        LR0Automaton { states, transitions }
    }

    pub fn get_states(&self) -> &Vec<BTreeSet<Item>> {
        &self.states
    }

    pub fn get_transitions(&self) -> &HashMap<(usize, String), usize> {
        &self.transitions
    }
}
