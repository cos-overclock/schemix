use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "schemix")]
#[command(about = "A Rust EDA framework for circuit design")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Build a schemix project")]
    Build {
        #[arg(short, long)]
        file: Option<String>,
    },
    #[command(about = "Create a new schemix project")]
    New {
        name: String,
    },
}