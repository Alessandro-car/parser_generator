pub fn generate_driver_code() -> String {
    let mut code = String::new();
    code.push_str(r#"
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
                        Token::Eof => "$",
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
                        Some(Action::Reduce(rule_idx, alt_idx)) => {
                            let (lhs, rhs_len) = RULES[*rule_idx][*alt_idx];
                            let mut children = Vec::new();
                            for _ in 0..rhs_len {
                                self.state_stack.pop().unwrap();
                                children.insert(0, self.value_stack.pop().unwrap());
                            }

                            let top_state = self.state_stack.last().unwrap();
                            let next_state = get_goto_table()
                                .get(&(*top_state, lhs))
                                .unwrap_or_else(|| panic!("GOTO table error: no transition for state {} on {}", top_state, lhs));

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
                            return Err(format!(
                                "Syntax Error: unexpected token {:?} at state {}. Expected one of: {}",
                                cur_token, cur_state, expected_str
                            ));
                        }
                    }
                }
            }
        }
    "#);
    code
}
