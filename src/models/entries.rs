use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryVariants {
    Note,
    Task,
    Event
}

impl Default for EntryVariants {
    fn default() -> Self {
        EntryVariants::Note
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub important: bool,
    pub content: String,
    pub completed: bool,
    pub cancelled: bool,
    pub variant: EntryVariants,
    pub children: Vec<Entry>
}

impl Entry {
    pub fn new(variant: EntryVariants, content: String) -> Entry {
        Entry {
            variant,
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
        match self.variant {
            EntryVariants::Event => format!(
                "{important} {symbol} {content}",
                important = if self.important { "*" } else { " " },
                symbol = "\u{26AC}",
                content = self.content
            ),
            EntryVariants::Task => format!(
                "{important} {symbol} {content}",
                important = if self.important { "*" } else { " " },
                symbol = if self.completed { "X" } else { "\u{2022}" },
                content = self.content
            ),
            EntryVariants::Note => format!(
                "{important} {symbol} {content}",
                important = if self.important { "*" } else { " " },
                symbol = "-",
                content = self.content
            ),
        }
    }
}
