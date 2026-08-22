use crate::automaton::table::ParseTables;
use crate::automaton::table::Action;
use std::collections::HashMap;

fn emit_action_enum() -> String {
    let mut code = String::new();
    code.push_str(r#"
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Action {
            Shift(usize),
            Reduce(usize, usize),
            Accept
        }
    "#);
    code
}

fn generate_action_table_code(action_table: &HashMap<(usize, String), Action>) -> String {
    let mut code = String::new();

    let mut sorted_actions: Vec<_> = action_table.iter().collect();
    sorted_actions.sort_by_key(|(&(state, ref sym), _)| (state, sym.clone()));

    code.push_str("static ACTION_ARRAY: &[((usize, &str), Action)] = &[\n");

    for ((state, sym), action) in sorted_actions {
        let action_str = match action {
            Action::Shift(s) => format!("Action::Shift({})", s),
            Action::Reduce(s, i) => format!("Action::Reduce({}, {})", s, i),
            Action::Accept => "Action::Acccept".to_string()
        };
        let line = format!("\t(({}, \"{}\"), {}),\n", state, sym.as_str(), action_str);
        code.push_str(line.as_str());
    }

    code.push_str("];\n");

    code.push_str(r#"
        static ACTION_TABLE: OnceLock<HashMap<(usize, &'static str), Action>> = OnceLock::new();

        pub fn get_action_table() -> &'static HashMap<(usize, &'static str), Action> {
            ACTION_TABLE.get_or_init(|| {
                let mut map = HashMap::new();
                for (key, action) in ACTION_ARRAY {
                    map.insert(*key, action.clone());
                }
                map
            })
        }
    "#);
    code
}

fn generate_goto_table_code(goto_table: &HashMap<(usize, String), usize>) -> String {
    let mut code = String::new();

    let mut sorted_gotos: Vec<_> = goto_table.iter().collect();
    sorted_gotos.sort_by_key(|(&(state, ref sym), _)| (state, sym.clone()));

    code.push_str("static GOTO_ARRAY: &[((usize, &str), usize)] = &[\n");

    for ((state, sym), next_state) in sorted_gotos {
        let line = format!("\t(({}, \"{}\"), {}),\n", state, sym.as_str(), next_state);
        code.push_str(line.as_str());
    }

    code.push_str("];\n");

    code.push_str(r#"
        static GOTO_TABLE: OnceLock<HashMap<(usize, &'static str), usize>> = OnceLock::new();

        pub fn get_goto_table() -> &'static HashMap<(usize, &'static str), usize> {
            GOTO_TABLE.get_or_init(|| {
                let mut map = HashMap::new();
                for (key, next_state) in GOTO_ARRAY {
                    map.insert(*key, *next_state);
                }
                map
            })
        }
    "#);
    code
}

pub fn generate_tables(parse_tables: &ParseTables) -> String {
    let mut code = String::new();
    code.push_str(emit_action_enum().as_str());
    code.push_str(generate_action_table_code(parse_tables.get_action_table()).as_str());
    code.push_str(generate_goto_table_code(parse_tables.get_goto_table()).as_str());
    code

}
