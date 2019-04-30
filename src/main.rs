use chrono::Local;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{clear, color, style};

#[derive(Default)]
struct Event {
    important: bool,
    content: String,
}

#[derive(Default)]
struct Task {
    important: bool,
    completed: bool,
    cancelled: bool,
    content: String,
}

#[derive(Default)]
struct Note {
    important: bool,
    content: String,
}

trait JournalItem<T> {
    fn print(&self);
    fn new(content: String) -> T;
    fn toggle_important(&self) -> T;
}

impl JournalItem<Event> for Event {
    fn print(&self) {
        println!(
            "{important} {symbol} {content}",
            important = if self.important { "*" } else { " " },
            symbol = "\u{26AC}",
            content = self.content
        )
    }

    fn new(content: String) -> Event {
        Event {
            content: content,
            ..Default::default()
        }
    }

    fn toggle_important(&self) -> Event {
        Event {
            content: self.content.clone(),
            important: !self.important,
            ..Default::default()
        }
    }
}

impl JournalItem<Task> for Task {
    fn print(&self) {
        println!(
            "{important} {symbol} {content}",
            important = if self.important { "*" } else { " " },
            symbol = if self.completed { "X" } else { "\u{2022}" },
            content = self.content
        )
    }

    fn new(content: String) -> Task {
        Task {
            content: content,
            ..Default::default()
        }
    }

    fn toggle_important(&self) -> Task {
        Task {
            content: self.content.clone(),
            important: !self.important,
            ..Default::default()
        }
    }
}

impl JournalItem<Note> for Note {
    fn print(&self) {
        println!(
            "{important} {symbol} {content}",
            important = if self.important { "*" } else { " " },
            symbol = "\u{2013}",
            content = self.content
        )
    }

    fn new(content: String) -> Note {
        Note {
            content: content,
            ..Default::default()
        }
    }

    fn toggle_important(&self) -> Note {
        Note {
            content: self.content.clone(),
            important: !self.important,
            ..Default::default()
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

    let task = Task::new("A task!".to_string());
    let mut completed_task = Task::new("A completed task!".to_string());
    completed_task.completed = true;
    let important_task = completed_task.toggle_important();
    let event = Event::new("An event!".to_string());
    let note = Note::new("A note!".to_string());

    task.print();
    completed_task.print();
    important_task.print();

    event.print();
    note.print();
}
