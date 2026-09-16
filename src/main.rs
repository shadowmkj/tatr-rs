// Copyright (C) 2026 Your Name
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
//

use std::collections::HashSet;

use clap::Parser;
use tatr_rs::{compiler::Compiler, lexer::Lexer, vm::Vm};

// #[derive(Debug, Subcommand)]
// enum Command {
//     #[command(name = "ls", alias = "list")]
//     List {
//         #[arg(short, long)]
//         query: Option<String>,
//     },
// }

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    query: String,
    // #[command(subcommand)]
    // command: Command,
}

fn main() {
    let cli = Cli::parse();
    let query = cli.query;
    println!("{query}");

    let lexer = Lexer::new(&query);
    let instructions = Compiler::compile(lexer);
    let mut item_tags = HashSet::new();
    item_tags.insert("bug".to_string());
    item_tags.insert("feature".to_string());
    println!("{instructions:?}");
    println!("Verdict: {}", Vm::eval(&instructions, &item_tags));
    // let parser = tatr_rs::parser::Parser::new(lexer);
    // let (arena, root) = parser.parse();
    // tatr_rs::parser::print_tree(&arena, root);
}
