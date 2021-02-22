use std::io::{Stdin, Stdout, Write};
use termion::{clear, cursor, event::Key, input::TermRead};

use crate::models::{Entry, EntryVariants, JournalEntry};

#[derive(PartialEq)]
enum EditorMode {
    Normal,
    Insert,
}

pub struct TextArea {
    editor_mode: EditorMode,
    entry_buffer: Option<String>,
    entry_variant: Option<EntryVariants>,
    entries: Vec<Entry>,
}

impl TextArea {
    pub fn new() -> Self {
        return TextArea {
            editor_mode: EditorMode::Normal,
            entry_buffer: Some(String::new()),
            entry_variant: None,
            entries: vec![],
        };
    }

    fn render_entries(&self, stdout: &mut Stdout) {
        for entry in self.entries.iter() {
            write!(stdout, "{}{}\n\r", clear::CurrentLine, entry.render()).unwrap();
        }
    }

    pub fn handle_input(&mut self, stdin: Stdin, stdout: &mut Stdout) {
        let mut changed_editor_mode = false;

        for c in stdin.keys() {
            let entry_count = self.entries.len() as u16;
            match self.editor_mode {
                EditorMode::Insert => {
                    // Perform any upfront setup
                    if changed_editor_mode {
                        write!(
                            stdout,
                            "{}{}{}",
                            cursor::Goto(1, 3 + entry_count),
                            cursor::Show,
                            cursor::BlinkingBar
                        )
                        .unwrap();
                        changed_editor_mode = false;
                    }

                    write!(
                        stdout,
                        "{}{}",
                        cursor::Goto(1, 3 + entry_count),
                        clear::CurrentLine
                    )
                    .unwrap();

                    match c.unwrap() {
                        Key::Esc => {
                            // Clear any item buffers
                            self.entry_buffer = None;
                            self.entry_variant = None;
                            // Switch the mode back to normal
                            self.editor_mode = EditorMode::Normal;
                            changed_editor_mode = true;
                        }
                        Key::Backspace => {
                            if let Some(mut buffer) = self.entry_buffer.clone() {
                                buffer.pop();
                                write!(stdout, "{}{}", clear::CurrentLine, &buffer).unwrap();
                                self.entry_buffer = Some(buffer);
                            }
                        }
                        Key::Char('\n') => {
                            // Commit the entry to the entries list
                            if let Some(entry_variant) = self.entry_variant.clone() {
                                if let Some(entry_buffer) = self.entry_buffer.clone() {
                                    self.entries.push(Entry::new(entry_variant, entry_buffer));
                                }
                            }

                            // Clear the text field
                            write!(stdout, "{}{}", cursor::Goto(1, 3), clear::AfterCursor).unwrap();

                            // Render any entries
                            self.render_entries(stdout);

                            // Clear any item buffers
                            self.entry_buffer = None;
                            self.entry_variant = None;
                            self.editor_mode = EditorMode::Normal;
                            changed_editor_mode = true;
                        }
                        Key::Char(any_char) => {
                            if let Some(mut buffer) = self.entry_buffer.clone() {
                                buffer.push(any_char);
                                write!(stdout, "{}", &buffer).unwrap();
                                self.entry_buffer = Some(buffer);
                            }
                        }
                        _ => {
                            // noop
                        }
                    }
                }
                EditorMode::Normal => {
                    // Perform any upfront setup
                    if changed_editor_mode {
                        write!(stdout, "{}{}", cursor::Goto(1, 3), cursor::SteadyBlock).unwrap();
                        changed_editor_mode = false;
                    }

                    match c.unwrap() {
                        Key::Char('q') => {
                            break;
                        }
                        // Append a note
                        Key::Char('n') => {
                            self.editor_mode = EditorMode::Insert;
                            self.entry_variant = Some(EntryVariants::Note);
                            changed_editor_mode = true;
                        }
                        // Append an event
                        Key::Char('e') => {
                            self.editor_mode = EditorMode::Insert;
                            self.entry_variant = Some(EntryVariants::Event);
                            changed_editor_mode = true;
                        }
                        // Append a todo
                        Key::Char('t') => {
                            self.editor_mode = EditorMode::Insert;
                            self.entry_variant = Some(EntryVariants::Task);
                            changed_editor_mode = true;
                        }
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
