use clap::Parser;

mod lib;
use crate::lib::{AddCommands, Cli, Commands, Entry, Event, Note, Task};

fn main() {
    let cli = Cli::parse();

    let mut entries: Vec<Box<dyn Entry>> = vec![
        Note::new("Hello note!".into()),
        Event::new("Hello event!".into()),
        Task::new("Hello todo!".into()),
    ];

    match &cli.command {
        Commands::Add(add_command) => match add_command {
            AddCommands::Event { content } => {
                entries.push(Event::new(content.into()));
            }
            AddCommands::Note { content } => {
                entries.push(Note::new(content.into()));
            }
            AddCommands::Task { content } => {
                entries.push(Task::new(content.into()));
            }
        },
    }

    for entry in entries {
        println!("{}", entry.text());
    }
}
