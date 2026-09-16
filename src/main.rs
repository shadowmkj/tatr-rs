// Copyright (C) 2026 Milan Pramod
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
//

use std::{
    fs::{self},
    path::Path,
};

use clap::{Parser, Subcommand};
use tatr_rs::{
    DIRNAME, compiler::Compiler, generate_datetime_id, lexer::Lexer, task::Task, vm::Vm,
};

#[derive(Debug, Subcommand)]
enum Command {
    #[command(name = "ls", alias = "list")]
    List {
        #[arg(short, long)]
        query: Option<String>,
    },
    Init,
    #[command(name = "new", alias = "add")]
    New {
        title: Option<String>,
        #[arg(short, long, default_value_t = 100)]
        priority: u8,
        #[arg(short, long, default_values_t = Vec::<String>::new(), value_delimiter = ',')]
        tags: Vec<String>,
        body: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::List { query } => {
            let dir_path = Path::new(DIRNAME);
            if !dir_path.exists() {
                println!("{DIRNAME} directory does not exist. Run 'tatr init' first.");
                return;
            }

            let mut tasks = Vec::new();
            if let Ok(entries) = fs::read_dir(dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(Some(task)) = Task::from_dir(&path) {
                            tasks.push(task);
                        }
                    }
                }
            }

            // If a query is provided, compile and filter with the VM
            if let Some(ref q) = query {
                let lexer = Lexer::new(q);
                let instructions = Compiler::compile(lexer);
                tasks.retain(|task| Vm::eval(&instructions, &task.tag_set()));
            }

            // Sort by priority descending, then by ID descending
            tasks.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| b.id.cmp(&a.id)));

            if tasks.is_empty() {
                println!("No tasks found.");
                return;
            }

            for task in tasks {
                println!(
                    "./{DIRNAME}/{}/TASK.md:1: {} [PRIORITY: {:>3}] [{}] {}",
                    task.id,
                    task.status,
                    task.priority,
                    task.tags.join(","),
                    task.title
                );
            }
        }

        Command::Init => {
            let dir_path = Path::new(DIRNAME);
            if !dir_path.is_dir() {
                let _ = fs::create_dir(DIRNAME);
                println!("Initialized empty directory");
            } else {
                println!("{DIRNAME} directory already exists");
            }
        }
        Command::New {
            title,
            priority,
            tags,
            body,
        } => {
            let id = generate_datetime_id();
            let task = Task::new(
                id,
                title.unwrap_or_default(),
                priority,
                tags,
                body.unwrap_or_default(),
            );
            if let Ok(_) = task.save() {
                println!("Saved");
            } else {
                println!("{DIRNAME} directory not found run tatr init");
            }
        }
    }
}
