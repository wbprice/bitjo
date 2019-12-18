use chrono::Local;
use std::io::{stdout, Stdout, Write};
use structopt::StructOpt;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, style};

mod services;
mod models;

use crate::{
    services::{
        InMemoryJournal
    },
    models::{
        Task,
        Event,
        Note,
        EntryType,
        Entries,
        JournalEntry
    }
};

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Command {
    Add {
        #[structopt(subcommand)]
        entry_type: EntryType,
    },
    Complete,
    Cancel,
    Remove,
}

#[derive(Debug)]
struct Application {
    entries: Vec<Entries>,
    mode: Modes,
}

impl Application {
    fn render_status_bar(&self, stdout: &mut RawTerminal<Stdout>) {
        writeln!(stdout, "{}", clear::All).unwrap();
        writeln!(
            stdout,
            "{green}Bit Journal v0.1.0{reset}\r",
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        )
        .unwrap();
        writeln!(
            stdout,
            "{yellow}Today is {bold}{date}.{reset}\r",
            yellow = color::Fg(color::Yellow),
            bold = style::Bold,
            date = Local::now().format("%a, %b %e").to_string(),
            reset = color::Fg(color::Reset)
        )
        .unwrap();
    }

    fn render_tasks(&self, stdout: &mut RawTerminal<Stdout>) {
        for entry in self.entries.iter() {
            writeln!(stdout, "{}\r", entry.render()).unwrap();
        }
    }

    fn render_header_bar(&self, stdout: &mut RawTerminal<Stdout>) {
        writeln!(
            stdout,
            "{background}{white}The current mode is {mode}{reset}{reset_bg}\r",
            background = color::Bg(color::Green),
            white = color::Fg(color::White),
            mode = self.mode.render(),
            reset = color::Fg(color::Reset),
            reset_bg = color::Bg(color::Reset)
        )
        .unwrap();
    }

    fn render(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        self.render_status_bar(&mut stdout);
        self.render_tasks(&mut stdout);
        self.render_header_bar(&mut stdout);
    }
}

#[derive(Debug)]
enum Modes {
    Normal,
    Insert,
}

impl Modes {
    fn render(&self) -> String {
        match self {
            Modes::Normal => "Normal".to_string(),
            Modes::Insert => "Insert".to_string(),
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    // Stubbing out behavior where this is serialized and persisted to a backend
    let mut entries = vec![
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

    // Handle input!
    if let Some(command) = opt.command {
        match command {
            Command::Add { entry_type } => match entry_type {
                EntryType::Event { text } => {
                    entries.push(Entries::Event(Event::new(text)));
                }
                EntryType::Note { text } => {
                    entries.push(Entries::Note(Note::new(text)));
                }
                EntryType::Task { text } => {
                    entries.push(Entries::Task(Task::new(text)));
                }
            },
            Command::Cancel => {
                unimplemented!();
            }
            Command::Complete => {
                unimplemented!();
            }
            Command::Remove => {
                unimplemented!();
            }
        }
    }

    let application = Application {
        mode: Modes::Normal,
        entries,
    };

    application.render();
}
