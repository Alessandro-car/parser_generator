# Project TODO

## Backlog / Features 🧊
- [X] Implementing the meta-parser (bootstrapping)
    - [X] Define the syntax for grammar files (.glr files)
    - [X] Write a lexer and a parser for the parser generator.
    - [X] Build an AST representing the user grammar
- [X] Core generator logic
    - [X] Write algorithms to compute FIRST and FOLLOW sets for any given grammar
    - [X] Implement the state machine generator (LR(0) or LR(1) item sets)
    - [X] Implement the parsing table constructors (Action and Goto tables)
    - [X] Write the conflict detector (Identifying Shift/Reduce and Reduce/Reduce conflicts)
- [ ] The code emitter
    - [X] Symbol/rule tables
        - [X] Sanitize grammar identifiers into valid Rust identifiers
        - [X] Emit a ```Token``` enum: one variant per terminal, carrying the lexeme(```Number(String)```) plus ```Eof```
        - [X] Emit a rule table mapping ```(rule_idx, alt_idx) -> (lhs_name, rhs_len)```
    - [ ] Lexer generation
        - [ ] Add ```regex``` as a dependency of the generated code
        - [ ] For each ```TokenDef```, emit a compiled ```Regex```
        - [ ] Emit a scanner doing maximal-munch: at each position, try all patterns, take the longest match, break ties by declaration order
        - [ ] Decide how to handle "throwaway" tokens
        - [ ] Add line/column tracking

    - [ ] Table serialization
        - [ ] Turn ```ParseTables.action``` and ```ParseTables.goto``` into something the emitted file can embed directly
        - [ ] Emit ```Action``` as its own enum inside the generated file
    - [ ]

