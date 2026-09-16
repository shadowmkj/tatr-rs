# tatr-rs

A Rust implementation and clone of [tatr](https://github.com/tsoding/tatr) by [Tsoding](https://github.com/tsoding).

`tatr` (Tag Tracker) is a tool designed to organize and query tasks, files, and items using tags and boolean expression queries.

## Features

- **Boolean Query Lexer**: Tokenizes query expressions supporting tags and logical operators (`and`, `or`, `not`).
- **Arena-Based AST Parser**: Builds syntax trees using an arena structure (`indextree`) with operator precedence parsing.
- **Tree Visualization**: Pretty-prints expression abstract syntax trees for query inspection.
- **CLI Interface**: Command-line parser built with `clap`.

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

