pub fn insert_regex_dependency() -> String {
    let mut code = String::new();
    code.push_str("use regex::Regex;\n");
    code
}

pub fn insert_oncelock_dependency() -> String {
    let mut code = String::new();
    code.push_str("use std::sync::OnceLock;\n");
    code
}

pub fn insert_hashmap_dependency() -> String {
    let code = String::from("use std::collections::HashMap;\n");
    code
}
