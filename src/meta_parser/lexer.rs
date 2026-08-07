//TODO: Handle Operator, StringLiteral, Semicolo and comments

static KEYWORDS: &[&str] = &[
    "prologue", "epilogue",
    "tokens", "start", "rules"
];


#[derive(Debug, PartialEq, Clone)]
pub enum TokType {
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

    fn get_raw_code(&mut self) -> TokType {
        let mut raw_code: String = String::new();
        while let Some(ch) = self.peek() {
            if ch == '}' {
                self.advance();

                if let Some('%') = self.peek() {
                    self.advance();
                    self.start_raw_code = false;
                    return TokType::RawCode(raw_code);
                } else {
                    raw_code.push(ch);
                    continue;
                }
            } else {
                raw_code.push(ch);
                self.advance();
            }
        }
        TokType::Error(String::from("Reached EOF without finding closing }%"))
    }

    fn read_keyword_or_identifier(&mut self) -> TokType {
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
            return TokType::Error(String::from("Expected keyword or identifier"));
        }

        if KEYWORDS.contains(&pattern.as_str()) {
            return TokType::Keyword(pattern);
        } else {
            return TokType::Identifier(pattern);
        }
    }


    pub fn get_next_token(&mut self) -> TokType {
        if self.start_raw_code {
            return self.get_raw_code();
        }

        self.skip_whitespace();

        let Some(ch) = self.peek() else {
            return TokType::Eof;
        };

        match ch {
            '\0' => TokType::Eof,
            '{' => {
                self.advance();
                TokType::Lbrace(ch)
            }

            '}' => {
                self.advance();
                TokType::Rbrace(ch)
            }

            '%' => {
                self.advance();
                if let Some('{') = self.peek() {
                    self.advance();
                    self.start_raw_code = true;
                    return TokType::CodeBlockStart;
                } else {
                    return self.read_keyword_or_identifier();
                }
            }

            '@' => {
                self.advance();
                return self.read_keyword_or_identifier();
            }

            _ => {
                //TODO: Handle general identifiers, like rules names, start symbols, etc
                return self.read_keyword_or_identifier();
            }
        }
    }


}
