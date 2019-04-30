use chrono::Local;
use termion::{clear, color, style};

#[derive(Default, Clone)]
struct Event {
    important: bool,
    content: String,
}

#[derive(Default, Clone, Debug)]
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

trait Journalable<T> {
    fn new(content: String) -> T;
    fn set_content(&self, content: String) -> T;
    fn toggle_important(&self) -> T;
    fn render(&self) -> String;
}

trait Completable<T> : Journalable<T> {
    fn toggle_completed(&self) -> T;
}

trait Cancellable<T> : Journalable<T> {
    fn toggle_cancellation(&self) -> T;
}

impl Journalable<Task> for Task {
    fn new(content: String) -> Task {
        Task {
            content: content.into(),
            ..Default::default()
        }
    }

    fn set_content(&self, content: String) -> Task {
        Task {
            content: content.into(),
            ..self.clone()
        }
    }

    fn toggle_important(&self) -> Task {
        Task {
            important: !self.important,
            ..self.clone()
        }
    }

    fn render(&self) -> String {
        format!("{content}", content = self.content)
    }
}

impl Cancellable<Task> for Task {
    fn toggle_cancellation(&self) -> Task {
        Task {
            cancelled: !self.cancelled,
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

    let task = Task::new("Figure out enums".into()).toggle_important().toggle_completed();
    println!("{}", task.render());
    dbg!(task);
}