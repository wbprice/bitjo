use chrono::Local;
use termion::{color, style};

use crate::models::{Entry, JournalEntry};

#[derive(Debug)]
pub struct Application<'a> {
    pub entries: &'a Vec<Entry>,
}

impl<'a> Application<'a> {
    fn render_header_bar(&self) {
        println!(
            "{green}Bit Journal v0.1.1{reset}\r",
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        );
        println!(
            "{yellow}Today is {bold}{date}.{reset}\r",
            yellow = color::Fg(color::Yellow),
            bold = style::Bold,
            date = Local::now().format("%a, %b %e").to_string(),
            reset = color::Fg(color::Reset)
        );
    }

    fn render_tasks(&self) {
        for entry in self.entries.iter() {
            println!("{}\r", entry.render());
        }
    }

    pub fn render(&self) {
        self.render_header_bar();
        self.render_tasks();
    }
}
