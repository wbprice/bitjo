use structopt::StructOpt;

mod services;
mod models;
mod views;
mod controllers;

use crate::{
    services::{
        LocalDiskJournal,
        Journalable
    },
    models::{
        Task,
        Event,
        Note,
        EntryType,
        Entries
    },
    controllers::{Opt, Command},
    views::{
        Application,
    }
};

fn main() {
    let opt = Opt::from_args();

    // Stubbing out behavior where this is serialized and persisted to a backend
    let mut journal = LocalDiskJournal::new();

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
            Command::Cancel => {
                unimplemented!();
            }
            Command::Complete => {
                unimplemented!();
            }
            Command::Remove => {
                unimplemented!();
            }
        }
    }

    let application = Application {
        entries: journal.list(),
    };

    application.render();
}
