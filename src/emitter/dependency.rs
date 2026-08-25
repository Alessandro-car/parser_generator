fn insert_regex_dependency() -> String {
    let mut code = String::new();
    code.push_str("use regex::Regex;\n");
    code
}

fn insert_oncelock_dependency() -> String {
    let mut code = String::new();
    code.push_str("use std::sync::OnceLock;\n");
    code
}

fn insert_hashmap_dependency() -> String {
    let code = String::from("use std::collections::HashMap;\n");
    code
}

pub fn insert_dependency() -> String {
    let mut code = String::new();
    code.push_str(insert_regex_dependency().as_str());
    code.push_str(insert_oncelock_dependency().as_str());
    code.push_str(insert_hashmap_dependency().as_str());

    code
}
