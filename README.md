# tatr-rs

> [!WARNING]
> **Work in Progress**: This project is currently in active development. Features, query evaluation, and CLI subcommands are still being built and tested.

A Rust implementation and clone of [tatr](https://github.com/tsoding/tatr) by [Tsoding](https://github.com/tsoding).

`tatr` (Tag Tracker) is a tool designed to organize and query tasks, files, and items using tags and boolean expression queries.

## Implementation Progress

The following components have been implemented:

- [x] **Lexer (`src/lexer.rs`)**: Streaming tokenizer for boolean queries supporting tag names, logical operators (`and`, `or`, `not`), and prefix dots.
- [x] **AST Parser (`src/parser.rs`)**: Recursive descent parser that constructs an arena-allocated Abstract Syntax Tree using `indextree`.
- [x] **AST Visualizer (`src/parser.rs`)**: Tree printer displaying parsed query expressions with branch connectors (`├──`, `└──`).
- [x] **Bytecode Compiler (`src/compiler.rs`)**: Shunting-yard compiler translating token streams into Reverse Polish Notation (RPN) stack bytecode instructions (`PUSH`, `APPLY`).
- [x] **Stack Machine VM (`src/vm.rs`)**: Stack-based virtual machine evaluator that evaluates compiled bytecode against target tag sets.
- [x] **CLI Interface (`src/main.rs`)**: Command-line interface with argument parsing using `clap`.

## Quick Start

### Prerequisites

Ensure you have Rust and Cargo installed:

```console
cargo --version
```

### Building & Running

Clone the repository and run queries using `cargo`:

```console
# Run a boolean query
cargo run -- "a and b or not c"
```

### Example Output

```text
a and b or not c
OR
├── AND
│   ├── TAG(a)
│   └── TAG(b)
└── NOT
    └── TAG(c)
```

## Acknowledgments

- Original C implementation and specification by [Tsoding](https://github.com/tsoding) in [tsoding/tatr](https://github.com/tsoding/tatr).

## License

GPL-2.0
