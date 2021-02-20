use chrono::Local;
use std::io::{Stdout, Write};
use termion::{clear, color, cursor, style};

pub struct HeaderBar;

impl HeaderBar {
    pub fn render(&self, stdout: &mut Stdout) {
        write!(
            stdout,
            "{}{}{}",
            cursor::Goto(1, 1),
            clear::CurrentLine,
            cursor::Hide
        )
        .unwrap();
        write!(
            stdout,
            "{green}Bit Journal v0.1.1{reset}\r",
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        )
        .unwrap();
        write!(
            stdout,
            "{}{yellow}Today is {bold}{date}.{reset}\r",
            cursor::Goto(1, 2),
            yellow = color::Fg(color::Yellow),
            bold = style::Bold,
            date = Local::now().format("%a, %b %e").to_string(),
            reset = color::Fg(color::Reset)
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}
