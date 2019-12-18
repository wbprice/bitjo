use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum EntryType {
    Task { text: String },
    Note { text: String },
    Event { text: String },
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
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

#[derive(Debug)]
pub enum Entries {
    Event(Event),
    Task(Task),
    Note(Note),
}

pub trait JournalEntry {
    fn render(&self) -> String;
    fn toggle_completed(&self) -> Entries;
    fn set_content(&self, content: String) -> Entries;
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
}