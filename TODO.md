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
    - [ ] Design the runtime skeleton (the generic loop that reads the parsing table and processes tokens)
    - [ ] Write the templating engine that injects the generated tables and the user's custom semantic action into the skeleton
    - [ ] Ensure the emitted code is optmized (e.g. compressing sparse parsing tables to save memory)


