use chrono::Local;
use termion::{clear, color, style};

#[derive(Debug, Default, Clone)]
struct Event {
    important: bool,
    content: String,
}

#[derive(Debug, Default, Clone)]
struct Task {
    important: bool,
    completed: bool,
    cancelled: bool,
    content: String,
}

#[derive(Debug, Default, Clone)]
struct Note {
    important: bool,
    content: String,
}

#[derive(Debug)]
enum Entries {
    Event(Event),
    Task(Task),
    Note(Note)
}

trait Journalable {
    fn render(&self) -> String;
    fn toggle_completed(&self) -> Entries;
    fn set_content(&self, content: String) -> Entries;
}

impl Journalable for Entries {
    fn render(&self) -> String {
        match self {
            Entries::Event(item) => format!("{important} {symbol} {content}",
                important = if item.important { "*" } else {" "},
                symbol = "\u{26AC}",
                content = item.content),
            Entries::Task(item) => format!("{important} {symbol} {content}",
                important = if item.important { "*" } else {" "},
                symbol = if item.completed { "X" } else { "\u{2022}" },
                content = item.content),
            Entries::Note(item) => format!("{important} {symbol} {content}",
                important = if item.important { "*" } else {" "},
                symbol = "-",
                content = item.content),
        }
    }

    fn toggle_completed(&self) -> Entries {
        match self {
            Entries::Task(item) => Entries::Task(Task {
                completed: !item.completed,
                ..item.clone()
            }),
            Entries::Note(note) => Entries::Note(Note {
                ..note.clone()
            }),
            Entries::Event(event) => Entries::Event(Event {
                ..event.clone()
            })
        }
    }

    fn set_content(&self, content: String) -> Entries {
        match self {
            Entries::Task(item) => Entries::Task(Task {
                content: content.into(),
                ..item.clone()
            }),
            Entries::Note(note) => Entries::Note(Note {
                content: content.into(),
                ..note.clone()
            }),
            Entries::Event(event) => Entries::Event(Event {
                content: content.into(),
                ..event.clone()
            })
        }
    }
}

fn main() {
    println!("{}", clear::All);
    println!(
        "{green}Bit Journal v0.1.0{reset}",
        green = color::Fg(color::Green),
        reset = color::Fg(color::Reset)
    );
    println!(
        "{yellow}Today is {bold}{date}.{reset}",
        yellow = color::Fg(color::Yellow),
        bold = style::Bold,
        date = Local::now().format("%a, %b %e").to_string(),
        reset = color::Fg(color::Reset)
    );

    let memory : Vec<Entries> = vec![
        Entries::Event(Event {
            content: "Internal Standup at 4pm".into(),
            ..Default::default()
        }),
        Entries::Task(Task {
            content: "Figure out enums".into(),
            ..Default::default()
        }),
        Entries::Task(Task {
            content: "Take out the trash".into(),
            ..Default::default()
        })
        .toggle_completed()
        .set_content("Laugh uncontrollably".into()),
        Entries::Note(Note {
            content: "I'm just surprised this worked".into(),
            ..Default::default()
        }),
    ];

    for entry in memory {
        println!("{}", entry.render());
    }
}
