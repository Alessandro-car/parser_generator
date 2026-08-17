use crate::meta_parser::parser::GrammarRuleSet;
use crate::meta_parser::parser::GrammarRuleDef;
pub fn augment(rules: &GrammarRuleSet, start: &str) -> (GrammarRuleSet, usize) {
    let augment_start_idx = rules.get_rules().len();
    let new_start = GrammarRuleDef::new(
        format!("{}'", start),
        vec![vec![start.to_string()]],
    );
    let augmented = rules.clone().with_appended_rule(new_start);
    (augmented, augment_start_idx)
}
