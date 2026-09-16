use clap::Parser;
use tatr_rs::lexer::Lexer;

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

    let lexer = Lexer::new(&query.trim());
    for token in lexer {
        println!("Token: {token:?}");
    }
}
