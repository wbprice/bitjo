use chrono::Local;
use termion::input::TermRead;
use termion::{clear, color, style};
use std::io::{self, Read, Write};


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
            })
        }
    }
}

#[derive(Debug)]
struct Application<R, W: Write> {
    entries: Vec<Entries>,
    mode: Modes,
    stdin: R,
    stdout: W
}

impl<R, W: Write> Application<R, W> {
    fn render_status_bar(&mut self) {
        writeln!(self.stdout, "{}", clear::All).unwrap();
        writeln!(
            self.stdout,
            "{green}Bit Journal v0.1.0{reset}\r",
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        ).unwrap();
        writeln!(
            self.stdout,
            "{yellow}Today is {bold}{date}.{reset}\r",
            yellow = color::Fg(color::Yellow),
            bold = style::Bold,
            date = Local::now().format("%a, %b %e").to_string(),
            reset = color::Fg(color::Reset)
        ).unwrap();
    }

    fn render_tasks(&mut self) {
        for entry in self.entries.iter() {
            writeln!(self.stdout, "{}\r", entry.render()).unwrap();
        }
    }

    fn render_header_bar(&mut self) {
        writeln!(
            self.stdout,
            "{background}{white}The current mode is {mode}{reset}{reset_bg}\r",
            background = color::Bg(color::Green),
            white = color::Fg(color::White),
            mode = self.mode.render(),
            reset = color::Fg(color::Reset),
            reset_bg = color::Bg(color::Reset)).unwrap();
    }

    fn switch_mode(&mut self, mode: Modes) {
        match mode {
            Modes::Normal => { self.mode = Modes::Normal; },
            Modes::Insert => { self.mode = Modes::Insert; },
            _ => ()
        }
    }

    fn on_keypress(&mut self) {

    }

    // A method for painting the entire screen.
    fn render(&mut self) {
        self.render_status_bar();
        self.render_tasks();
        self.render_header_bar();
    }

    // The application loop
    fn start(&mut self) {
        loop {
            self.render();
        }
    }
}

#[derive(Debug)]
enum Modes {
    Normal,
    Insert,
    Command
}

impl Modes {
    fn render(&self) -> String {
        match self {
            Modes::Normal => "Normal".to_string(),
            Modes::Insert => "Insert".to_string(),
            Modes::Command => "Command".to_string()
        }
    }
}


fn main() {

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut application = Application {
        mode: Modes::Normal,
        stdin: stdin.keys(),
        stdout: stdout,
        entries: vec![
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
        ]
    };

    application.start();

}
