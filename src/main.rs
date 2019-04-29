use termion::{color, clear, style};
use chrono::{Utc};


fn main() {
    println!("{}", clear::All);
    println!("{green}Bit Journal v0.1.0{reset}",
           green = color::Fg(color::Green),
           reset = color::Fg(color::Reset));
    println!("{yellow}Today is {bold}{date}.{reset}",
            yellow = color::Fg(color::Yellow),
            bold = style::Bold,
            date = Utc::now().format("%a, %b %e").to_string(),
            reset = color::Fg(color::Reset));
}