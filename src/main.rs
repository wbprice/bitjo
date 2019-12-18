use structopt::StructOpt;

mod services;
mod models;
mod views;
mod controllers;

use crate::{
    services::{
        InMemoryJournal,
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
        Modes
    }
};

fn main() {
    let opt = Opt::from_args();

    // Stubbing out behavior where this is serialized and persisted to a backend
    let mut journal = InMemoryJournal::new();

    journal.append(
        Entries::Event(Event {
            content: "Internal Standup at 4pm".into(),
            ..Default::default()
        })
    );
    journal.append(
        Entries::Task(Task {
            content: "Figure out enums".into(),
            ..Default::default()
        })
    );
    journal.append(
        Entries::Note(Note {
            content: "I'm just surprised this worked".into(),
            ..Default::default()
        })
    );

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
        mode: Modes::Normal,
        entries: journal.list(),
    };

    application.render();
}
