use chrono::Local;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;
use termion::{color, style};

use crate::models::{Entry, JournalEntry};

pub struct Application<'a> {
    pub stdout: RawTerminal<Stdout>,
    pub entries: &'a Vec<Entry>,
}

impl<'a> Application<'a> {
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

    pub fn render(&mut self) {
        self.render_header_bar();
        self.render_tasks();
    }
}
