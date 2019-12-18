use chrono::Local;
use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, style};

use crate::{
    models::{
        Entries,
        JournalEntry
    }
};

#[derive(Debug)]
pub enum Modes {
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

#[derive(Debug)]
pub struct Application<'a> {
    pub entries: &'a Vec<Entries>,
    pub mode: Modes,
}

impl<'a> Application<'a> {
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

    pub fn render(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        self.render_status_bar(&mut stdout);
        self.render_tasks(&mut stdout);
        self.render_header_bar(&mut stdout);
    }
}