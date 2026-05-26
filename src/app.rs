use std::{
    io,
    path::{Path, PathBuf},
};

use chrono::{Days, NaiveDate};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::journal::{EntryKind, Journal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Command,
    Journal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPaneMode {
    Normal,
    Search,
    Entry(CommandAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAction {
    Add(EntryKind),
    Quit,
    Split,
    Complete,
    Cancel,
    Important,
}

impl CommandAction {
    fn token(self) -> Option<&'static str> {
        match self {
            CommandAction::Add(EntryKind::Note) => Some(":n"),
            CommandAction::Add(EntryKind::Event) => Some(":e"),
            CommandAction::Add(EntryKind::Feeling) => Some(":f"),
            CommandAction::Add(EntryKind::Task) => Some(":t"),
            CommandAction::Add(EntryKind::Raw) => None,
            CommandAction::Quit => Some(":q"),
            CommandAction::Split => Some(":split"),
            CommandAction::Complete => Some(":x"),
            CommandAction::Cancel => Some(":c"),
            CommandAction::Important => Some(":i"),
        }
    }

    fn entry_title(self) -> &'static str {
        match self {
            CommandAction::Add(EntryKind::Note) => "Enter a note",
            CommandAction::Add(EntryKind::Event) => "Enter an event",
            CommandAction::Add(EntryKind::Feeling) => "Enter a feeling",
            CommandAction::Add(EntryKind::Task) => "Enter a task",
            _ => "Command",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add(EntryKind, String),
    Quit,
    ToggleSplit,
    Complete,
    Cancel,
    Important,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandContext {
    CommandPane,
    JournalPane,
}

#[derive(Debug, Clone, Copy)]
struct CommandOption {
    name: &'static str,
    token: &'static str,
    aliases: &'static [&'static str],
    action: CommandAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandSearchResult {
    pub name: &'static str,
    pub token: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitPane {
    Older,
    Newer,
}

#[derive(Debug, Clone)]
pub struct JournalPane {
    pub journal: Journal,
    pub selected: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SplitJournalView {
    pub older: JournalPane,
    pub newer: JournalPane,
    pub active: SplitPane,
}

impl JournalPane {
    fn new(journal: Journal) -> Self {
        let selected = last_entry_index(&journal);

        Self { journal, selected }
    }
}

impl SplitJournalView {
    pub fn pane(&self, pane: SplitPane) -> &JournalPane {
        match pane {
            SplitPane::Older => &self.older,
            SplitPane::Newer => &self.newer,
        }
    }

    fn pane_mut(&mut self, pane: SplitPane) -> &mut JournalPane {
        match pane {
            SplitPane::Older => &mut self.older,
            SplitPane::Newer => &mut self.newer,
        }
    }

    pub fn active_pane(&self) -> &JournalPane {
        self.pane(self.active)
    }

    fn active_pane_mut(&mut self) -> &mut JournalPane {
        self.pane_mut(self.active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MatchQuality {
    Prefix,
    Subsequence,
}

const COMMAND_SEARCH_RESULT_LIMIT: usize = 5;

const COMMAND_PANE_OPTIONS: &[CommandOption] = &[
    CommandOption {
        name: "note",
        token: ":n",
        aliases: &["n", "new note"],
        action: CommandAction::Add(EntryKind::Note),
    },
    CommandOption {
        name: "event",
        token: ":e",
        aliases: &["e", "calendar"],
        action: CommandAction::Add(EntryKind::Event),
    },
    CommandOption {
        name: "feeling",
        token: ":f",
        aliases: &["f", "mood"],
        action: CommandAction::Add(EntryKind::Feeling),
    },
    CommandOption {
        name: "task",
        token: ":t",
        aliases: &["t", "todo"],
        action: CommandAction::Add(EntryKind::Task),
    },
    CommandOption {
        name: "quit",
        token: ":q",
        aliases: &["q", "exit"],
        action: CommandAction::Quit,
    },
    CommandOption {
        name: "split",
        token: ":split",
        aliases: &["compare", "dual"],
        action: CommandAction::Split,
    },
];

const COMPLETE_COMMAND_OPTION: CommandOption = CommandOption {
    name: "complete",
    token: ":x",
    aliases: &["x", "done"],
    action: CommandAction::Complete,
};

const CANCEL_COMMAND_OPTION: CommandOption = CommandOption {
    name: "cancel",
    token: ":c",
    aliases: &["c", "cancelled"],
    action: CommandAction::Cancel,
};

const IMPORTANT_COMMAND_OPTION: CommandOption = CommandOption {
    name: "important",
    token: ":i",
    aliases: &["i"],
    action: CommandAction::Important,
};

#[derive(Debug)]
pub struct App {
    pub journal: Journal,
    pub focus: Focus,
    pub command_mode: CommandPaneMode,
    pub command_input: String,
    pub command_result_index: usize,
    pub selected: Option<usize>,
    pub status: String,
    pub should_quit: bool,
    journal_root: PathBuf,
    today: NaiveDate,
    split: Option<SplitJournalView>,
    command_context: CommandContext,
}

// App construction and top-level input routing.
impl App {
    pub fn new(journal: Journal) -> Self {
        let selected = last_entry_index(&journal);
        let journal_root = journal_root(&journal);
        let today = journal.date;

        Self {
            journal,
            focus: Focus::Journal,
            command_mode: CommandPaneMode::Normal,
            command_input: String::new(),
            command_result_index: 0,
            selected,
            status: String::from("Ready."),
            should_quit: false,
            journal_root,
            today,
            split: None,
            command_context: CommandContext::CommandPane,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.focus {
            Focus::Command => self.handle_command_key(key),
            Focus::Journal => self.handle_journal_key(key),
        }
    }

    fn handle_command_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.command_mode {
            CommandPaneMode::Normal => self.handle_normal_command_key(key),
            CommandPaneMode::Search => self.handle_command_search_key(key),
            CommandPaneMode::Entry(_) => self.handle_command_entry_key(key),
        }
    }

    fn handle_normal_command_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char(':') if is_text_input(key.modifiers) => {
                self.open_command_search(CommandContext::CommandPane);
            }
            KeyCode::Esc => self.focus_journal(),
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

    fn handle_command_search_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.reset_command_pane();
                self.focus_journal();
            }
            KeyCode::Enter => {
                if self.execute_command_from_search_input()? {
                    return Ok(());
                }

                let matches = self.matching_command_options();
                let Some(command) = matches.get(self.command_result_index).copied() else {
                    self.status = String::from("No matching commands.");
                    return Ok(());
                };

                self.select_command(command)?;
            }
            KeyCode::Backspace => {
                self.command_input.pop();
                self.normalize_command_result_index();
                self.status = String::from("Search commands.");
            }
            KeyCode::Up => self.select_previous_command_result(),
            KeyCode::Down => self.select_next_command_result(),
            KeyCode::Char(character) if is_text_input(key.modifiers) => {
                self.command_input.push(character);
                self.normalize_command_result_index();
                self.status = String::from("Search commands.");
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_command_entry_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.reset_command_pane();
                self.focus_journal();
            }
            KeyCode::Enter => self.submit_selected_command_entry()?,
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
            KeyCode::Esc => self.focus_journal(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Char('k') if is_unmodified_key(key.modifiers) => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Char('j') if is_unmodified_key(key.modifiers) => self.select_next(),
            KeyCode::Left => self.navigate_left()?,
            KeyCode::Char('h') if is_unmodified_key(key.modifiers) => self.navigate_left()?,
            KeyCode::Right => self.navigate_right()?,
            KeyCode::Char('l') if is_unmodified_key(key.modifiers) => self.navigate_right()?,
            KeyCode::Char(character) if opens_journal_command_search(character, key.modifiers) => {
                self.open_command_search(CommandContext::JournalPane);
            }
            _ => {}
        }

        Ok(())
    }
}

// Journal focus and date navigation.
impl App {
    fn navigate_left(&mut self) -> io::Result<()> {
        if self.split.is_some() {
            self.navigate_split_left()
        } else {
            self.switch_to_previous_day();
            Ok(())
        }
    }

    fn navigate_right(&mut self) -> io::Result<()> {
        if self.split.is_some() {
            self.navigate_split_right()
        } else {
            self.switch_to_next_day();
            Ok(())
        }
    }

    fn switch_to_previous_day(&mut self) {
        let Some(date) = self.journal.date.checked_sub_days(Days::new(1)) else {
            self.status = String::from("Cannot switch before the supported date range.");
            return;
        };

        self.switch_to_day(date);
    }

    fn switch_to_next_day(&mut self) {
        let Some(date) = self.journal.date.checked_add_days(Days::new(1)) else {
            self.status = String::from("Cannot switch after the supported date range.");
            return;
        };

        self.switch_to_day(date);
    }

    fn switch_to_day(&mut self, date: chrono::NaiveDate) {
        match Journal::load_for_date(&self.journal_root, date) {
            Ok(journal) => {
                self.journal = journal;
                self.selected = last_entry_index(&self.journal);
                self.reset_command_pane();
                self.focus = Focus::Journal;
                self.status = format!("Loaded {}.", self.journal.date.format("%Y-%m-%d"));
            }
            Err(error) => {
                self.status = format!("Could not load {}: {error}", date.format("%Y-%m-%d"));
            }
        }
    }

    fn open_command_search(&mut self, context: CommandContext) {
        self.focus = Focus::Command;
        self.command_input.clear();
        self.command_mode = CommandPaneMode::Search;
        self.command_result_index = 0;
        self.command_context = context;
        self.status = String::from("Search commands.");
    }

    fn focus_journal(&mut self) {
        self.focus = Focus::Journal;
        self.status = String::from("Journal pane focused.");
    }
}

// Command selection and execution.
impl App {
    fn execute_command_from_search_input(&mut self) -> io::Result<bool> {
        if let Some((input, context)) =
            command_from_search_input(&self.command_input, self.command_context)
        {
            self.reset_command_pane();
            self.execute_command(&input, context)?;
            return Ok(true);
        }

        Ok(false)
    }

    fn select_command(&mut self, command: &CommandOption) -> io::Result<()> {
        self.command_input.clear();
        self.command_result_index = 0;

        match command.action {
            CommandAction::Add(_) => {
                self.command_mode = CommandPaneMode::Entry(command.action);
                self.status = format!("Selected {}.", command.name);
            }
            CommandAction::Quit => {
                self.reset_command_pane();
                self.execute_command(command.token, CommandContext::CommandPane)?;
            }
            CommandAction::Split => {
                self.reset_command_pane();
                self.execute_command(command.token, CommandContext::CommandPane)?;
            }
            CommandAction::Complete | CommandAction::Cancel | CommandAction::Important => {
                self.reset_command_pane();
                self.execute_command(command.token, CommandContext::JournalPane)?;
            }
        }

        Ok(())
    }

    fn submit_selected_command_entry(&mut self) -> io::Result<()> {
        let CommandPaneMode::Entry(action) = self.command_mode else {
            return Ok(());
        };

        if self.command_input.trim().is_empty() {
            self.status = String::from("Entry text cannot be empty.");
            return Ok(());
        }

        let Some(token) = action.token() else {
            return Ok(());
        };

        let input = format!("{token} {}", self.command_input.trim());
        self.reset_command_pane();
        self.execute_command(&input, CommandContext::CommandPane)
    }

    fn execute_command(&mut self, input: &str, context: CommandContext) -> io::Result<()> {
        match parse_command(input) {
            Ok(Command::Add(kind, text)) => {
                if context != CommandContext::CommandPane {
                    self.status = String::from("Entry commands are available in the command pane.");
                    return Ok(());
                }

                let path = self.add_entry_to_active_journal(kind, text)?;
                self.status = format!("Wrote {}.", path.display());
                if kind == EntryKind::Note {
                    self.focus = Focus::Journal;
                }
            }
            Ok(Command::Quit) => {
                self.should_quit = true;
            }
            Ok(Command::ToggleSplit) => {
                if context != CommandContext::CommandPane {
                    self.status = String::from("Split is available in the command pane.");
                    return Ok(());
                }
                self.toggle_split_view();
            }
            Ok(Command::Complete) => {
                if context != CommandContext::JournalPane {
                    self.status = String::from("Complete is available in the journal pane.");
                    return Ok(());
                }
                self.complete_selected()?;
                self.focus = Focus::Journal;
            }
            Ok(Command::Cancel) => {
                if context != CommandContext::JournalPane {
                    self.status = String::from("Cancel is available in the journal pane.");
                    return Ok(());
                }
                self.cancel_selected()?;
                self.focus = Focus::Journal;
            }
            Ok(Command::Important) => {
                if context != CommandContext::JournalPane {
                    self.status = String::from("Important is available in the journal pane.");
                    return Ok(());
                }
                self.toggle_important_selected()?;
                self.focus = Focus::Journal;
            }
            Err(message) => {
                self.status = message;
            }
        }

        Ok(())
    }
}

// Split journal view management.
impl App {
    fn toggle_split_view(&mut self) {
        if let Some(split) = self.split.take() {
            let pane = match split.active {
                SplitPane::Older => split.older,
                SplitPane::Newer => split.newer,
            };
            self.journal = pane.journal;
            self.selected = pane.selected;
            self.focus = Focus::Journal;
            self.status = format!(
                "Split view off. Loaded {}.",
                self.journal.date.format("%Y-%m-%d")
            );
            return;
        }

        let Some(older_date) = self.today.checked_sub_days(Days::new(1)) else {
            self.status = String::from("Cannot split before the supported date range.");
            return;
        };

        match self.load_split_window(older_date, SplitPane::Newer) {
            Ok(split) => {
                self.split = Some(split);
                self.sync_active_journal_from_split();
                self.focus = Focus::Journal;
                if let Some(split) = &self.split {
                    self.status = format!(
                        "Split view on: {} and {}.",
                        split.older.journal.date.format("%Y-%m-%d"),
                        split.newer.journal.date.format("%Y-%m-%d")
                    );
                }
            }
            Err(error) => {
                self.status = format!("Could not load split view: {error}");
            }
        }
    }

    fn load_split_window(
        &self,
        older_date: NaiveDate,
        active: SplitPane,
    ) -> io::Result<SplitJournalView> {
        let newer_date = older_date.checked_add_days(Days::new(1)).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot split after the supported date range.",
            )
        })?;

        Ok(SplitJournalView {
            older: JournalPane::new(Journal::load_for_date(&self.journal_root, older_date)?),
            newer: JournalPane::new(Journal::load_for_date(&self.journal_root, newer_date)?),
            active,
        })
    }

    fn navigate_split_left(&mut self) -> io::Result<()> {
        let Some(split) = &self.split else {
            return Ok(());
        };

        if split.active == SplitPane::Newer {
            self.set_active_split_pane(SplitPane::Older);
            return Ok(());
        }

        let Some(new_older_date) = split.older.journal.date.checked_sub_days(Days::new(1)) else {
            self.status = String::from("Cannot switch before the supported date range.");
            return Ok(());
        };

        let new_older = match Journal::load_for_date(&self.journal_root, new_older_date) {
            Ok(journal) => JournalPane::new(journal),
            Err(error) => {
                self.status = format!(
                    "Could not load {}: {error}",
                    new_older_date.format("%Y-%m-%d")
                );
                return Ok(());
            }
        };

        if let Some(split) = &mut self.split {
            let previous_older = std::mem::replace(&mut split.older, new_older);
            split.newer = previous_older;
            split.active = SplitPane::Older;
        }

        self.sync_active_journal_from_split();
        self.status = format!("Loaded {}.", self.journal.date.format("%Y-%m-%d"));
        Ok(())
    }

    fn navigate_split_right(&mut self) -> io::Result<()> {
        let Some(split) = &self.split else {
            return Ok(());
        };

        if split.active == SplitPane::Older {
            self.set_active_split_pane(SplitPane::Newer);
            return Ok(());
        }

        let Some(new_newer_date) = split.newer.journal.date.checked_add_days(Days::new(1)) else {
            self.status = String::from("Cannot switch after the supported date range.");
            return Ok(());
        };

        let new_newer = match Journal::load_for_date(&self.journal_root, new_newer_date) {
            Ok(journal) => JournalPane::new(journal),
            Err(error) => {
                self.status = format!(
                    "Could not load {}: {error}",
                    new_newer_date.format("%Y-%m-%d")
                );
                return Ok(());
            }
        };

        if let Some(split) = &mut self.split {
            let previous_newer = std::mem::replace(&mut split.newer, new_newer);
            split.older = previous_newer;
            split.active = SplitPane::Newer;
        }

        self.sync_active_journal_from_split();
        self.status = format!("Loaded {}.", self.journal.date.format("%Y-%m-%d"));
        Ok(())
    }

    fn set_active_split_pane(&mut self, active: SplitPane) {
        if let Some(split) = &mut self.split {
            split.active = active;
        }

        self.sync_active_journal_from_split();
        self.status = format!("Focused {}.", self.journal.date.format("%Y-%m-%d"));
    }
}

// Journal entry mutation and selection.
impl App {
    fn add_entry_to_active_journal(
        &mut self,
        kind: EntryKind,
        text: String,
    ) -> io::Result<PathBuf> {
        if let Some(split) = &mut self.split {
            let pane = split.active_pane_mut();
            pane.journal.add_entry(kind, text);
            pane.journal.save()?;
            pane.selected = last_entry_index(&pane.journal);
            let path = pane.journal.path().to_path_buf();
            self.sync_active_journal_from_split();
            return Ok(path);
        }

        self.journal.add_entry(kind, text);
        self.journal.save()?;
        self.selected = last_entry_index(&self.journal);
        Ok(self.journal.path().to_path_buf())
    }

    fn complete_selected(&mut self) -> io::Result<()> {
        let Some(index) = self.highlighted_entry_index() else {
            self.status = String::from("No entry selected.");
            return Ok(());
        };

        if let Some(split) = &mut self.split {
            let pane = split.active_pane_mut();
            self.status = match pane.journal.entries[index].toggle_complete() {
                Ok(message) => {
                    pane.journal.save()?;
                    message.to_string()
                }
                Err(message) => message.to_string(),
            };
            self.sync_active_journal_from_split();
            return Ok(());
        }

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
        let Some(index) = self.highlighted_entry_index() else {
            self.status = String::from("No entry selected.");
            return Ok(());
        };

        if let Some(split) = &mut self.split {
            let pane = split.active_pane_mut();
            self.status = match pane.journal.entries[index].toggle_cancel() {
                Ok(message) => {
                    pane.journal.save()?;
                    message.to_string()
                }
                Err(message) => message.to_string(),
            };
            self.sync_active_journal_from_split();
            return Ok(());
        }

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

    fn toggle_important_selected(&mut self) -> io::Result<()> {
        let Some(index) = self.highlighted_entry_index() else {
            self.status = String::from("No entry selected.");
            return Ok(());
        };

        if let Some(split) = &mut self.split {
            let pane = split.active_pane_mut();
            self.status = pane.journal.entries[index].toggle_important().to_string();
            pane.journal.save()?;
            self.sync_active_journal_from_split();
            return Ok(());
        }

        self.status = self.journal.entries[index].toggle_important().to_string();
        self.journal.save()?;

        Ok(())
    }

    fn highlighted_entry_index(&self) -> Option<usize> {
        let index = self.active_selected()?;
        (index < self.active_journal().entries.len()).then_some(index)
    }

    fn valid_selected_index(&mut self) -> Option<usize> {
        let entry_count = self.active_journal().entries.len();
        if entry_count == 0 {
            self.set_active_selected(None);
            return None;
        }

        let max = entry_count - 1;
        let index = self.active_selected().unwrap_or(max).min(max);
        self.set_active_selected(Some(index));
        Some(index)
    }

    fn select_previous(&mut self) {
        let Some(index) = self.valid_selected_index() else {
            return;
        };

        self.set_active_selected(Some(index.saturating_sub(1)));
    }

    fn select_next(&mut self) {
        let Some(index) = self.valid_selected_index() else {
            return;
        };

        let max = self.active_journal().entries.len() - 1;
        self.set_active_selected(Some((index + 1).min(max)));
    }

    fn active_journal(&self) -> &Journal {
        self.split
            .as_ref()
            .map(|split| &split.active_pane().journal)
            .unwrap_or(&self.journal)
    }

    fn active_selected(&self) -> Option<usize> {
        self.split
            .as_ref()
            .map(|split| split.active_pane().selected)
            .unwrap_or(self.selected)
    }

    fn set_active_selected(&mut self, selected: Option<usize>) {
        if let Some(split) = &mut self.split {
            split.active_pane_mut().selected = selected;
            self.sync_active_journal_from_split();
        } else {
            self.selected = selected;
        }
    }

    fn sync_active_journal_from_split(&mut self) {
        if let Some(split) = &self.split {
            self.journal = split.active_pane().journal.clone();
            self.selected = split.active_pane().selected;
        }
    }
}

// UI-facing state for split panes and command search.
impl App {
    pub fn split_view(&self) -> Option<&SplitJournalView> {
        self.split.as_ref()
    }

    fn normalize_command_result_index(&mut self) {
        let count = self.matching_command_options().len();
        if count == 0 {
            self.command_result_index = 0;
        } else {
            self.command_result_index = self.command_result_index.min(count - 1);
        }
    }

    fn select_previous_command_result(&mut self) {
        self.normalize_command_result_index();
        self.command_result_index = self.command_result_index.saturating_sub(1);
    }

    fn select_next_command_result(&mut self) {
        self.normalize_command_result_index();
        let count = self.matching_command_options().len();
        if count > 0 {
            self.command_result_index = (self.command_result_index + 1).min(count - 1);
        }
    }

    fn reset_command_pane(&mut self) {
        self.command_mode = CommandPaneMode::Normal;
        self.command_input.clear();
        self.command_result_index = 0;
        self.command_context = CommandContext::CommandPane;
    }

    pub fn command_title(&self) -> &'static str {
        match self.command_mode {
            CommandPaneMode::Normal => "Command",
            CommandPaneMode::Search => "Search Commands",
            CommandPaneMode::Entry(action) => action.entry_title(),
        }
    }

    pub fn visible_command_search_results(&self) -> Vec<(usize, CommandSearchResult)> {
        let results = self.command_search_results();
        if results.is_empty() {
            return Vec::new();
        }

        let selected = self.command_result_index.min(results.len() - 1);
        let start = selected
            .saturating_add(1)
            .saturating_sub(COMMAND_SEARCH_RESULT_LIMIT);

        results
            .into_iter()
            .enumerate()
            .skip(start)
            .take(COMMAND_SEARCH_RESULT_LIMIT)
            .collect()
    }

    pub fn command_search_result_limit(&self) -> usize {
        COMMAND_SEARCH_RESULT_LIMIT
    }

    pub fn command_search_input_is_exact_command(&self) -> bool {
        command_from_search_input(&self.command_input, self.command_context).is_some()
    }

    fn command_search_results(&self) -> Vec<CommandSearchResult> {
        self.matching_command_options()
            .into_iter()
            .map(|command| CommandSearchResult {
                name: command.name,
                token: command.token,
            })
            .collect()
    }

    fn matching_command_options(&self) -> Vec<&'static CommandOption> {
        let options = self.available_command_options();
        matching_command_options(&self.command_input, &options)
    }

    fn available_command_options(&self) -> Vec<&'static CommandOption> {
        let mut options = self.entry_action_command_options();
        options.extend(COMMAND_PANE_OPTIONS.iter());
        options
    }

    fn entry_action_command_options(&self) -> Vec<&'static CommandOption> {
        if self.command_context != CommandContext::JournalPane {
            return Vec::new();
        }

        let Some(index) = self.highlighted_entry_index() else {
            return Vec::new();
        };

        let entry = &self.active_journal().entries[index];
        match entry.kind {
            EntryKind::Task => {
                let mut options = vec![&COMPLETE_COMMAND_OPTION];
                if entry.state != crate::journal::EntryState::Completed {
                    options.push(&CANCEL_COMMAND_OPTION);
                }
                options.push(&IMPORTANT_COMMAND_OPTION);
                options
            }
            EntryKind::Event => vec![&CANCEL_COMMAND_OPTION, &IMPORTANT_COMMAND_OPTION],
            EntryKind::Note | EntryKind::Feeling | EntryKind::Raw => {
                vec![&IMPORTANT_COMMAND_OPTION]
            }
        }
    }
}

pub fn command_search_results(query: &str) -> Vec<CommandSearchResult> {
    let options = COMMAND_PANE_OPTIONS.iter().collect::<Vec<_>>();
    matching_command_options(query, &options)
        .into_iter()
        .map(|command| CommandSearchResult {
            name: command.name,
            token: command.token,
        })
        .collect()
}

fn matching_command_options(
    query: &str,
    options: &[&'static CommandOption],
) -> Vec<&'static CommandOption> {
    let mut matches = options
        .iter()
        .enumerate()
        .filter_map(|(index, &command)| {
            command_match_quality(query, command).map(|quality| (quality, index, command))
        })
        .collect::<Vec<_>>();

    matches.sort_by_key(|(quality, index, _)| (*quality, *index));
    matches.into_iter().map(|(_, _, command)| command).collect()
}

fn command_match_quality(query: &str, command: &CommandOption) -> Option<MatchQuality> {
    let query = query.trim();
    if query.is_empty() {
        return Some(MatchQuality::Prefix);
    }

    let mut quality = None;
    for candidate in std::iter::once(command.name)
        .chain(std::iter::once(command.token))
        .chain(command.aliases.iter().copied())
    {
        if starts_with_ignore_ascii_case(candidate, query) {
            return Some(MatchQuality::Prefix);
        }

        if fuzzy_subsequence_match(query, candidate) {
            quality = Some(MatchQuality::Subsequence);
        }
    }

    quality
}

pub fn fuzzy_subsequence_match(query: &str, candidate: &str) -> bool {
    let query = query.trim();
    if query.is_empty() {
        return true;
    }

    let mut candidate_chars = candidate
        .chars()
        .map(|character| character.to_ascii_lowercase());

    'query: for query_char in query
        .chars()
        .map(|character| character.to_ascii_lowercase())
    {
        for candidate_char in candidate_chars.by_ref() {
            if candidate_char == query_char {
                continue 'query;
            }
        }

        return false;
    }

    true
}

fn starts_with_ignore_ascii_case(candidate: &str, query: &str) -> bool {
    candidate
        .to_ascii_lowercase()
        .starts_with(&query.to_ascii_lowercase())
}

fn command_from_search_input(
    query: &str,
    context: CommandContext,
) -> Option<(String, CommandContext)> {
    let query = query.trim();
    if query.is_empty() {
        return None;
    }

    let input = format!(":{query}");
    match parse_command(&input).ok()? {
        Command::Add(_, _) | Command::Quit | Command::ToggleSplit => {
            Some((input, CommandContext::CommandPane))
        }
        Command::Complete | Command::Cancel | Command::Important
            if context == CommandContext::JournalPane =>
        {
            Some((input, CommandContext::JournalPane))
        }
        Command::Complete | Command::Cancel | Command::Important => None,
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
        ":q" => Ok(Command::Quit),
        ":split" => Ok(Command::ToggleSplit),
        ":x" => Ok(Command::Complete),
        ":c" => Ok(Command::Cancel),
        ":i" | ":important" => Ok(Command::Important),
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

fn opens_journal_command_search(character: char, modifiers: KeyModifiers) -> bool {
    matches!(character, ':' | ' ') && is_text_input(modifiers)
}

fn is_unmodified_key(modifiers: KeyModifiers) -> bool {
    modifiers == KeyModifiers::NONE
}

fn last_entry_index(journal: &Journal) -> Option<usize> {
    journal.entries.len().checked_sub(1)
}

fn journal_root(journal: &Journal) -> PathBuf {
    journal
        .path()
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env, fs, io,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use chrono::NaiveDate;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 5, 21).unwrap()
    }

    fn test_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        env::temp_dir().join(format!("bullet-journal-tui-app-test-{unique}"))
    }

    fn test_app() -> io::Result<(App, PathBuf)> {
        let root = test_root();
        let journal = Journal::load_for_date(&root, date())?;
        Ok((App::new(journal), root))
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn type_text(app: &mut App, text: &str) -> io::Result<()> {
        for character in text.chars() {
            app.handle_key(key(KeyCode::Char(character)))?;
        }

        Ok(())
    }

    fn search_result_names(app: &App) -> Vec<&'static str> {
        app.visible_command_search_results()
            .into_iter()
            .map(|(_, result)| result.name)
            .collect()
    }

    fn run_journal_search(app: &mut App, input: &str) -> io::Result<()> {
        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(app, input)?;
        app.handle_key(key(KeyCode::Enter))
    }

    fn toggle_split(app: &mut App) -> io::Result<()> {
        run_journal_search(app, "split")
    }

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
        assert_eq!(parse_command(":q").unwrap(), Command::Quit);
        assert_eq!(parse_command(":split").unwrap(), Command::ToggleSplit);
        assert_eq!(parse_command(":x").unwrap(), Command::Complete);
        assert_eq!(parse_command(":c").unwrap(), Command::Cancel);
        assert_eq!(parse_command(":i").unwrap(), Command::Important);
        assert_eq!(parse_command(":important").unwrap(), Command::Important);
    }

    #[test]
    fn rejects_empty_entries_and_unknown_commands() {
        assert!(parse_command(":n").is_err());
        assert!(parse_command(":cw").is_err());
        assert!(parse_command(":wat").is_err());
        assert!(parse_command("").is_err());
    }

    #[test]
    fn starts_with_journal_focused() -> io::Result<()> {
        let (app, root) = test_app()?;

        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(app.command_mode, CommandPaneMode::Normal);

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn space_opens_command_search_from_focused_journal() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(' ')))?;

        assert_eq!(app.focus, Focus::Command);
        assert_eq!(app.command_mode, CommandPaneMode::Search);
        assert_eq!(app.command_context, CommandContext::JournalPane);
        assert!(app.command_input.is_empty());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn space_opens_journal_command_search_with_highlighted_entry_actions() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        app.journal.add_entry(EntryKind::Task, "ship feature");
        app.selected = Some(0);

        app.handle_key(key(KeyCode::Char(' ')))?;

        let names = search_result_names(&app);
        assert_eq!(names[0], "complete");
        assert_eq!(names[1], "cancel");
        assert!(names.contains(&"important"));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn space_remains_literal_input_in_command_text_modes() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.focus = Focus::Command;
        app.handle_key(key(KeyCode::Char(' ')))?;
        assert_eq!(app.focus, Focus::Command);
        assert_eq!(app.command_mode, CommandPaneMode::Normal);
        assert_eq!(app.command_input, " ");

        app.open_command_search(CommandContext::CommandPane);
        app.handle_key(key(KeyCode::Char(' ')))?;
        assert_eq!(app.command_mode, CommandPaneMode::Search);
        assert_eq!(app.command_input, " ");

        app.command_mode = CommandPaneMode::Entry(CommandAction::Add(EntryKind::Note));
        app.command_input = String::from("draft");
        app.handle_key(key(KeyCode::Char(' ')))?;
        assert_eq!(
            app.command_mode,
            CommandPaneMode::Entry(CommandAction::Add(EntryKind::Note))
        );
        assert_eq!(app.command_input, "draft ");

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn switches_between_journal_days_with_left_and_right_arrows() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        fs::create_dir_all(&root)?;
        fs::write(
            root.join("2026-05-20.md"),
            "- yesterday note\n· yesterday task\n",
        )?;

        app.handle_key(key(KeyCode::Left))?;

        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(app.journal.entries.len(), 2);
        assert_eq!(app.journal.entries[0].text, "yesterday note");
        assert_eq!(app.journal.entries[1].text, "yesterday task");
        assert_eq!(app.selected, Some(1));
        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(app.status, "Loaded 2026-05-20.");

        app.handle_key(key(KeyCode::Right))?;

        assert_eq!(app.journal.date, date());
        assert!(app.journal.entries.is_empty());
        assert_eq!(app.selected, None);
        assert!(!root.join("2026-05-21.md").exists());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn navigates_single_journal_with_vim_keys() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        fs::create_dir_all(&root)?;
        fs::write(root.join("2026-05-20.md"), "- yesterday note\n")?;
        app.journal.add_entry(EntryKind::Note, "today one");
        app.journal.add_entry(EntryKind::Note, "today two");
        app.journal.add_entry(EntryKind::Note, "today three");
        app.selected = Some(1);

        app.handle_key(key(KeyCode::Char('k')))?;
        assert_eq!(app.selected, Some(0));

        app.handle_key(key(KeyCode::Char('j')))?;
        assert_eq!(app.selected, Some(1));

        app.handle_key(key(KeyCode::Char('h')))?;

        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(app.journal.entries[0].text, "yesterday note");
        assert_eq!(app.selected, Some(0));

        app.handle_key(key(KeyCode::Char('l')))?;

        assert_eq!(app.journal.date, date());
        assert!(app.journal.entries.is_empty());
        assert_eq!(app.selected, None);

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn switching_to_empty_day_does_not_create_file() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Right))?;

        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 22).unwrap()
        );
        assert!(app.journal.entries.is_empty());
        assert_eq!(app.selected, None);
        assert!(!root.join("2026-05-22.md").exists());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn writes_new_entries_to_displayed_day() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Right))?;
        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "n future note")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 22).unwrap()
        );
        assert_eq!(app.journal.entries.len(), 1);
        assert_eq!(
            app.journal.entries[0].created_on,
            NaiveDate::from_ymd_opt(2026, 5, 22).unwrap()
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-22.md"))?,
            "  - future note\n"
        );
        assert!(!root.join("2026-05-21.md").exists());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn toggles_split_view_on_for_yesterday_and_today() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        fs::create_dir_all(&root)?;
        fs::write(root.join("2026-05-20.md"), "- yesterday note\n")?;
        fs::write(root.join("2026-05-21.md"), "- today note\n")?;

        toggle_split(&mut app)?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(
            split.older.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(split.newer.journal.date, date());
        assert_eq!(split.active, SplitPane::Newer);
        assert_eq!(app.journal.date, date());
        assert_eq!(app.selected, Some(0));
        assert_eq!(split.older.journal.entries[0].text, "yesterday note");
        assert_eq!(split.newer.journal.entries[0].text, "today note");

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_view_arrows_focus_and_shift_the_two_day_window() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        toggle_split(&mut app)?;
        app.handle_key(key(KeyCode::Left))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Older);
        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );

        app.handle_key(key(KeyCode::Left))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(
            split.older.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 19).unwrap()
        );
        assert_eq!(
            split.newer.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(split.active, SplitPane::Older);
        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 19).unwrap()
        );

        app.handle_key(key(KeyCode::Right))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Newer);
        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );

        app.handle_key(key(KeyCode::Right))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(
            split.older.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(split.newer.journal.date, date());
        assert_eq!(split.active, SplitPane::Newer);
        assert_eq!(app.journal.date, date());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_view_navigates_with_vim_keys() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        fs::create_dir_all(&root)?;
        fs::write(
            root.join("2026-05-20.md"),
            "- yesterday one\n- yesterday two\n",
        )?;
        fs::write(
            root.join("2026-05-21.md"),
            "- today one\n- today two\n- today three\n",
        )?;

        toggle_split(&mut app)?;
        app.handle_key(key(KeyCode::Char('k')))?;
        assert_eq!(app.selected, Some(1));
        assert_eq!(
            app.split.as_ref().expect("split view").newer.selected,
            Some(1)
        );

        app.handle_key(key(KeyCode::Char('h')))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Older);
        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(app.selected, Some(1));

        app.handle_key(key(KeyCode::Char('k')))?;
        assert_eq!(app.selected, Some(0));

        app.handle_key(key(KeyCode::Char('j')))?;
        assert_eq!(app.selected, Some(1));

        app.handle_key(key(KeyCode::Char('h')))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(
            split.older.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 19).unwrap()
        );
        assert_eq!(
            split.newer.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(split.active, SplitPane::Older);

        app.handle_key(key(KeyCode::Char('l')))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Newer);
        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );

        app.handle_key(key(KeyCode::Char('l')))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(
            split.older.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(split.newer.journal.date, date());
        assert_eq!(split.active, SplitPane::Newer);

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_view_keeps_independent_selection_per_pane() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        fs::create_dir_all(&root)?;
        fs::write(
            root.join("2026-05-20.md"),
            "- yesterday one\n- yesterday two\n",
        )?;
        fs::write(
            root.join("2026-05-21.md"),
            "- today one\n- today two\n- today three\n",
        )?;

        toggle_split(&mut app)?;
        assert_eq!(app.selected, Some(2));

        app.handle_key(key(KeyCode::Up))?;
        assert_eq!(app.selected, Some(1));
        assert_eq!(
            app.split.as_ref().expect("split view").newer.selected,
            Some(1)
        );

        app.handle_key(key(KeyCode::Left))?;
        assert_eq!(app.selected, Some(1));
        app.handle_key(key(KeyCode::Up))?;
        assert_eq!(app.selected, Some(0));
        assert_eq!(
            app.split.as_ref().expect("split view").older.selected,
            Some(0)
        );

        app.handle_key(key(KeyCode::Right))?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Newer);
        assert_eq!(app.selected, Some(1));
        assert_eq!(split.older.selected, Some(0));
        assert_eq!(split.newer.selected, Some(1));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_view_writes_new_entries_to_focused_pane() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        toggle_split(&mut app)?;
        app.handle_key(key(KeyCode::Left))?;
        run_journal_search(&mut app, "n yesterday add")?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Older);
        assert_eq!(
            split.older.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );
        assert_eq!(split.older.journal.entries[0].text, "yesterday add");
        assert!(split.newer.journal.entries.is_empty());
        assert_eq!(app.journal.date, split.older.journal.date);
        assert_eq!(app.selected, Some(0));
        assert_eq!(
            fs::read_to_string(root.join("2026-05-20.md"))?,
            "  - yesterday add\n"
        );
        assert!(!root.join("2026-05-21.md").exists());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_view_applies_entry_actions_to_focused_pane() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        fs::create_dir_all(&root)?;
        fs::write(root.join("2026-05-20.md"), "· old task\n")?;
        fs::write(root.join("2026-05-21.md"), "· today task\n")?;

        toggle_split(&mut app)?;
        app.handle_key(key(KeyCode::Left))?;
        run_journal_search(&mut app, "x")?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Older);
        assert_eq!(
            split.older.journal.entries[0].state,
            crate::journal::EntryState::Completed
        );
        assert_eq!(
            split.newer.journal.entries[0].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-20.md"))?,
            "  X old task\n"
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "· today task\n"
        );

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_view_toggles_importance_on_focused_pane() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        fs::create_dir_all(&root)?;
        fs::write(root.join("2026-05-20.md"), "- old note\n")?;
        fs::write(root.join("2026-05-21.md"), "- today note\n")?;

        toggle_split(&mut app)?;
        app.handle_key(key(KeyCode::Left))?;
        run_journal_search(&mut app, "i")?;

        let split = app.split.as_ref().expect("split view should be active");
        assert_eq!(split.active, SplitPane::Older);
        assert!(split.older.journal.entries[0].important);
        assert!(!split.newer.journal.entries[0].important);
        assert_eq!(
            fs::read_to_string(root.join("2026-05-20.md"))?,
            "* - old note\n"
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "- today note\n"
        );

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn toggling_split_view_off_keeps_the_active_day() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        toggle_split(&mut app)?;
        app.handle_key(key(KeyCode::Left))?;
        toggle_split(&mut app)?;

        assert!(app.split.is_none());
        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
        );

        app.handle_key(key(KeyCode::Left))?;

        assert_eq!(
            app.journal.date,
            NaiveDate::from_ymd_opt(2026, 5, 19).unwrap()
        );

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_view_does_not_create_files_for_empty_displayed_days() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        toggle_split(&mut app)?;

        assert!(app.split.is_some());
        assert!(!root.join("2026-05-20.md").exists());
        assert!(!root.join("2026-05-21.md").exists());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn matches_fuzzy_subsequences_case_insensitively() {
        assert!(fuzzy_subsequence_match("nt", "note"));
        assert!(fuzzy_subsequence_match("TD", "todo"));
        assert!(!fuzzy_subsequence_match("task", "note"));
    }

    #[test]
    fn searches_only_command_pane_commands() {
        let task_results = command_search_results("t");
        assert_eq!(task_results[0].name, "task");

        assert!(command_search_results("cw").is_empty());

        let all_names = command_search_results("")
            .into_iter()
            .map(|result| result.name)
            .collect::<Vec<_>>();
        assert!(all_names.contains(&"split"));
        assert!(!all_names.contains(&"switch to journal"));
        assert!(!all_names.contains(&"yesterday"));
        assert!(!all_names.contains(&"tomorrow"));
        assert!(!all_names.contains(&"complete"));
        assert!(!all_names.contains(&"cancel"));
        assert!(!all_names.contains(&"important"));
    }

    #[test]
    fn selects_entry_command_from_fuzzy_search_and_submits_text() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        assert_eq!(app.command_mode, CommandPaneMode::Search);
        assert_eq!(app.focus, Focus::Command);

        type_text(&mut app, "nt")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(
            app.command_mode,
            CommandPaneMode::Entry(CommandAction::Add(EntryKind::Note))
        );
        assert_eq!(app.command_title(), "Enter a note");

        type_text(&mut app, "a fuzzy note")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(app.command_mode, CommandPaneMode::Normal);
        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(app.journal.entries.len(), 1);
        assert_eq!(app.journal.entries[0].kind, EntryKind::Note);
        assert_eq!(app.journal.entries[0].text, "a fuzzy note");

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn navigates_search_results_with_arrow_keys() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        app.handle_key(key(KeyCode::Down))?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(
            app.command_mode,
            CommandPaneMode::Entry(CommandAction::Add(EntryKind::Event))
        );
        assert_eq!(app.command_title(), "Enter an event");

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn escape_hides_command_search_and_focuses_journal() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "nt")?;
        app.handle_key(key(KeyCode::Esc))?;

        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(app.command_mode, CommandPaneMode::Normal);
        assert!(app.command_input.is_empty());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn escape_hides_selected_command_entry_and_focuses_journal() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "nt")?;
        app.handle_key(key(KeyCode::Enter))?;
        type_text(&mut app, "draft")?;
        app.handle_key(key(KeyCode::Esc))?;

        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(app.command_mode, CommandPaneMode::Normal);
        assert!(app.command_input.is_empty());
        assert!(app.journal.entries.is_empty());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn executes_no_text_command_immediately_from_search() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "q")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(app.command_mode, CommandPaneMode::Normal);
        assert!(app.should_quit);

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn keeps_exact_command_submission_available_from_search() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "n exact note")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(app.command_mode, CommandPaneMode::Normal);
        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(app.journal.entries.len(), 1);
        assert_eq!(app.journal.entries[0].kind, EntryKind::Note);
        assert_eq!(app.journal.entries[0].text, "exact note");

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn identifies_exact_shortcut_input_during_search() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "n exact note")?;

        assert!(app.visible_command_search_results().is_empty());
        assert!(app.command_search_input_is_exact_command());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn clears_no_match_status_when_search_input_changes() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "zzz")?;
        app.handle_key(key(KeyCode::Enter))?;
        assert_eq!(app.status, "No matching commands.");

        for _ in 0.."zzz".len() {
            app.handle_key(key(KeyCode::Backspace))?;
        }
        type_text(&mut app, "n exact note")?;

        assert_eq!(app.status, "Search commands.");
        assert!(app.command_search_input_is_exact_command());

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn reports_empty_search_results_without_exiting_search() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "zzz")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(app.command_mode, CommandPaneMode::Search);
        assert_eq!(app.status, "No matching commands.");

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn command_text_inputs_keep_vim_keys_as_text() -> io::Result<()> {
        let (mut app, root) = test_app()?;

        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "hjkl")?;
        assert_eq!(app.focus, Focus::Command);
        assert_eq!(app.command_mode, CommandPaneMode::Search);
        assert_eq!(app.command_input, "hjkl");

        app.handle_key(key(KeyCode::Esc))?;
        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(&mut app, "n")?;
        app.handle_key(key(KeyCode::Enter))?;
        type_text(&mut app, "hjkl")?;

        assert_eq!(
            app.command_mode,
            CommandPaneMode::Entry(CommandAction::Add(EntryKind::Note))
        );
        assert_eq!(app.command_input, "hjkl");

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn exposes_and_executes_task_actions_for_highlighted_task() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        app.journal.add_entry(EntryKind::Note, "keep note");
        app.journal.add_entry(EntryKind::Task, "ship feature");
        app.selected = Some(1);

        app.handle_key(key(KeyCode::Char(':')))?;

        let names = search_result_names(&app);
        assert_eq!(names[0], "complete");
        assert_eq!(names[1], "cancel");
        assert!(names.contains(&"important"));

        type_text(&mut app, "complete")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(
            app.journal.entries[1].state,
            crate::journal::EntryState::Completed
        );
        assert_eq!(
            app.journal.entries[0].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "  - keep note\n  X ship feature\n"
        );

        run_journal_search(&mut app, "c")?;

        assert_eq!(app.status, "Completed tasks cannot be cancelled.");
        assert_eq!(
            app.journal.entries[1].state,
            crate::journal::EntryState::Completed
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "  - keep note\n  X ship feature\n"
        );

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn exposes_and_executes_event_actions_for_highlighted_event() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        app.journal.add_entry(EntryKind::Note, "keep note");
        app.journal.add_entry(EntryKind::Event, "planning");
        app.journal.add_entry(EntryKind::Task, "ship feature");
        app.selected = Some(1);

        app.handle_key(key(KeyCode::Char(':')))?;

        let names = search_result_names(&app);
        assert_eq!(names[0], "cancel");
        assert!(names.contains(&"important"));
        assert!(!names.contains(&"complete"));

        type_text(&mut app, "cancel")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(app.focus, Focus::Journal);
        assert_eq!(
            app.journal.entries[1].state,
            crate::journal::EntryState::Cancelled
        );
        assert_eq!(
            app.journal.entries[0].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(
            app.journal.entries[2].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "  - keep note\n  ◦ ~~planning~~\n  · ship feature\n"
        );

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn toggles_importance_for_highlighted_entry_and_alias() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        app.journal.add_entry(EntryKind::Note, "plain note");
        app.selected = Some(0);

        run_journal_search(&mut app, "i")?;

        assert_eq!(app.status, "Entry marked important.");
        assert!(app.journal.entries[0].important);
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "* - plain note\n"
        );

        run_journal_search(&mut app, "important")?;

        assert_eq!(app.status, "Entry unmarked important.");
        assert!(!app.journal.entries[0].important);
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "  - plain note\n"
        );

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn exposes_only_importance_for_notes_and_feelings_and_keeps_invalid_actions_unchanged(
    ) -> io::Result<()> {
        let (mut app, root) = test_app()?;
        app.journal.add_entry(EntryKind::Note, "plain note");
        app.journal.add_entry(EntryKind::Feeling, "focused");
        app.journal.add_entry(EntryKind::Task, "ship feature");
        app.journal.save()?;
        let before = fs::read_to_string(root.join("2026-05-21.md"))?;

        app.selected = Some(0);
        app.handle_key(key(KeyCode::Char(':')))?;
        let note_names = search_result_names(&app);
        assert!(note_names.contains(&"important"));
        assert!(!note_names.contains(&"complete"));
        assert!(!note_names.contains(&"cancel"));
        type_text(&mut app, "x")?;
        app.handle_key(key(KeyCode::Enter))?;

        app.selected = Some(1);
        app.handle_key(key(KeyCode::Char(':')))?;
        let feeling_names = search_result_names(&app);
        assert!(feeling_names.contains(&"important"));
        assert!(!feeling_names.contains(&"complete"));
        assert!(!feeling_names.contains(&"cancel"));
        type_text(&mut app, "c")?;
        app.handle_key(key(KeyCode::Enter))?;

        assert_eq!(
            app.journal.entries[0].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(
            app.journal.entries[1].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(
            app.journal.entries[2].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(fs::read_to_string(root.join("2026-05-21.md"))?, before);

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn does_not_apply_actions_without_a_highlighted_entry() -> io::Result<()> {
        let (mut app, root) = test_app()?;
        app.journal.add_entry(EntryKind::Task, "leave open");
        app.journal.save()?;
        app.selected = None;

        run_journal_search(&mut app, "x")?;

        assert_eq!(app.status, "No entry selected.");
        assert_eq!(app.selected, None);
        assert_eq!(
            app.journal.entries[0].state,
            crate::journal::EntryState::Open
        );
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "  · leave open\n"
        );

        run_journal_search(&mut app, "i")?;

        assert_eq!(app.status, "No entry selected.");
        assert_eq!(app.selected, None);
        assert!(!app.journal.entries[0].important);
        assert_eq!(
            fs::read_to_string(root.join("2026-05-21.md"))?,
            "  · leave open\n"
        );

        let _ = fs::remove_dir_all(root);
        Ok(())
    }
}
