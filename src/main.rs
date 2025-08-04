use clap::Parser;
use schemix::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { file } => {
            println!("Building schemix project...");
            if let Some(file) = file {
                println!("Building file: {}", file);
            }
        }
        Commands::New { name } => {
            println!("Creating new schemix project: {}", name);
        }
    }
}
