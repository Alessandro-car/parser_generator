static KEYWORDS: &[&str] = &[
    "@prologue", "@epilogue",
    "%tokens", "%start", "%rules"
];


#[derive(Debug, PartialEq, Clone)]
pub enum TokType {
    Eof,
    Keyword(String),
    Identifier(String),
    Rbrace(char),
    Lbrace(char),
    CodeBlockStart,
    CodeBlockEnd,
    RawCode(String),
    Error(String)
}

pub struct Lexer {
    input: Vec<char>,
    position: usize
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Lexer {
            input: content.chars().collect(),
            position: 0
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

    fn get_keyword_or_identifier(&self, pattern: &str) -> TokType {
        if KEYWORDS.contains(&pattern) {
            TokType::Keyword(pattern.to_string())
        } else {
            TokType::Identifier(pattern.to_string())
        }
    }

    fn get_raw_code(&mut self) -> TokType {
        let mut raw_code: String = String::new();
        while let Some(ch) = self.peek() {
            if ch == '}' {
                self.advance();

                if let Some('%') = self.peek() {
                    self.advance();
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


    pub fn get_next_token(&mut self) -> TokType {
        self.skip_whitespace();

        let Some(ch) = self.peek() else {
            return TokType::Eof;
        };

        match ch {
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
                    TokType::CodeBlockStart;
                } else {
                    //TODO: Handle %start, %rules, ...
                }
            }

            '@' => {
                //TODO: Handle @prologue, @epilogue
            }

            _ => {
                //TODO: Handle general identifiers, like rules names, start symbols, etc
            }
        }
    }

}
