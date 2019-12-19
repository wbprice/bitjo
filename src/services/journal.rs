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
}