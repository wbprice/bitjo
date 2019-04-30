use chrono::Local;
use termion::{clear, color, style};

#[derive(Default, Clone)]
struct Event {
    important: bool,
    content: String,
}

#[derive(Default, Clone)]
struct Task {
    important: bool,
    completed: bool,
    cancelled: bool,
    content: String,
}

#[derive(Default, Clone)]
struct Note {
    important: bool,
    content: String,
}

trait JournalItem<T> {
    fn render(&self);
    fn new(content: String) -> T;
    fn toggle_important(&self) -> T;
    fn set_content(&self, content: String) -> T;
}

#[derive(Debug)]
enum JournalItems {
    Event,
    Task,
    Note
}

trait Completable<T> {
    fn toggle_completed(&self) -> T;
}

trait Cancellable<T> {
    fn toggle_cancellation(&self) -> T;
}

impl JournalItem<T> for JournalItems {

}

impl JournalItem<Event> for Event {
    fn render(&self) {
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
            important: !self.important,
            ..self.clone()
        }
    }

    fn set_content(&self, content: String) -> Event {
        Event {
            content: content.into(),
            ..self.clone()
        }
    }
}

impl Completable<Task> for Task {
    fn toggle_completed(&self) -> Task {
        Task {
            completed: !self.completed,
            ..self.clone()
        }
    }
}

impl JournalItem<Task> for Task {
    fn render(&self) {
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
            important: !self.important,
            ..self.clone()
        }
    }

    fn set_content(&self, content: String) -> Task {
        Task {
            content: content.into(),
            ..self.clone()
        }
    }
}

impl JournalItem<Note> for Note {
    fn render(&self) {
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
            important: !self.important,
            ..self.clone()
        }
    }

    fn set_content(&self, content: String) -> Note {
        Note {
            content: content.into(),
            ..self.clone()
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
    let important_task = completed_task
        .toggle_important()
        .set_content("An important, completed task".into());
    let event = Event::new("An event!".to_string());
    let note = Note::new("A note!".to_string());

    task.render();
    completed_task.render();
    important_task.render();
    event.render();
    note.render();

    let tasklist : Vec<JournalItems> = vec![
        JournalItems::Task::new("A task!".into()),
        JournalItems::Task::new("A completed task!".into()).toggle_completed(),
        JournalItems::Task::new("An important, completed task!".into()).toggle_completed().toggle_important(),
        JournalItems::Event::new("An event!".into()),
        JournalItems::Note::new("A note!".into())
    ];

    dbg!(tasklist);
}
