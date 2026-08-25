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
    - [X] Lexer generation
        - [X] Add ```regex``` as a dependency of the generated code
        - [X] For each ```TokenDef```, emit a compiled ```Regex```
        - [X] Emit a scanner doing maximal-munch: at each position, try all patterns, take the longest match, break ties by declaration order
        - [X] Decide how to handle "throwaway" tokens
        - [X] Add line/column tracking

    - [X] Table serialization
        - [X] Turn ```ParseTables.action``` and ```ParseTables.goto``` into something the emitted file can embed directly
        - [X] Emit ```Action``` as its own enum inside the generated file
    - [X] Driver
        - [X] Stack-based loop: ```state_stack: Vec<usize>```, ```value_stack: Vec<ParseTree>```
        - [X] ```ParseTree``` for now: ```enum ParseTree { Leaf(Token), Node(&'static str, Vec<ParseTree>)}```
        - [X] Shift: push token + target state. Reduce: pop ```rhs_len``` off both stacks, build a ```Node``` and consult GOTO table. Accept: done. No action found: returns an error
    - [ ] Assembly
        - [ ] Order: generator's own ```use regex::Regex;``` etc. -> prologue raw code -> Token enum -> lexer -> tables -> ```ParseTree```-> epilogue raw code.
        - [ ] Run the assembled string through ```rustfmt``` or the ```prettyplease``` crate before writing
        - [ ] Wire into main.rs: call to ```emitter::generate``` and ```fs::write```
