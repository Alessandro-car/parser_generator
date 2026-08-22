use crate::automaton::symbol::is_token;
use crate::automaton::lr0::LR0Automaton;
use crate::automaton::follow::FollowSets;
use crate::meta_parser::parser::GrammarRuleSet;
use crate::meta_parser::parser::TokenSet;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Shift(usize),
    Reduce(usize, usize),
    Accept
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTables {
    action: HashMap<(usize, String), Action>,
    goto: HashMap<(usize, String), usize>
}

impl ParseTables {
    pub fn build(
        automaton: &LR0Automaton,
        rules: &GrammarRuleSet,
        token_set: &TokenSet,
        follow_set: &FollowSets,
        augumented_start_idx: usize
    ) -> Result<Self, Vec<String>> {
        let mut action = HashMap::new();
        let mut goto = HashMap::new();
        let mut conflicts = Vec::new();

        for ((state, symbol), &target) in automaton.get_transitions() {
            if is_token(symbol, token_set) {
                insert_action(&mut action, &mut conflicts, *state, symbol.clone(), Action::Shift(target));
            } else {
                goto.insert((*state, symbol.clone()), target);
            }
        }

        for (state, item_set) in automaton.get_states().iter().enumerate() {
            for item in item_set {
                if !item.is_complete(rules) {
                    continue;
                }

                if item.get_rule_idx() == augumented_start_idx {
                    insert_action(&mut action, &mut conflicts, state, "$".to_string(), Action::Accept);
                    continue;
                }

                let lhs = rules.get_rules()[item.get_rule_idx()].get_lhs();
                let follow = follow_set.get_set().get(lhs).cloned().unwrap_or_default();

                for terminal in follow {
                    insert_action(&mut action, &mut conflicts, state, terminal, Action::Reduce(item.get_rule_idx(), item.get_alt_idx()));
                }
            }
        }

        if conflicts.is_empty() {
            Ok(ParseTables { action, goto })
        } else {
            Err(conflicts)
        }
    }

    pub fn get_action_table(&self) -> &HashMap<(usize, String), Action> {
        &self.action
    }

    pub fn get_goto_table(&self) -> &HashMap<(usize, String), usize> {
        &self.goto
    }

    pub fn get_action(&self, state: usize, terminal: &str) -> Option<&Action> {
        self.action.get(&(state, terminal.to_string()))
    }

    pub fn get_goto(&self, state: usize, nonterminal: &str) -> Option<&usize> {
        self.goto.get(&(state, nonterminal.to_string()))
    }
}

fn insert_action(
    action: &mut HashMap<(usize, String), Action>,
    conflicts: &mut Vec<String>,
    state: usize,
    terminal: String,
    new_action: Action
) {
    match action.get(&(state, terminal.clone())) {
        Some(existing) if *existing != new_action => {
            conflicts.push(format!(
                "Conflict in state {} on '{}': {:?} vs {:?}",
                state, terminal, existing, new_action
            ));
        }
        Some(_) => {}
        None => {
            action.insert((state, terminal), new_action);
        }
    }
}

