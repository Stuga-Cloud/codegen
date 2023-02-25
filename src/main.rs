extern crate tera;

mod core;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Debug, Parser)]
#[command(name = "generate")]
#[command(about = "generate new files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Component {
        name: String,
    },
    Page {
        name: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Component { name } => {
            println!("{name}");
        },
        Commands::Page { name } => {
            println!("{name}");
        },
    };
    Ok(())
}
