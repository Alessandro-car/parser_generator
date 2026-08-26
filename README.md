# Rust SLR(1) Parser Generator

A custom parser generator built in Rust that takes a grammar specification file (`.glr`) and compiles it into a fast, heavily optimized, and self-contained Rust module.

Currently implemented using an **SLR(1)** (Simple LR) parsing automaton, this tool generates a complete pipeline: a regex-based lexical analyzer, a deterministic finite automaton (DFA) state engine, and an untyped Abstract Syntax Tree (`ParseTree`) output.

## Features

* **SLR(1) Parsing Engine:** Uses FIRST and FOLLOW sets to construct deterministic ACTION and GOTO tables, ensuring linear `O(n)` parse times for unambiguous grammars.
* **Regex-Based Lexing:** Generates a robust lexer that automatically matches string patterns and keywords using Rust's `regex` crate, supporting maximal-munch tokenization.
* **Zero-Dependency Output:** The generated parser relies only on `std::sync::OnceLock` to statically initialize state tables, meaning your compiled parsers are blazingly fast and require minimal external dependencies.
* **Code Injection:** Supports `@prologue` and `@epilogue` blocks to seamlessly inject custom Rust structures, imports, and a `main` execution loop directly into the generated file.

## How It Works

1. **Read:** The generator parses a `.glr` file containing lexical token definitions and Context-Free Grammar (CFG) rules.
2. **Automaton Construction:** It builds an LR(0) state machine and augments it with global FOLLOW sets to resolve shift/reduce actions (SLR).
3. **Emission:** It emits a single `.rs` file containing:
   * A strongly typed `Token` enum.
   * Static ACTION and GOTO matrices.
   * A `Lexer` struct for string tokenization.
   * A `Parser` struct that drives the state machine.
   * A generic `ParseTree` enum representing the Graph-Structured Stack.

## Grammar Syntax (`.glr`)

Grammars are defined in a custom format broken into four main sections:

```rust
// 1. Prologue: Inject Rust imports
@prologue %{
    use std::fs;
}%

// 2. Tokens: Define regex-based lexical rules
%tokens {
    PLUS   = "\+";
    NUMBER = "[0-9]+";
}

// 3. Skip: Define whitespace or comments to ignore
%skip {
    WS = "[ \t\r\n]+";
}

// 4. Rules: Define the Context-Free Grammar
%start Program
%rules {
    Program -> Expr ;
    Expr -> Expr PLUS Expr | NUMBER ;
}

// 5. Epilogue: Inject driver code
@epilogue %{
    pub fn main() {
        // Run your parsed code here!
    }
}%
