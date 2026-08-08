use crate::TokenType;
use crate::Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    GrammarFile {
        prologue: Option<Prologue>,
        tokens: TokenSet,
        start: String,
        rules: GrammarRuleSet,
        epilogue: Option<Epilogue>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prologue { raw_code: String }

#[derive(Debug, Clone, PartialEq)]
pub struct Epilogue { raw_code: String }

#[derive(Debug, Clone, PartialEq)]
pub struct TokenDef { id: String, pattern: String }

#[derive(Debug, Clone, PartialEq)]
pub struct TokenSet { defs: Vec<TokenDef> }

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarRuleDef {
    lhs: String,
    alternatives: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarRuleSet { rules: Vec<GrammarRuleDef> }


pub struct Parser {
    lexer: Lexer,
    token: TokenType
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, String> {
        let token = lexer.get_next_token();
        if let TokenType::Error(msg) = token {
            return Err(msg);
        }
        Ok( Parser { lexer, token } )
    }

    fn advance(&mut self) {
        self.token = self.lexer.get_next_token();
    }

    fn expect_token(&mut self, expected: TokenType) {
        if self.token != expected {
            panic!("Expected {:?} but got {:?}", expected, self.token);
        }
    }

    fn parse_prologue(&mut self) -> Prologue {
        self.advance();
        self.expect_token(TokenType::CodeBlockStart);
        self.advance();
        match std::mem::replace(&mut self.token, TokenType::Eof) {
            TokenType::RawCode(code) => {
                self.advance();
                Prologue { raw_code: code }
            }
            other => panic!("Expected RawCode but got {:?}", other),
        }
    }

    fn parse_epilogue(&mut self) -> Epilogue {
        self.advance();
        self.expect_token(TokenType::CodeBlockStart);
        self.advance();
        match std::mem::replace(&mut self.token, TokenType::Eof) {
            TokenType::RawCode(code) => {
                self.advance();
                Epilogue { raw_code: code }
            }
            other => panic!("Expected RawCode but got {:?}", other),
        }
    }

    fn parse_token_def(&mut self) -> TokenDef {
        let token_id = match std::mem::replace(&mut self.token, TokenType::Eof) {
            TokenType::Identifier(id) => {
                self.advance();
                id
            }

            other => panic!("Expected an identifier but got {:?}", other),
        };

        self.expect_token(TokenType::Operator(String::from("=")));
        self.advance();
        let token_val = match std::mem::replace(&mut self.token, TokenType::Eof) {
            TokenType::StringLiteral(val) => {
                self.advance();
                val
            }
            other => panic!("Expected a string literal but got {:?}", other),
        };
        self.expect_token(TokenType::Semicolon(';'));
        self.advance();
        TokenDef { id: token_id, pattern: token_val }
    }

    fn parse_token_section(&mut self) -> TokenSet {
        self.advance();
        self.expect_token(TokenType::Lbrace('{'));
        self.advance();

        let mut defs = Vec::new();

        while !matches!(self.token, TokenType::Rbrace('}')) {
            defs.push(self.parse_token_def());
        }

        self.expect_token(TokenType::Rbrace('}'));
        self.advance();
        TokenSet { defs }
    }

    fn parse_start_section(&mut self) -> String {
        self.advance();

        match std::mem::replace(&mut self.token, TokenType::Eof) {
            TokenType::Identifier(start) => {
                self.advance();
                start
            }
            other => panic!("Expected a string literal but got {:?}", other),
        }
    }

    fn parse_rule_def(&mut self) -> GrammarRuleDef {
        let lhs = match std::mem::replace(&mut self.token, TokenType::Eof) {
            TokenType::Identifier(lhs) => {
                self.advance();
                lhs
            }
            other => panic!("Expected an identifier but got {:?}", other),
        };
        self.expect_token(TokenType::Operator(String::from("->")));
        self.advance();
        let mut alternatives = Vec::new();
        let mut expression = Vec::new();
        while !matches!(self.token, TokenType::Semicolon(';')) {
            match std::mem::replace(&mut self.token, TokenType::Eof) {
                TokenType::Operator(op) if op == "|" => {
                    self.advance();
                    alternatives.push(std::mem::take(&mut expression));
                }

                TokenType::Identifier(entry) => {
                    self.advance();
                    expression.push(entry);
                }
                other => panic!("Expected a | or an identifier but got {:?}", other),
            }
        }
        alternatives.push(expression);
        self.expect_token(TokenType::Semicolon(';'));
        self.advance();
        GrammarRuleDef { lhs, alternatives }
    }

    fn parse_rule_section(&mut self) -> GrammarRuleSet {
        self.advance();
        self.expect_token(TokenType::Lbrace('{'));
        self.advance();
        let mut rules = Vec::new();

        while !matches!(self.token, TokenType::Rbrace('}')) {
            rules.push(self.parse_rule_def());
        }
        self.expect_token(TokenType::Rbrace('}'));
        self.advance();
        GrammarRuleSet { rules }
    }

    pub fn parse(&mut self) -> ASTNode {
        let mut prologue: Option<Prologue> = None;
        let mut tokens: Option<TokenSet> = None;
        let mut start: Option<String> = None;
        let mut rules: Option<GrammarRuleSet> = None;
        let mut epilogue: Option<Epilogue> = None;

        loop {
            match &self.token {
                TokenType::Keyword(k) if k == "prologue" => {
                    prologue = Some(self.parse_prologue());
                }
                TokenType::Keyword(k) if k == "tokens" => {
                    tokens = Some(self.parse_token_section());
                }
                TokenType::Keyword(k) if k == "start" => {
                    start = Some(self.parse_start_section());
                }
                TokenType::Keyword(k) if k == "rules" => {
                    rules = Some(self.parse_rule_section());
                }
                TokenType::Keyword(k) if k == "epilogue" => {
                    epilogue = Some(self.parse_epilogue());
                }
                TokenType::Eof => break,
                other => panic!("Unexpected top-level token: {:?}", other),
            }
        }

        ASTNode::GrammarFile {
            prologue,
            tokens: tokens.expect("Missing %tokens section"),
            start: start.expect("Missing %start section"),
            rules: rules.expect("Missing %rules section"),
            epilogue,
        }
    }
}
