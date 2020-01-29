use chrono::Local;
use dirs::home_dir;
use serde_yaml;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::models::Entry;

pub trait Journalable {
    fn new(path: Option<String>) -> Self;
    fn append(&mut self, entry: Entry);
    fn list(&self) -> &Vec<Entry>;
    fn remove(&mut self, index: usize);
    fn toggle_importance(&mut self, index: usize);
    fn toggle_completion(&mut self, index: usize);
    fn commit(&self);
}

pub struct InMemoryJournal {
    entries: Vec<Entry>,
}

impl Journalable for InMemoryJournal {
    fn new(_path: Option<String>) -> InMemoryJournal {
        InMemoryJournal { entries: vec![] }
    }

    fn append(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn list(&self) -> &Vec<Entry> {
        &self.entries
    }

    fn remove(&mut self, index: usize) {
        self.entries.remove(index);
    }

    fn commit(&self) {}

    fn toggle_importance(&mut self, index: usize) {
        let mut entry = self.entries.get_mut(index).unwrap();
        entry.important = !entry.important;
    }

    fn toggle_completion(&mut self, index: usize) {
        let mut entry = self.entries.get_mut(index).unwrap();
        entry.completed = !entry.completed;
    }
}

pub struct LocalDiskJournal {
    path: String,
    entries: Vec<Entry>,
}

impl Journalable for LocalDiskJournal {
    fn new(path: Option<String>) -> LocalDiskJournal {
        let prefix = home_dir().unwrap().join(".bitjo");
        fs::create_dir_all(prefix.to_str().unwrap()).unwrap();

        let path = match path {
            Some(path) => prefix.join(path).to_str().unwrap().to_string(),
            None => prefix
                .join(Local::now().format("%Y-%m-%d.yaml").to_string())
                .to_str()
                .unwrap()
                .to_string(),
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
                dbg!(&error);
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
                let entries: Vec<Entry> = serde_yaml::from_str(&contents).unwrap();
                LocalDiskJournal { path, entries }
            }
        }
    }

    fn append(&mut self, entry: Entry) {
        self.entries.push(entry);
        self.commit();
    }

    fn list(&self) -> &Vec<Entry> {
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

    fn toggle_importance(&mut self, index: usize) {
        let mut entry = self.entries.get_mut(index).unwrap();
        entry.important = !entry.important;
        self.commit();
    }

    fn toggle_completion(&mut self, index: usize) {
        let mut entry = self.entries.get_mut(index).unwrap();
        entry.completed = !entry.completed;
        self.commit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Entries, Entry};
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
        journal.append(Entry::new(
            Entries::Note,
            "Learn how to write unit tests".to_string(),
        ));
        let entries = journal.list();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn in_memory_journal_removes() {
        let mut journal = InMemoryJournal::new(None);
        journal.append(Entry::new(
            Entries::Note,
            "Learn how to write unit tests".to_string(),
        ));
        assert_eq!(journal.list().len(), 1);
        journal.remove(0);
        assert_eq!(journal.list().len(), 0);
    }

    #[test]
    fn in_memory_journal_toggles_importance() {
        let mut journal = InMemoryJournal::new(None);
        journal.append(Entry::new(
            Entries::Note,
            "Learn how to write unit tests".to_string(),
        ));
        journal.toggle_importance(0);
        assert_eq!(journal.list().len(), 1);
        if let note = &journal.list()[0] {
            assert_eq!(note.important, true);
        }
    }

    #[test]
    fn in_memory_journal_toggles_completion() {
        let mut journal = InMemoryJournal::new(None);
        journal.append(Entry::new(
            Entries::Task,
            "Learn how to write unit tests".to_string(),
        ));
        journal.toggle_completion(0);
        assert_eq!(journal.list().len(), 1);
        if let task = &journal.list()[0] {
            assert_eq!(task.completed, true);
        }
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
        journal.append(Entry::new(
            Entries::Note,
            "Learn how to write unit tests".to_string(),
        ));
        let entries = journal.list();
        let mut file = File::open(&journal.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let disk_entries: Vec<Entry> = serde_yaml::from_str(&contents).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(disk_entries.len(), 1);
        remove_file(&journal.path).unwrap();
    }

    #[test]
    fn on_disk_journal_removes() {
        let path = "remove-test".to_string();
        let mut journal = LocalDiskJournal::new(Some(path));
        journal.append(Entry::new(
            Entries::Note,
            "Learn how to write unit tests".to_string(),
        ));
        journal.append(Entry::new(
            Entries::Note,
            "Learn how to write integration tests".to_string(),
        ));

        let mut file = File::open(&journal.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let disk_entries: Vec<Entry> = serde_yaml::from_str(&contents).unwrap();
        assert_eq!(journal.list().len(), 2);
        assert_eq!(disk_entries.len(), 2);

        journal.remove(0);
        let mut file2 = File::open(&journal.path).unwrap();
        let mut contents2 = String::new();
        file2.read_to_string(&mut contents2).unwrap();

        assert_eq!(journal.list().len(), 1);
        let disk_entries2: Vec<Entry> = serde_yaml::from_str(&contents2).unwrap();
        assert_eq!(disk_entries2.len(), 1);

        remove_file(&journal.path).unwrap();
    }

    #[test]
    fn on_disk_journal_toggles_importance() {
        let mut journal = LocalDiskJournal::new(Some("importance-test".to_string()));
        journal.append(Entry::new(
            Entries::Note,
            "Learn how to write unit tests".to_string(),
        ));
        journal.toggle_importance(0);
        assert_eq!(journal.list().len(), 1);
        if let note = &journal.list()[0] {
            assert_eq!(note.important, true);
        }

        let mut file = File::open(&journal.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let disk_entries: Vec<Entry> = serde_yaml::from_str(&contents).unwrap();
        if let note = &disk_entries[0] {
            assert_eq!(note.important, true);
        }
        remove_file(&journal.path).unwrap();
    }

    #[test]
    fn on_disk_journal_toggles_completion() {
        let mut journal = LocalDiskJournal::new(Some("completion-test".to_string()));
        journal.append(Entry::new(
            Entries::Task,
            "Learn how to write unit tests".to_string(),
        ));
        journal.toggle_completion(0);
        assert_eq!(journal.list().len(), 1);
        if let task = &journal.list()[0] {
            assert_eq!(task.completed, true);
        }

        let mut file = File::open(&journal.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let disk_entries: Vec<Entry> = serde_yaml::from_str(&contents).unwrap();
        if let task = &disk_entries[0] {
            assert_eq!(task.completed, true);
        }
        remove_file(&journal.path).unwrap();
    }
}
