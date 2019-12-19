use chrono::Local;
use serde_yaml;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::models::Entries;

pub trait Journalable {
    fn new(path: Option<String>) -> Self;
    fn append(&mut self, entry: Entries);
    fn list(&self) -> &Vec<Entries>;
    fn remove(&mut self, index: usize);
    fn commit(&self);
}

pub struct InMemoryJournal {
    entries: Vec<Entries>,
}

impl Journalable for InMemoryJournal {
    fn new(_path: Option<String>) -> InMemoryJournal {
        InMemoryJournal { entries: vec![] }
    }

    fn append(&mut self, entry: Entries) {
        self.entries.push(entry);
    }

    fn list(&self) -> &Vec<Entries> {
        &self.entries
    }

    fn remove(&mut self, index: usize) {
        self.entries.remove(index);
    }

    fn commit(&self) {}
}

pub struct LocalDiskJournal {
    path: String,
    entries: Vec<Entries>,
}

impl Journalable for LocalDiskJournal {
    fn new(path: Option<String>) -> LocalDiskJournal {
        // If no path is provided, use the current date.
        let path = match path {
            Some(path) => path,
            None => Local::now().format("%Y-%m-%d.yaml").to_string()
        };

        // Get a handle to the file
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
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

        // If the file has contents, set the contents as the initial state of entries
        match &contents.is_empty() {
            true => LocalDiskJournal {
                path,
                entries: vec![],
            },
            false => {
                let entries: Vec<Entries> = serde_yaml::from_str(&contents).unwrap();
                LocalDiskJournal { path, entries }
            }
        }
    }

    fn append(&mut self, entry: Entries) {
        self.entries.push(entry);
        self.commit();
    }

    fn list(&self) -> &Vec<Entries> {
        &self.entries
    }

    fn remove(&mut self, index: usize) {
        self.entries.remove(index);
        self.commit();
    }

    fn commit(&self) {
        // Update the file.
        let yaml = format!("{}\n", serde_yaml::to_string(&self.entries).unwrap());

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
        {
            Ok(file) => file,
            Err(error) => {
                panic!(error);
            }
        };

        file.write_all(&yaml.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Note;
    use std::fs::{remove_file, File};

    #[test]
    fn in_memory_journal_created() {
        let journal = InMemoryJournal::new(None);
        let entries = journal.list();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn in_memory_journal_appends() {
        let mut journal = InMemoryJournal::new(None);
        journal.append(Entries::Note(Note::new(
            "Learn how to write unit tests".to_string(),
        )));
        let entries = journal.list();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn in_memory_journal_removes() {
        let mut journal = InMemoryJournal::new(None);
        journal.append(Entries::Note(Note::new(
            "Learn how to write unit tests".to_string(),
        )));
        assert_eq!(journal.list().len(), 1);
        journal.remove(0);
        assert_eq!(journal.list().len(), 0);
    }

    #[test]
    fn on_disk_journal_created() {
        let journal = LocalDiskJournal::new(Some("creation-test".to_string()));
        let entries = journal.list();
        let file = File::open(&journal.path);
        assert_eq!(entries.len(), 0);
        assert!(file.is_ok());
        remove_file(&journal.path).unwrap();
    }

    #[test]
    fn on_disk_journal_appends() {
        let mut journal = LocalDiskJournal::new(Some("append-test".to_string()));
        journal.append(Entries::Note(Note::new(
            "Learn how to write unit tests".to_string(),
        )));
        let entries = journal.list();
        let mut file = File::open(&journal.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let disk_entries: Vec<Entries> = serde_yaml::from_str(&contents).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(disk_entries.len(), 1);
        remove_file(&journal.path).unwrap();
    }

    #[test]
    fn on_disk_journal_removes() {
        let path = "remove-test".to_string();
        let mut journal = LocalDiskJournal::new(Some(path));
        journal.append(Entries::Note(Note::new(
            "Learn how to write unit tests".to_string(),
        )));
        journal.append(Entries::Note(Note::new(
            "Learn how to write integration tests".to_string(),
        )));

        let mut file = File::open(&journal.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let disk_entries: Vec<Entries> = serde_yaml::from_str(&contents).unwrap();
        assert_eq!(journal.list().len(), 2);
        assert_eq!(disk_entries.len(), 2);

        journal.remove(0);
        let mut file2 = File::open(&journal.path).unwrap();
        let mut contents2 = String::new();
        file2.read_to_string(&mut contents2).unwrap();

        assert_eq!(journal.list().len(), 1);
        let disk_entries2 : Vec<Entries> = serde_yaml::from_str(&contents2).unwrap();
        assert_eq!(disk_entries2.len(), 1);

        remove_file(&journal.path).unwrap();
    }
}
