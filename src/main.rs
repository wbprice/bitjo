use clap::Parser;

mod lib;
use crate::lib::{AddCommands, Cli, Commands, Entry, Event, Note, Task};

fn main() {
    let cli = Cli::parse();

    let note = Note::new("Hello note!".into());
    let mut event = Event::new("Hello event!".into());
    let task = Task::new("Hello todo!".into());
    event.insert(Note::new("Hello child note".into()));

    let mut entries: Vec<Box<dyn Entry>> = vec![note, event, task];

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
