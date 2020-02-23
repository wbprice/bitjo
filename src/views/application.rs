use chrono::Local;
use std::io::stdout;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::{color, raw::IntoRawMode, style};

use crate::models::{Entry, JournalEntry};

#[derive(Copy, Clone)]
pub struct Cursor {
    col: u8,
    row: u8,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { col: 0, row: 0 }
    }
}

pub struct Application<'a> {
    pub stdout: RawTerminal<Stdout>,
    pub entries: &'a Vec<Entry>,
    pub cursor: Option<Cursor>,
}

impl<'a> Application<'a> {
    pub fn new(entries: &'a Vec<Entry>) -> Application<'a> {
        Application {
            cursor: None,
            stdout: stdout().into_raw_mode().unwrap(),
            entries,
        }
    }

    fn render_header_bar(&mut self) {
        writeln!(
            self.stdout,
            "{green}Bit Journal v0.1.1{reset}\r",
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        )
        .unwrap();
        writeln!(
            self.stdout,
            "{yellow}Today is {bold}{date}.{reset}\r",
            yellow = color::Fg(color::Yellow),
            bold = style::Bold,
            date = Local::now().format("%a, %b %e").to_string(),
            reset = color::Fg(color::Reset)
        )
        .unwrap();
    }

    fn render_tasks(&mut self) {
        for entry in self.entries.iter() {
            writeln!(self.stdout, "{}\r", entry.render()).unwrap();
        }
    }

    fn render_cursor(&mut self) {
        if let Some(cursor) = self.cursor {
            dbg!("render cursor");
        }
    }

    pub fn render(&mut self) {
        self.render_header_bar();
        self.render_tasks();
    }
}
