use std::io::{Stdin, Stdout, Write};
use termion::{cursor, event::Key, input::TermRead};

pub struct TextArea;

impl TextArea {
    pub fn handle_input(&self, stdin: Stdin, stdout: &mut Stdout) {
        write!(stdout, "{}", cursor::Goto(1, 3)).unwrap();
        for c in stdin.keys() {
            // Print the key we type...
            match c.unwrap() {
                // Exit.
                Key::Char('q') => break,
                Key::Char(c) => println!("{}", c),
                Key::Alt(c) => println!("Alt-{}", c),
                Key::Ctrl(c) => println!("Ctrl-{}", c),
                Key::Left => println!("<left>"),
                Key::Right => println!("<right>"),
                Key::Up => println!("<up>"),
                Key::Down => println!("<down>"),
                _ => println!("Other"),
            }

            // Flush again.
            stdout.flush().unwrap();
        }
    }
}
