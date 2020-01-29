use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entries {
    Note,
    Task,
    Event
}

impl Default for Entries {
    fn default() -> Self {
        Entries::Note
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub important: bool,
    pub content: String,
    pub completed: bool,
    pub cancelled: bool,
    pub kind: Entries,
    pub children: Vec<Entry>
}

impl Entry {
    pub fn new(kind: Entries, content: String) -> Entry {
        Entry {
            kind,
            content,
            ..Default::default()
        }
    }
}

pub trait JournalEntry {
    fn render(&self) -> String;
}

impl JournalEntry for Entry {
    fn render(&self) -> String {
        match self.kind {
            Entries::Event => format!(
                "{important} {symbol} {content}",
                important = if self.important { "*" } else { " " },
                symbol = "\u{26AC}",
                content = self.content
            ),
            Entries::Task => format!(
                "{important} {symbol} {content}",
                important = if self.important { "*" } else { " " },
                symbol = if self.completed { "X" } else { "\u{2022}" },
                content = self.content
            ),
            Entries::Note => format!(
                "{important} {symbol} {content}",
                important = if self.important { "*" } else { " " },
                symbol = "-",
                content = self.content
            ),
        }
    }
}
