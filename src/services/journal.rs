use chrono::Local;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde_yaml;

use crate::models::{
    Entries,
    Note
};

pub trait Journalable {
    fn new() -> Self;
    fn append(&mut self, entry: Entries);
    fn list(&self) -> &Vec<Entries>;
}

pub struct InMemoryJournal {
    entries: Vec<Entries>
}

impl Journalable for InMemoryJournal {
    fn new() -> InMemoryJournal {
       InMemoryJournal {
           entries: vec![]
       } 
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
    entries: Vec<Entries>
}

impl Journalable for LocalDiskJournal {
    fn new() -> LocalDiskJournal {

        let path = Local::now().format("%a-%b-%e.txt").to_string();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(error) => {
                match File::create(&path) {
                    Ok(file) => file,
                    Err(error) => {
                        panic!("The backing file doesn't exist and can't be created");
                    }
                }
            }
        };

        // Read the file.  Does it have any entries?
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let entries : Vec<Entries> = serde_yaml::from_str(&contents).unwrap();

        // base case, create a new file and return the vec
        LocalDiskJournal {
            file,
            entries
        }
    }

    fn append(&mut self, entry: Entries) {
        self.entries.push(entry);
        // Update the file.

        self.file.write_all(serde_yaml::to_string(&self.entries).unwrap().as_bytes()).unwrap();
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
        journal.append(Entries::Note(Note::new("Learn how to write unit tests".to_string())));
        let entries = journal.list();
        assert_eq!(entries.len(), 1);
        dbg!(entries);
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
        let mut journal = InMemoryJournal::new();
        journal.append(Entries::Note(Note::new("Learn how to write unit tests".to_string())));
        let entries = journal.list();
        assert_eq!(entries.len(), 1);
    }
}