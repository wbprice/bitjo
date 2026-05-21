use std::io;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add(EntryKind, String),
    Quit,
    Complete,
    Cancel,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MatchQuality {
    Prefix,
    Subsequence,
}

const COMMAND_SEARCH_RESULT_LIMIT: usize = 5;

const COMMAND_OPTIONS: &[CommandOption] = &[
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
];

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
}

impl App {
    pub fn new(journal: Journal) -> Self {
        let selected = last_entry_index(&journal);

        Self {
            journal,
            focus: Focus::Journal,
            command_mode: CommandPaneMode::Normal,
            command_input: String::new(),
            command_result_index: 0,
            selected,
            status: String::from("Ready."),
            should_quit: false,
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
                self.open_command_search();
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

                let matches = matching_command_options(&self.command_input);
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
            KeyCode::Down => self.select_next(),
            KeyCode::Char(':') if is_text_input(key.modifiers) => {
                self.open_command_search();
            }
            _ => {}
        }

        Ok(())
    }

    fn open_command_search(&mut self) {
        self.focus = Focus::Command;
        self.command_input.clear();
        self.command_mode = CommandPaneMode::Search;
        self.command_result_index = 0;
        self.status = String::from("Search commands.");
    }

    fn focus_journal(&mut self) {
        self.focus = Focus::Journal;
        self.status = String::from("Journal pane focused.");
    }

    fn execute_command_from_search_input(&mut self) -> io::Result<bool> {
        if let Some(input) = command_pane_command_from_search_input(&self.command_input) {
            self.reset_command_pane();
            self.execute_command(&input, CommandContext::CommandPane)?;
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
                self.command_mode = CommandPaneMode::Normal;
                self.execute_command(command.token, CommandContext::CommandPane)?;
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

                self.journal.add_entry(kind, text);
                self.journal.save()?;
                self.selected = last_entry_index(&self.journal);
                self.status = format!("Wrote {}.", self.journal.path().display());
                if kind == EntryKind::Note {
                    self.focus = Focus::Journal;
                }
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

    fn normalize_command_result_index(&mut self) {
        let count = matching_command_options(&self.command_input).len();
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
        let count = matching_command_options(&self.command_input).len();
        if count > 0 {
            self.command_result_index = (self.command_result_index + 1).min(count - 1);
        }
    }

    fn reset_command_pane(&mut self) {
        self.command_mode = CommandPaneMode::Normal;
        self.command_input.clear();
        self.command_result_index = 0;
    }

    pub fn command_title(&self) -> &'static str {
        match self.command_mode {
            CommandPaneMode::Normal => "Command",
            CommandPaneMode::Search => "Search Commands",
            CommandPaneMode::Entry(action) => action.entry_title(),
        }
    }

    pub fn visible_command_search_results(&self) -> Vec<(usize, CommandSearchResult)> {
        let results = command_search_results(&self.command_input);
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
        command_pane_command_from_search_input(&self.command_input).is_some()
    }
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

pub fn command_search_results(query: &str) -> Vec<CommandSearchResult> {
    matching_command_options(query)
        .into_iter()
        .map(|command| CommandSearchResult {
            name: command.name,
            token: command.token,
        })
        .collect()
}

fn matching_command_options(query: &str) -> Vec<&'static CommandOption> {
    let mut matches = COMMAND_OPTIONS
        .iter()
        .enumerate()
        .filter_map(|(index, command)| {
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

fn is_command_pane_command(input: &str) -> bool {
    matches!(parse_command(input), Ok(Command::Add(_, _) | Command::Quit))
}

fn command_pane_command_from_search_input(query: &str) -> Option<String> {
    let query = query.trim();
    if query.is_empty() {
        return None;
    }

    let input = format!(":{query}");
    is_command_pane_command(&input).then_some(input)
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
        assert_eq!(parse_command(":x").unwrap(), Command::Complete);
        assert_eq!(parse_command(":c").unwrap(), Command::Cancel);
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
        assert!(!all_names.contains(&"switch to journal"));
        assert!(!all_names.contains(&"complete"));
        assert!(!all_names.contains(&"cancel"));
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
}
