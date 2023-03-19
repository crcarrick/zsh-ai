use anyhow::Result;
use clap::{Parser, Subcommand};

use zsh_ai::commands;

#[derive(Debug, Subcommand)]
enum Commands {
    Completion(commands::completion::Completion),
    Login(commands::login::Login),
}

#[derive(Debug, Parser)]
#[command(
    name = "zsh_ai",
    about = "A CLI for interacting with OpenAI's API",
    version = "0.1.0"
)]
struct CLI {
    #[clap(subcommand)]
    command: Commands,
}

pub fn main() -> Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Completion(args) => match commands::completion::completion(args) {
            Ok(r) => {
                r.choices.first().map(|c| println!("{}", c.text));
            }
            Err(e) => eprintln!("Completion failed: {}", e),
        },
        Commands::Login(args) => match commands::login::login(args) {
            Ok(_) => println!("Login successful"),
            Err(e) => eprintln!("Login failed: {}", e),
        },
    }

    Ok(())
}
