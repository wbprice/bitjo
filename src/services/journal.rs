use chrono::Local;
use serde_yaml;
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::path::Path;

use crate::models::{Entries, Note, Event};

pub trait Journalable {
    fn new() -> Self;
    fn append(&mut self, entry: Entries);
    fn list(&self) -> &Vec<Entries>;
}

pub struct InMemoryJournal {
    entries: Vec<Entries>,
}

impl Journalable for InMemoryJournal {
    fn new() -> InMemoryJournal {
        InMemoryJournal { entries: vec![] }
    }

    fn append(&mut self, entry: Entries) {
        self.entries.push(entry);
    }

    fn list(&self) -> &Vec<Entries> {
        &self.entries
    }
}

pub struct LocalDiskJournal {
    file: std::fs::File,
    entries: Vec<Entries>,
}

impl Journalable for LocalDiskJournal {
    fn new() -> LocalDiskJournal {
        let path = Local::now().format("%a-%b-%e.yaml").to_string();
        let mut file = match OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(error) => {
                panic!(error);
            }
        };

        // Read the file.  Does it have any entries?
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        return match &contents.is_empty() {
            true => LocalDiskJournal {
                file,
                entries: vec![],
            },
            false => {
                let entries: Vec<Entries> = serde_yaml::from_str(&contents).unwrap();
                LocalDiskJournal { file, entries }
            }
        };
    }

    fn append(&mut self, entry: Entries) {
        self.entries.push(entry);
        // Update the file.
        let yaml = serde_yaml::to_string(&self.entries).unwrap();
        self.file.write_all(&yaml.as_bytes()).unwrap();
    }

    fn list(&self) -> &Vec<Entries> {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_journal_created() {
        let journal = InMemoryJournal::new();
        let entries = journal.list();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn in_memory_journal_appends() {
        let mut journal = InMemoryJournal::new();
        journal.append(Entries::Note(Note::new(
            "Learn how to write unit tests".to_string(),
        )));
        let entries = journal.list();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn on_disk_journal_created() {
        let journal = LocalDiskJournal::new();
        let entries = journal.list();
        let path = Local::now().format("%a-%b-%e.txt").to_string();
        let file = File::open(&path);
        assert_eq!(entries.len(), 0);
        assert!(file.is_ok());
    }

    #[test]
    fn on_dish_journal_appends() {
        let mut journal = LocalDiskJournal::new();
        journal.append(Entries::Note(Note::new(
            "Learn how to write unit tests".to_string(),
        )));
        let entries = journal.list();
        assert_eq!(entries.len(), 1);
    }
}
