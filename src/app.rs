use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::journal::{EntryKind, Journal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Command,
    Journal,
    JournalCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add(EntryKind, String),
    SwitchToJournal,
    Quit,
    Complete,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandContext {
    CommandPane,
    JournalPane,
}

#[derive(Debug)]
pub struct App {
    pub journal: Journal,
    pub focus: Focus,
    pub command_input: String,
    pub mini_input: String,
    pub selected: Option<usize>,
    pub status: String,
    pub should_quit: bool,
}

impl App {
    pub fn new(journal: Journal) -> Self {
        let selected = last_entry_index(&journal);

        Self {
            journal,
            focus: Focus::Command,
            command_input: String::new(),
            mini_input: String::new(),
            selected,
            status: String::from("Ready."),
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.focus {
            Focus::Command => self.handle_command_key(key),
            Focus::Journal => self.handle_journal_key(key),
            Focus::JournalCommand => self.handle_journal_command_key(key),
        }
    }

    fn handle_command_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Enter => {
                let input = std::mem::take(&mut self.command_input);
                self.execute_command(&input, CommandContext::CommandPane)?;
            }
            KeyCode::Backspace => {
                self.command_input.pop();
            }
            KeyCode::Char(character) if is_text_input(key.modifiers) => {
                self.command_input.push(character);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_journal_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.focus = Focus::Command;
                self.status = String::from("Command pane focused.");
            }
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Char(':') if is_text_input(key.modifiers) => {
                self.mini_input = String::from(":");
                self.focus = Focus::JournalCommand;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_journal_command_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mini_input.clear();
                self.focus = Focus::Journal;
            }
            KeyCode::Enter => {
                let input = std::mem::take(&mut self.mini_input);
                self.execute_command(&input, CommandContext::JournalPane)?;
                if !self.should_quit {
                    self.focus = Focus::Journal;
                }
            }
            KeyCode::Backspace => {
                if self.mini_input.len() > 1 {
                    self.mini_input.pop();
                } else {
                    self.mini_input.clear();
                    self.focus = Focus::Journal;
                }
            }
            KeyCode::Char(character) if is_text_input(key.modifiers) => {
                self.mini_input.push(character);
            }
            _ => {}
        }

        Ok(())
    }

    fn execute_command(&mut self, input: &str, context: CommandContext) -> io::Result<()> {
        match parse_command(input) {
            Ok(Command::Add(kind, text)) => {
                if context != CommandContext::CommandPane {
                    self.status = String::from("Entry commands are available in the command pane.");
                    return Ok(());
                }

                self.journal.add_entry(kind, text);
                self.journal.save()?;
                self.selected = last_entry_index(&self.journal);
                self.status = format!("Wrote {}.", self.journal.path().display());
            }
            Ok(Command::SwitchToJournal) => {
                if self.journal.entries.is_empty() {
                    self.selected = None;
                    self.status = String::from("Journal pane focused; no entries yet.");
                } else {
                    self.selected.get_or_insert(self.journal.entries.len() - 1);
                    self.status = String::from("Journal pane focused.");
                }
                self.focus = Focus::Journal;
            }
            Ok(Command::Quit) => {
                self.should_quit = true;
            }
            Ok(Command::Complete) => {
                if context != CommandContext::JournalPane {
                    self.status = String::from("Complete is available in the journal pane.");
                    return Ok(());
                }
                self.complete_selected()?;
            }
            Ok(Command::Cancel) => {
                if context != CommandContext::JournalPane {
                    self.status = String::from("Cancel is available in the journal pane.");
                    return Ok(());
                }
                self.cancel_selected()?;
            }
            Err(message) => {
                self.status = message;
            }
        }

        Ok(())
    }

    fn complete_selected(&mut self) -> io::Result<()> {
        let Some(index) = self.valid_selected_index() else {
            self.status = String::from("No entry selected.");
            return Ok(());
        };

        match self.journal.entries[index].toggle_complete() {
            Ok(message) => {
                self.journal.save()?;
                self.status = message.to_string();
            }
            Err(message) => {
                self.status = message.to_string();
            }
        }

        Ok(())
    }

    fn cancel_selected(&mut self) -> io::Result<()> {
        let Some(index) = self.valid_selected_index() else {
            self.status = String::from("No entry selected.");
            return Ok(());
        };

        match self.journal.entries[index].toggle_cancel() {
            Ok(message) => {
                self.journal.save()?;
                self.status = message.to_string();
            }
            Err(message) => {
                self.status = message.to_string();
            }
        }

        Ok(())
    }

    fn valid_selected_index(&mut self) -> Option<usize> {
        if self.journal.entries.is_empty() {
            self.selected = None;
            return None;
        }

        let max = self.journal.entries.len() - 1;
        let index = self.selected.unwrap_or(max).min(max);
        self.selected = Some(index);
        Some(index)
    }

    fn select_previous(&mut self) {
        let Some(index) = self.valid_selected_index() else {
            return;
        };

        self.selected = Some(index.saturating_sub(1));
    }

    fn select_next(&mut self) {
        let Some(index) = self.valid_selected_index() else {
            return;
        };

        let max = self.journal.entries.len() - 1;
        self.selected = Some((index + 1).min(max));
    }
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(String::from("Enter a command."));
    }

    let (command, rest) = split_command(trimmed);
    match command {
        ":n" => entry_command(EntryKind::Note, rest),
        ":e" => entry_command(EntryKind::Event, rest),
        ":f" => entry_command(EntryKind::Feeling, rest),
        ":t" => entry_command(EntryKind::Task, rest),
        ":cw" => Ok(Command::SwitchToJournal),
        ":q" => Ok(Command::Quit),
        ":x" => Ok(Command::Complete),
        ":c" => Ok(Command::Cancel),
        _ => Err(format!("Unknown command: {command}")),
    }
}

fn split_command(input: &str) -> (&str, &str) {
    match input.find(char::is_whitespace) {
        Some(index) => (&input[..index], input[index..].trim()),
        None => (input, ""),
    }
}

fn entry_command(kind: EntryKind, text: &str) -> Result<Command, String> {
    if text.is_empty() {
        return Err(String::from("Entry text cannot be empty."));
    }

    Ok(Command::Add(kind, text.to_string()))
}

fn is_text_input(modifiers: KeyModifiers) -> bool {
    !modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
}

fn last_entry_index(journal: &Journal) -> Option<usize> {
    journal.entries.len().checked_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_entry_commands() {
        assert_eq!(
            parse_command(":n a note").unwrap(),
            Command::Add(EntryKind::Note, String::from("a note"))
        );
        assert_eq!(
            parse_command(":e Meeting at 10pm").unwrap(),
            Command::Add(EntryKind::Event, String::from("Meeting at 10pm"))
        );
        assert_eq!(
            parse_command(":f accomplished").unwrap(),
            Command::Add(EntryKind::Feeling, String::from("accomplished"))
        );
        assert_eq!(
            parse_command(":t ship MVP").unwrap(),
            Command::Add(EntryKind::Task, String::from("ship MVP"))
        );
    }

    #[test]
    fn parses_navigation_and_action_commands() {
        assert_eq!(parse_command(":cw").unwrap(), Command::SwitchToJournal);
        assert_eq!(parse_command(":q").unwrap(), Command::Quit);
        assert_eq!(parse_command(":x").unwrap(), Command::Complete);
        assert_eq!(parse_command(":c").unwrap(), Command::Cancel);
    }

    #[test]
    fn rejects_empty_entries_and_unknown_commands() {
        assert!(parse_command(":n").is_err());
        assert!(parse_command(":wat").is_err());
        assert!(parse_command("").is_err());
    }
}
