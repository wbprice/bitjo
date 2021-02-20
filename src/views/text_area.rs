use std::io::{Stdin, Stdout, Write};
use cursor::BlinkingBar;
use termion::{cursor, event::Key, input::TermRead, clear};

use crate::models::{
    Entry,
    EntryVariants
};

#[derive(PartialEq)]
enum EditorMode {
    Normal,
    Insert,
}

pub struct TextArea {
    editor_mode: EditorMode,
    entry_buffer: Option<String>,
    entry_variant: Option<EntryVariants>,
    entries: Vec<Entry>
}

impl TextArea {

    pub fn new() -> Self {
        return TextArea {
            editor_mode: EditorMode::Normal,
            entry_buffer: Some(String::new()),
            entry_variant: None,
            entries: vec![]
        }
    }

    pub fn handle_input(&mut self, stdin: Stdin, stdout: &mut Stdout) {
        write!(stdout, "{}{}", cursor::Goto(1, 3), cursor::Show).unwrap();
        for c in stdin.keys() {
            match self.editor_mode {
                EditorMode::Insert => {
                    write!(stdout, "{}{}", cursor::Goto(1,3), cursor::BlinkingBar).unwrap();
                    match c.unwrap() {
                        Key::Esc => {
                            self.editor_mode = EditorMode::Normal;
                        }
                        Key::Backspace => {
                            if let Some(mut buffer) = self.entry_buffer.clone() {
                                buffer.pop();
                                self.entry_buffer = Some(buffer.clone());
                                write!(stdout, "{}{}", clear::CurrentLine, buffer).unwrap();
                            }
                        }
                        Key::Char(any_char) => {
                            if let Some(mut buffer) = self.entry_buffer.clone() {
                                buffer.push(any_char);
                                self.entry_buffer = Some(buffer.clone());
                                write!(stdout, "{}", buffer).unwrap();
                            }
                        }
                        _ => {
                            // noop
                        }
                    }
                },
                EditorMode::Normal => {
                    write!(stdout, "{}{}", cursor::Goto(1,3), cursor::SteadyBlock).unwrap();
                    match c.unwrap() {
                        Key::Char('q') => {
                            break;
                        }
                        // Append a note
                        Key::Char('n') => {
                            self.editor_mode = EditorMode::Insert;
                            self.entry_variant = Some(EntryVariants::Note);
                        },
                        // Append an event
                        Key::Char('e') => {
                            self.editor_mode = EditorMode::Insert;
                            self.entry_variant = Some(EntryVariants::Event);
                        },
                        // Append a todo
                        Key::Char('t') => {
                            self.editor_mode = EditorMode::Insert;
                            self.entry_variant = Some(EntryVariants::Task);
                        },
                        // Movement
                        Key::Char('j') => {
                            write!(stdout, "{}", cursor::Down(1)).unwrap(); // up
                        }
                        Key::Char('k') => {
                            write!(stdout, "{}", cursor::Up(1)).unwrap();
                        }
                        _ => {
                            // noop
                        }
                    }
                }
            }

            // Flush again.
            stdout.flush().unwrap();
        }
    }
}
