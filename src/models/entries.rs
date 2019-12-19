use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use termion::style::{
    CrossedOut,
    NoCrossedOut,
    Reset
};

#[derive(Debug, StructOpt)]
pub enum EntryType {
    Task { text: String },
    Note { text: String },
    Event { text: String },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Event {
    pub important: bool,
    pub content: String,
    pub cancelled: bool,
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
    pub cancelled: bool
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
    fn toggle_completed(&self) -> Entries;
    fn set_content(&self, content: String) -> Entries;
    fn toggle_cancelled(&self) -> Entries;
}

impl JournalEntry for Entries {
    fn render(&self) -> String {
        match self {
            Entries::Event(item) => format!(
                "{important} {crossed} {symbol} {content} {reset}",
                important = if item.important { "*" } else { " " },
                crossed = if item.cancelled { CrossedOut } else { None },
                symbol = "\u{26AC}",
                content = item.content,
                reset = Reset
            ),
            Entries::Task(item) => format!(
                "{important} {crossed} {symbol} {content} {reset}",
                important = if item.important { "*" } else { " " },
                crossed = if item.cancelled { CrossedOut } else { None },
                symbol = if item.completed { "X" } else { "\u{2022}" },
                content = item.content,
                reset = Reset
            ),
            Entries::Note(item) => format!(
                "{important} {crossed} {symbol} {content} {reset}",
                important = if item.important { "*" } else { " " },
                crossed = if item.cancelled { CrossedOut } else { None },
                symbol = "-",
                content = item.content,
                reset = Reset
            ),
        }
    }

    fn toggle_completed(&self) -> Entries {
        match self {
            Entries::Task(item) => Entries::Task(Task {
                completed: !item.completed,
                ..item.clone()
            }),
            Entries::Note(note) => Entries::Note(Note { ..note.clone() }),
            Entries::Event(event) => Entries::Event(Event { ..event.clone() }),
        }
    }

    fn set_content(&self, content: String) -> Entries {
        match self {
            Entries::Task(item) => Entries::Task(Task {
                content,
                ..item.clone()
            }),
            Entries::Note(note) => Entries::Note(Note {
                content,
                ..note.clone()
            }),
            Entries::Event(event) => Entries::Event(Event {
                content,
                ..event.clone()
            }),
        }
    }

    fn toggle_cancelled(&self) -> Entries {
        match self {
            Entries::Task(item) => Entries::Task(Task {
                cancelled: !item.completed,
                ..item.clone()
            }),
            Entries::Note(note) => Entries::Note(Note { 
                cancelled: !note.cancelled,
                ..note.clone() 
            }),
            Entries::Event(event) => Entries::Event(Event { 
                cancelled: !event.cancelled,
                ..event.clone() 
            }),
        }
    }
}
