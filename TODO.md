# Project TODO

## Backlog / Features 🧊
- [X] Implementing the meta-parser (bootstrapping)
    - [X] Define the syntax for grammar files (.glr files)
    - [X] Write a lexer and a parser for the parser generator.
    - [X] Build an AST representing the user grammar
- [ ] Core generator logic
    - [ ] Write algorithms to compute FIRST and FOLLOW sets for any given grammar
    - [ ] Implement the state machine generator (LR(0) or LR(1) item sets)
    - [ ] Implement the parsing table constructors (Action and Goto tables)
    - [ ] Write the conflict detector (Identifying Shift/Reduce and Reduce/Reduce conflicts)
- [ ]


