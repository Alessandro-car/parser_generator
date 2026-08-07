static KEYWORDS: &[&str] = &[
    "prologue", "epilogue",
    "tokens", "start", "rules"
];

static OPERATORS: &[&str] = &[
    "=", "|", "->"
];


#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Eof,
    Keyword(String),
    Identifier(String),
    Rbrace(char),
    Lbrace(char),
    CodeBlockStart,
    RawCode(String),
    Operator(String),
    StringLiteral(String),
    Semicolon(char),
    Error(String)
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    start_raw_code: bool
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Lexer {
            input: content.chars().collect(),
            position: 0,
            start_raw_code: false
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_commment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn get_raw_code(&mut self) -> TokenType {
        let mut raw_code: String = String::new();
        while let Some(ch) = self.peek() {
            if ch == '}' {
                self.advance();

                if let Some('%') = self.peek() {
                    self.advance();
                    self.start_raw_code = false;
                    return TokenType::RawCode(raw_code);
                } else {
                    raw_code.push(ch);
                    continue;
                }
            } else {
                raw_code.push(ch);
                self.advance();
            }
        }
        TokenType::Error(String::from("Reached EOF without finding closing }%"))
    }

    fn read_keyword_or_identifier(&mut self) -> TokenType {
        let mut pattern: String = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                pattern.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if pattern.is_empty() {
            return TokenType::Error(String::from("Expected keyword or identifier"));
        }

        if KEYWORDS.contains(&pattern.as_str()) {
            return TokenType::Keyword(pattern);
        } else {
            return TokenType::Identifier(pattern);
        }
    }

    fn read_operator(&mut self) -> TokenType {
        let mut pattern: String = String::new();
        let valid_operator_chars = ['=', '|', '-', '>'];
        let start = self.position;
        let mut best_match: Option<String> = None;

        while let Some(ch) = self.peek() {
            if !valid_operator_chars.contains(&ch) {
                break;
            }
            pattern.push(ch);
            self.advance();

            if OPERATORS.contains(&pattern.as_str()) {
                best_match = Some(pattern.clone());
            }
        }

        match best_match {
            Some(op) => {
                self.position = start + op.chars().count();
                TokenType::Operator(op)
            }
            None => {
                self.position = start;
                TokenType::Error(String::from("Unexpected operator!"))
            }
        }
    }

    fn read_string_literal(&mut self) -> TokenType {
        let mut pattern = String::new();
        let mut escape: bool = false;
        while let Some(ch) = self.peek() {

            match ch {
                '\"' => {
                    if escape {
                        pattern.push(ch);
                        self.advance();
                        escape = false;
                    } else {
                        self.advance();
                        break;
                    }
                }
                '\\' => {
                    escape = true;
                    pattern.push(ch);
                    self.advance();
                }
                _ => {
                    pattern.push(ch);
                    self.advance();
                    if escape {
                        escape = false;
                    }
                }
            }
        }

        if pattern.is_empty() {
            return TokenType::Error(String::from("Unexpected string literal!"));
        } else {
            return TokenType::StringLiteral(pattern);
        }
    }

    pub fn get_next_token(&mut self) -> TokenType {
        if self.start_raw_code {
            return self.get_raw_code();
        }

        self.skip_whitespace();

        let Some(ch) = self.peek() else {
            return TokenType::Eof;
        };

        match ch {
            '\0' => TokenType::Eof,

            '"' => {
                self.advance();
                self.read_string_literal()
            }

            '{' => {
                self.advance();
                TokenType::Lbrace(ch)
            }

            '}' => {
                self.advance();
                TokenType::Rbrace(ch)
            }

            '/' => {
                self.advance();
                if let Some('/') = self.peek() {
                    self.advance();
                    self.skip_commment();
                    return self.get_next_token();
                } else {
                    return TokenType::Error("Unexpected character: expected '//' for a comment".to_string());
                }
            }

            '%' => {
                self.advance();
                if let Some('{') = self.peek() {
                    self.advance();
                    self.start_raw_code = true;
                    return TokenType::CodeBlockStart;
                } else {
                    return self.read_keyword_or_identifier();
                }
            }
            '@' => {
                self.advance();
                return self.read_keyword_or_identifier();
            }

            ';' => {
                self.advance();
                TokenType::Semicolon(ch)
            }


            _ => {
                let tok = self.read_keyword_or_identifier();
                if matches!(tok, TokenType::Error(_)) {
                    let op_tok = self.read_operator();
                    if matches!(op_tok, TokenType::Error(_)) {
                        let bad_ch = ch;
                        self.advance();
                        TokenType::Error(format!("Unrecognized character: {}", bad_ch))

                    } else {
                        op_tok
                    }
                } else {
                    tok
                }
            },

        }
    }
}
