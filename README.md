# tatr-rs

> [!WARNING]
> **Work in Progress**: This project is currently in active development. Additional subcommands and features are still being built and refined.

A Rust implementation and clone of [tatr](https://github.com/tsoding/tatr) by [Tsoding](https://github.com/tsoding).

`tatr` (Task Tracker) is a tool designed to organize and query tasks, files, and items using tags and boolean expression queries.

## Current Status & Features

- [x] **Lexer (`src/lexer.rs`)**: Streaming tokenizer for boolean queries supporting tag names (`tag`, `.tag`), logical operators (`and`, `or`, `not`), and error tokens.
- [x] **AST Parser & Visualizer (`src/parser.rs`)**: Recursive descent parser building arena-allocated syntax trees (`indextree`) with tree branch visualization (`├──`, `└──`).
- [x] **Bytecode Compiler (`src/compiler.rs`)**: Shunting-Yard compiler translating token streams into Reverse Polish Notation (RPN) stack bytecode instructions (`PUSH`, `APPLY`) with `Ord`-derived operator precedence.
- [x] **Stack Machine VM (`src/vm.rs`)**: Fast stack-based virtual machine evaluating compiled bytecode queries against task tag sets.
- [x] **Task Management (`src/task.rs`)**: Serializes and parses `TASK.md` files conforming to the Tatr file format specification.
- [x] **CLI Subcommands (`src/main.rs`)**:
  - `init`: Initializes the task storage directory.
  - `new` / `add`: Creates a new task with title, priority (`-p`), tags (`-t`), and body.
  - `ls` / `list`: Lists tasks in canonical format, sorted by priority and date, with boolean query filtering (`-q`).
- [x] **Test Data Generator (`examples/generate_tasks.rs`)**: Utility script to seed realistic test tasks with distinct timestamp IDs for testing query filters.

## Quick Start

### Prerequisites

Ensure you have Rust and Cargo installed:

```console
cargo --version
```

### CLI Commands

#### 1. Initialize Task Storage
```console
cargo run -- init
```

#### 2. Create a New Task
```console
cargo run -- new "Implement parser recovery" -p 90 -t "parser,bug" "Fix syntax error handling."
```

#### 3. List All Tasks
```console
cargo run -- ls
```

#### 4. Filter Tasks with Boolean Queries
```console
# Filter tasks with boolean expressions
cargo run -- ls -q ":parser and not :bug"

# Combine operators
cargo run -- ls -q ":cli or :vm"
```

### Example Listing Output

```text
./issues/20260916-193927/TASK.md:1: OPEN [PRIORITY: 100] [parser,vm] Optimize issue search filter
./issues/20260916-193923/TASK.md:1: OPEN [PRIORITY: 100] [cli] Optimize tag indexing and caching
./issues/20260916-193918/TASK.md:1: OPEN [PRIORITY:  80] [refactor,parser] Benchmark parser precedence handling
./issues/20260916-193905/TASK.md:1: OPEN [PRIORITY:  70] [parser] Refactor task metadata serialization
./issues/20260916-193922/TASK.md:1: OPEN [PRIORITY:  10] [cli,bug,lexer] Refactor issue search filter
```

### Generating Test Tasks

To populate the issues directory with sample tasks:

```console
cargo run --example generate_tasks -- 10
```

## Acknowledgments

- Original C implementation and specification by [Tsoding](https://github.com/tsoding) in [tsoding/tatr](https://github.com/tsoding/tatr).

## License

GPL-2.0
