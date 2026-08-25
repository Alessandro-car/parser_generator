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

impl ASTNode {
    pub fn get_prologue(&self) -> &Option<Prologue> {
        match self {
            ASTNode::GrammarFile { prologue, .. } => prologue,
        }
    }

    pub fn get_epilogue(&self) -> &Option<Epilogue> {
        match self {
            Self::GrammarFile { epilogue, .. } => epilogue,
        }
    }

    pub fn get_rules(&self) -> &GrammarRuleSet {
        match self {
            ASTNode::GrammarFile { rules, .. } => rules,
        }
    }

    pub fn get_token_set(&self) -> &TokenSet {
        match self {
            ASTNode::GrammarFile { tokens, .. } => tokens,
        }
    }

    pub fn get_start_sym(&self) -> &String {
        match self {
            ASTNode::GrammarFile { start, .. } => start,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prologue { raw_code: String }

impl Prologue {
    pub fn get_prologue(&self) -> &String {
        &self.raw_code
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Epilogue { raw_code: String }

impl Epilogue {
    pub fn get_epilogue(&self) -> &String {
        &self.raw_code
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenDef {
    id: String,
    pattern: String,
    is_skip: bool
}

impl TokenDef {
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_pattern(&self) -> &String {
        &self.pattern
    }

    pub fn is_skip(&self) -> bool {
        self.is_skip
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenSet { defs: Vec<TokenDef> }

impl TokenSet {
    pub fn get_defs(&self) -> &Vec<TokenDef> {
        &self.defs
    }

    pub fn get_defs_mut(&mut self) -> &mut Vec<TokenDef> {
        &mut self.defs
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarRuleDef {
    lhs: String,
    alternatives: Vec<Vec<String>>,
}

impl GrammarRuleDef {
    pub fn new(lhs: String, alternatives: Vec<Vec<String>>) -> Self {
        GrammarRuleDef {
            lhs,
            alternatives
        }
    }

    pub fn get_lhs(&self) -> &String {
        &self.lhs
    }

    pub fn get_alternatives(&self) -> &Vec<Vec<String>> {
        &self.alternatives
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarRuleSet { rules: Vec<GrammarRuleDef> }

impl GrammarRuleSet {
    pub fn get_rules(&self) -> &Vec<GrammarRuleDef> {
        &self.rules
    }

    pub fn with_appended_rule(mut self, rule: GrammarRuleDef) -> Self {
        self.rules.push(rule);
        self
    }
}


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

    fn parse_token_def(&mut self, is_skip: bool) -> TokenDef {
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
        TokenDef { id: token_id, pattern: token_val, is_skip }
    }

    fn parse_token_section(&mut self, is_skip: bool) -> TokenSet {
        self.advance();
        self.expect_token(TokenType::Lbrace('{'));
        self.advance();

        let mut defs = Vec::new();

        while !matches!(self.token, TokenType::Rbrace('}')) {
            defs.push(self.parse_token_def(is_skip));
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
        let mut skippable_tokens: Option<TokenSet> = None;
        let mut start: Option<String> = None;
        let mut rules: Option<GrammarRuleSet> = None;
        let mut epilogue: Option<Epilogue> = None;

        loop {
            match &self.token {
                TokenType::Keyword(k) if k == "prologue" => {
                    prologue = Some(self.parse_prologue());
                }
                TokenType::Keyword(k) if k == "tokens" => {
                    tokens = Some(self.parse_token_section(false));
                }
                TokenType::Keyword(k) if k == "skip" => {
                    skippable_tokens = Some(self.parse_token_section(true));
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

        let mut merged_tokens = tokens.expect("Missing %tokens section");

        if let Some(mut skip_tokens) = skippable_tokens {
            merged_tokens.get_defs_mut().append(skip_tokens.get_defs_mut());
        }

        ASTNode::GrammarFile {
            prologue,
            tokens: merged_tokens,
            start: start.expect("Missing %start section"),
            rules: rules.expect("Missing %rules section"),
            epilogue,
        }
    }
}
