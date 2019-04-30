use chrono::Local;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{clear, color, style};

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

    println!("\u{2022} a task!");
    println!("\u{0058} a completed task!");
    println!("\u{26AC} an event!");
    println!("\u{2013} a note!");

    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout, "Hey there.").unwrap();
}
