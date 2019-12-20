use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Event {
    pub important: bool,
    pub content: String,
}

impl Event {
    pub fn new(content: String) -> Event {
        Event {
            content,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    pub important: bool,
    pub completed: bool,
    pub cancelled: bool,
    pub content: String,
}

impl Task {
    pub fn new(content: String) -> Task {
        Task {
            content,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Note {
    pub important: bool,
    pub content: String,
}

impl Note {
    pub fn new(content: String) -> Note {
        Note {
            content,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Entries {
    Event(Event),
    Task(Task),
    Note(Note),
}

pub trait JournalEntry {
    fn render(&self) -> String;
}

impl JournalEntry for Entries {
    fn render(&self) -> String {
        match self {
            Entries::Event(item) => format!(
                "{important} {symbol} {content}",
                important = if item.important { "*" } else { " " },
                symbol = "\u{26AC}",
                content = item.content
            ),
            Entries::Task(item) => format!(
                "{important} {symbol} {content}",
                important = if item.important { "*" } else { " " },
                symbol = if item.completed { "X" } else { "\u{2022}" },
                content = item.content
            ),
            Entries::Note(item) => format!(
                "{important} {symbol} {content}",
                important = if item.important { "*" } else { " " },
                symbol = "-",
                content = item.content
            ),
        }
    }
}
