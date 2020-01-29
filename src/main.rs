use structopt::StructOpt;

mod controllers;
mod models;
mod services;
mod views;

use crate::{
    controllers::{Command, EntryType, Opt},
    models::{Entry, EntryVariants},
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
                    journal.append(Entry::new(EntryVariants::Event, text))
                }
                EntryType::Note { text } => {
                    journal.append(Entry::new(EntryVariants::Note, text))
                }
                EntryType::Task { text } => {
                    journal.append(Entry::new(EntryVariants::Task, text))
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
