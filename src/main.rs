
use clap::{Parser, Subcommand};

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "stringutils-cli")]
#[command(version = "0.1")]
#[command(about = "", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    Reverse {
        s: Option<String>,
    },
    Palindrome {
        s: Option<String>,
    },
    Count {
        s: Option<String>,
        #[arg(short, long, default_value_t=true)]
        chars: bool,
        #[arg(short, long)]
        words: bool,
    },
    Unique {
        s: Option<String>,
        #[arg(short, long, default_value_t=true)]
        chars: bool,
        #[arg(short, long)]
        words: bool,
    }
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Reverse { s }) => {
            commands::reverse::run(s);
        },
        Some(Commands::Palindrome { s }) => {
            commands::palindrome::run(s);
        },
        Some(Commands::Count { s, chars, words }) => {
            commands::count::run(s, chars, words);
        },
        Some(Commands::Unique { s, chars, words }) => {
            commands::unique::run(s, chars, words);
        },
        None => println!("No command specified. Try --help for available commands.")
    }

}
