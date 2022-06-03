use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(subcommand)]
    Add(AddCommands),
}

#[derive(Subcommand, Debug)]
pub enum AddCommands {
    Event { content: String },
    Note { content: String },
    Task { content: String },
}
