use chrono::Local;
use termion::{clear, color, style};


#[derive(Default, Clone, Debug)]
struct Task {
    important: bool,
    completed: bool,
    cancelled: bool,
    content: String,
}

enum JournalItems {
    Task
}

trait Journalable {
    fn new(content: &'static str) -> Self;
    fn render(&self) -> String;
}

impl Journalable for Task {
    fn new(content: &'static str) -> Task {
        Task {
            content: content.into(),
            ..Default::default()
        }
    }

    fn render(&self) -> String {
        format!("{}", self.content)
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

    let mut memory : Vec<JournalItems> = vec![];
    let task = Task::new("Meeting at 5:30pm".into());
    memory.push(task);

    for task in memory.iter() {
        println!("{}", task.render());
    }
}