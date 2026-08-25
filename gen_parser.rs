use regex::Regex;
use std::sync::OnceLock;
use std::collections::HashMap;
use crate::ast::{Expression, Operator};
use std::num::ParseIntError;
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    DIV(String),
    LPAREN(String),
    MINUS(String),
    MUL(String),
    NUMBER(String),
    PLUS(String),
    RPAREN(String),
    WS(String),
    Eof,
    Error(String),
}
impl Token {
    pub fn name(&self) -> &'static str {
        match self {
            Token::DIV(_) => "DIV",
            Token::LPAREN(_) => "LPAREN",
            Token::MINUS(_) => "MINUS",
            Token::MUL(_) => "MUL",
            Token::NUMBER(_) => "NUMBER",
            Token::PLUS(_) => "PLUS",
            Token::RPAREN(_) => "RPAREN",
            Token::WS(_) => "WS",
            Token::Eof => "Eof",
            Token::Error => "Error",
        }
    }
}
pub static RULES: &[(&str, usize)] = &[
    ("Program", 1),
    ("Expr", 3),
    ("Expr", 3),
    ("Expr", 1),
    ("Term", 3),
    ("Term", 3),
    ("Term", 1),
    ("Factor", 3),
    ("Factor", 1),
];
static NUMBER_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_NUMBER_regex() -> &'static Regex {
    NUMBER_REGEXPR
        .get_or_init(|| { Regex::new(r"^[0-9]+").expect("Invalid regex pattern") })
}
static PLUS_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_PLUS_regex() -> &'static Regex {
    PLUS_REGEXPR.get_or_init(|| { Regex::new(r"^\+").expect("Invalid regex pattern") })
}
static MINUS_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_MINUS_regex() -> &'static Regex {
    MINUS_REGEXPR.get_or_init(|| { Regex::new(r"^\-").expect("Invalid regex pattern") })
}
static MUL_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_MUL_regex() -> &'static Regex {
    MUL_REGEXPR.get_or_init(|| { Regex::new(r"^\*").expect("Invalid regex pattern") })
}
static DIV_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_DIV_regex() -> &'static Regex {
    DIV_REGEXPR.get_or_init(|| { Regex::new(r"^/").expect("Invalid regex pattern") })
}
static LPAREN_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_LPAREN_regex() -> &'static Regex {
    LPAREN_REGEXPR.get_or_init(|| { Regex::new(r"^\(").expect("Invalid regex pattern") })
}
static RPAREN_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_RPAREN_regex() -> &'static Regex {
    RPAREN_REGEXPR.get_or_init(|| { Regex::new(r"^\)").expect("Invalid regex pattern") })
}
static WS_REGEXPR: OnceLock<Regex> = OnceLock::new();
pub fn get_WS_regex() -> &'static Regex {
    WS_REGEXPR
        .get_or_init(|| { Regex::new(r"^[ \t\r\n]+").expect("Invalid regex pattern") })
}
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }
    pub fn next_token(&mut self) -> Token {
        loop {
            if self.position >= self.input.len() {
                return Token::Eof;
            }
            let slice = &self.input[self.position..];
            let mut best_len = 0;
            let mut best_token = None;
            let mut is_ignored = false;
            if let Some(mat) = get_NUMBER_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    best_token = Some(Token::NUMBER(mat.as_str().to_string()));
                    is_ignored = false;
                }
            }
            if let Some(mat) = get_PLUS_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    best_token = Some(Token::PLUS(mat.as_str().to_string()));
                    is_ignored = false;
                }
            }
            if let Some(mat) = get_MINUS_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    best_token = Some(Token::MINUS(mat.as_str().to_string()));
                    is_ignored = false;
                }
            }
            if let Some(mat) = get_MUL_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    best_token = Some(Token::MUL(mat.as_str().to_string()));
                    is_ignored = false;
                }
            }
            if let Some(mat) = get_DIV_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    best_token = Some(Token::DIV(mat.as_str().to_string()));
                    is_ignored = false;
                }
            }
            if let Some(mat) = get_LPAREN_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    best_token = Some(Token::LPAREN(mat.as_str().to_string()));
                    is_ignored = false;
                }
            }
            if let Some(mat) = get_RPAREN_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    best_token = Some(Token::RPAREN(mat.as_str().to_string()));
                    is_ignored = false;
                }
            }
            if let Some(mat) = get_WS_regex().find(slice) {
                if mat.len() > best_len {
                    best_len = mat.len();
                    is_ignored = true;
                }
            }
            if best_len > 0 {
                self.position += best_len;
                let matched_string = &slice[..best_len];
                let newline_count = matched_string.matches('\n').count();
                if newline_count > 0 {
                    self.line += newline_count;
                    let last_newline_idx = matched_string.rfind('\n').unwrap();
                    let after_newline = &matched_string[last_newline_idx + 1..];
                    self.column = after_newline.chars().count() + 1;
                } else {
                    self.column += matched_string.chars().count();
                }
                if is_ignored {
                    continue;
                }
                return best_token.unwrap();
            }
            let bad_char = slice.chars().next().unwrap();
            self.position += bad_char.len_utf8();
            if bad_char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            return Token::Error(
                format!(
                    "Unrecognized character: {} at line {} column {}", bad_char, self
                    .line, self.column
                ),
            );
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Shift(usize),
    Reduce(usize, usize),
    Accept,
}
static ACTION_ARRAY: &[((usize, &str), Action)] = &[
    ((0, "LPAREN"), Action::Shift(3)),
    ((0, "NUMBER"), Action::Shift(4)),
    ((1, "$"), Action::Reduce(0, 0)),
    ((1, "MINUS"), Action::Shift(7)),
    ((1, "PLUS"), Action::Shift(8)),
    ((2, "$"), Action::Reduce(2, 2)),
    ((2, "DIV"), Action::Reduce(2, 2)),
    ((2, "MINUS"), Action::Reduce(2, 2)),
    ((2, "MUL"), Action::Reduce(2, 2)),
    ((2, "PLUS"), Action::Reduce(2, 2)),
    ((2, "RPAREN"), Action::Reduce(2, 2)),
    ((3, "LPAREN"), Action::Shift(3)),
    ((3, "NUMBER"), Action::Shift(4)),
    ((4, "$"), Action::Reduce(3, 1)),
    ((4, "DIV"), Action::Reduce(3, 1)),
    ((4, "MINUS"), Action::Reduce(3, 1)),
    ((4, "MUL"), Action::Reduce(3, 1)),
    ((4, "PLUS"), Action::Reduce(3, 1)),
    ((4, "RPAREN"), Action::Reduce(3, 1)),
    ((5, "$"), Action::Accept),
    ((6, "$"), Action::Reduce(1, 2)),
    ((6, "DIV"), Action::Shift(10)),
    ((6, "MINUS"), Action::Reduce(1, 2)),
    ((6, "MUL"), Action::Shift(11)),
    ((6, "PLUS"), Action::Reduce(1, 2)),
    ((6, "RPAREN"), Action::Reduce(1, 2)),
    ((7, "LPAREN"), Action::Shift(3)),
    ((7, "NUMBER"), Action::Shift(4)),
    ((8, "LPAREN"), Action::Shift(3)),
    ((8, "NUMBER"), Action::Shift(4)),
    ((9, "MINUS"), Action::Shift(7)),
    ((9, "PLUS"), Action::Shift(8)),
    ((9, "RPAREN"), Action::Shift(14)),
    ((10, "LPAREN"), Action::Shift(3)),
    ((10, "NUMBER"), Action::Shift(4)),
    ((11, "LPAREN"), Action::Shift(3)),
    ((11, "NUMBER"), Action::Shift(4)),
    ((12, "$"), Action::Reduce(1, 1)),
    ((12, "DIV"), Action::Shift(10)),
    ((12, "MINUS"), Action::Reduce(1, 1)),
    ((12, "MUL"), Action::Shift(11)),
    ((12, "PLUS"), Action::Reduce(1, 1)),
    ((12, "RPAREN"), Action::Reduce(1, 1)),
    ((13, "$"), Action::Reduce(1, 0)),
    ((13, "DIV"), Action::Shift(10)),
    ((13, "MINUS"), Action::Reduce(1, 0)),
    ((13, "MUL"), Action::Shift(11)),
    ((13, "PLUS"), Action::Reduce(1, 0)),
    ((13, "RPAREN"), Action::Reduce(1, 0)),
    ((14, "$"), Action::Reduce(3, 0)),
    ((14, "DIV"), Action::Reduce(3, 0)),
    ((14, "MINUS"), Action::Reduce(3, 0)),
    ((14, "MUL"), Action::Reduce(3, 0)),
    ((14, "PLUS"), Action::Reduce(3, 0)),
    ((14, "RPAREN"), Action::Reduce(3, 0)),
    ((15, "$"), Action::Reduce(2, 1)),
    ((15, "DIV"), Action::Reduce(2, 1)),
    ((15, "MINUS"), Action::Reduce(2, 1)),
    ((15, "MUL"), Action::Reduce(2, 1)),
    ((15, "PLUS"), Action::Reduce(2, 1)),
    ((15, "RPAREN"), Action::Reduce(2, 1)),
    ((16, "$"), Action::Reduce(2, 0)),
    ((16, "DIV"), Action::Reduce(2, 0)),
    ((16, "MINUS"), Action::Reduce(2, 0)),
    ((16, "MUL"), Action::Reduce(2, 0)),
    ((16, "PLUS"), Action::Reduce(2, 0)),
    ((16, "RPAREN"), Action::Reduce(2, 0)),
];
static ACTION_TABLE: OnceLock<HashMap<(usize, &'static str), Action>> = OnceLock::new();
pub fn get_action_table() -> &'static HashMap<(usize, &'static str), Action> {
    ACTION_TABLE
        .get_or_init(|| {
            let mut map = HashMap::new();
            for (key, action) in ACTION_ARRAY {
                map.insert(*key, action.clone());
            }
            map
        })
}
static GOTO_ARRAY: &[((usize, &str), usize)] = &[
    ((0, "Expr"), 1),
    ((0, "Factor"), 2),
    ((0, "Program"), 5),
    ((0, "Term"), 6),
    ((3, "Expr"), 9),
    ((3, "Factor"), 2),
    ((3, "Term"), 6),
    ((7, "Factor"), 2),
    ((7, "Term"), 12),
    ((8, "Factor"), 2),
    ((8, "Term"), 13),
    ((10, "Factor"), 15),
    ((11, "Factor"), 16),
];
static GOTO_TABLE: OnceLock<HashMap<(usize, &'static str), usize>> = OnceLock::new();
pub fn get_goto_table() -> &'static HashMap<(usize, &'static str), usize> {
    GOTO_TABLE
        .get_or_init(|| {
            let mut map = HashMap::new();
            for (key, next_state) in GOTO_ARRAY {
                map.insert(*key, *next_state);
            }
            map
        })
}
#[derive(Debug, Clone)]
pub enum ParseTree {
    Leaf(Token),
    Node(&'static str, Vec<ParseTree>),
}
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    state_stack: Vec<usize>,
    value_stack: Vec<ParseTree>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            state_stack: vec![0],
            value_stack: Vec::new(),
        }
    }
    pub fn parse(&mut self) -> Result<ParseTree, String> {
        let mut cur_token = self.lexer.next_token();
        loop {
            let cur_state = *self.state_stack.last().unwrap();
            let sym_name = match &cur_token {
                Token::Eof => "Eof",
                Token::Error(msg) => return Err(format!("Lexer error: {}", msg)),
                t => t.name(),
            };
            let action = get_action_table().get(&(cur_state, sym_name));
            match action {
                Some(Action::Shift(next_state)) => {
                    self.state_stack.push(*next_state);
                    self.value_stack.push(ParseTree::Leaf(cur_token.clone()));
                    cur_token = self.lexer.next_token();
                }
                Some(Action::Reduce(rule_idx, _)) => {
                    let (lhs, rhs_len) = RULES[*rule_idx];
                    let mut children = Vec::new();
                    for _ in 0..rhs_len {
                        children.insert(0, self.value_stack.pop().unwrap());
                    }
                    let top_state = self.state_stack.last().unwrap();
                    let next_state = get_goto_table()
                        .get(*(top_state, lhs))
                        .unwrap_or_else(|| {
                            panic!(
                                "GOTO table error: no transition for state {} on {}",
                                top_state, lhs
                            )
                        });
                    self.state_stack.push(*next_state);
                    self.value_stack.push(ParseTree::Node(lhs, children));
                }
                Some(Action::Accept) => {
                    return Ok(self.value_stack.pop().unwrap());
                }
                None => {
                    let mut expected_symbols: Vec<&str> = get_action_table()
                        .keys()
                        .filter(|(state, _)| *state == cur_state)
                        .map(|(_, sym)| *sym)
                        .collect();
                    expected_symbols.sort();
                    let expected_str = expected_symbols.join(", ");
                    return Err(
                        format!(
                            "Syntax Error: unexpected token {:?} at state {}. Expected one of: {}",
                            cur_token, cur_state, expected_str
                        ),
                    );
                }
            }
        }
    }
}
pub fn report_error(line: usize, msg: &str) {
    eprintln!("Parse Error [Line {}]: {}", line, msg);
}
fn main() {
    println!("Calculator parser generated successfully!");
}
