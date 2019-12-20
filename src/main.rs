use structopt::StructOpt;

mod controllers;
mod models;
mod services;
mod views;

use crate::{
    controllers::{Command, Opt},
    models::{Entries, EntryType, Event, Note, Task},
    services::{Journalable, LocalDiskJournal},
    views::Application,
};

fn main() {
    let opt = Opt::from_args();

    let mut journal = LocalDiskJournal::new(None);

    // Handle input!
    if let Some(command) = opt.command {
        match command {
            Command::Add { entry_type } => match entry_type {
                EntryType::Event { text } => {
                    journal.append(Entries::Event(Event::new(text)));
                }
                EntryType::Note { text } => {
                    journal.append(Entries::Note(Note::new(text)));
                }
                EntryType::Task { text } => {
                    journal.append(Entries::Task(Task::new(text)));
                }
            },
            Command::Emph { index } => {
                journal.toggle_importance(index);
            }
            Command::Complete { index } => {
                journal.toggle_completion(index);
            }
            Command::Remove { index } => journal.remove(index),
        }
    }

    let application = Application {
        entries: journal.list(),
    };

    application.render();
}
