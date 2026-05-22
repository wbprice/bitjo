use std::{
    fs, io,
    path::{Path, PathBuf},
};

use chrono::{Datelike, NaiveDate, Weekday};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Note,
    Event,
    Feeling,
    Task,
    Raw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryState {
    Open,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    pub kind: EntryKind,
    pub text: String,
    pub state: EntryState,
    pub created_on: NaiveDate,
}

impl JournalEntry {
    pub fn new(kind: EntryKind, text: impl Into<String>, created_on: NaiveDate) -> Self {
        Self {
            kind,
            text: text.into(),
            state: EntryState::Open,
            created_on,
        }
    }

    pub fn raw(line: impl Into<String>, created_on: NaiveDate) -> Self {
        Self {
            kind: EntryKind::Raw,
            text: line.into(),
            state: EntryState::Open,
            created_on,
        }
    }

    pub fn to_markdown_line(&self) -> String {
        match self.kind {
            EntryKind::Note => format!("- {}", self.text),
            EntryKind::Event => format!("◦ {}", self.render_text()),
            EntryKind::Feeling => format!("= {}", self.text),
            EntryKind::Task => match self.state {
                EntryState::Completed => format!("X {}", self.text),
                EntryState::Open | EntryState::Cancelled => format!("· {}", self.render_text()),
            },
            EntryKind::Raw => self.text.clone(),
        }
    }

    pub fn is_struck(&self) -> bool {
        matches!(self.state, EntryState::Cancelled)
    }

    pub fn toggle_complete(&mut self) -> Result<&'static str, &'static str> {
        if self.kind != EntryKind::Task {
            return Err("Only tasks can be completed.");
        }

        self.state = if self.state == EntryState::Completed {
            EntryState::Open
        } else {
            EntryState::Completed
        };

        Ok(match self.state {
            EntryState::Completed => "Task completed.",
            _ => "Task reopened.",
        })
    }

    pub fn toggle_cancel(&mut self) -> Result<&'static str, &'static str> {
        match self.kind {
            EntryKind::Task if self.state == EntryState::Completed => {
                Err("Completed tasks cannot be cancelled.")
            }
            EntryKind::Task | EntryKind::Event => {
                self.state = if self.state == EntryState::Cancelled {
                    EntryState::Open
                } else {
                    EntryState::Cancelled
                };

                Ok(match self.state {
                    EntryState::Cancelled => "Entry cancelled.",
                    _ => "Entry reopened.",
                })
            }
            _ => Err("Only tasks and events can be cancelled."),
        }
    }

    fn render_text(&self) -> String {
        if self.is_struck() {
            format!("~~{}~~", self.text)
        } else {
            self.text.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Journal {
    pub date: NaiveDate,
    pub entries: Vec<JournalEntry>,
    path: PathBuf,
}

impl Journal {
    pub fn load_for_date(root: &Path, date: NaiveDate) -> io::Result<Self> {
        let path = root.join(format!("{}.md", date.format("%Y-%m-%d")));
        let entries = if path.exists() {
            let contents = fs::read_to_string(&path)?;
            parse_markdown(&contents, date)
        } else {
            Vec::new()
        };

        Ok(Self {
            date,
            entries,
            path,
        })
    }

    pub fn add_entry(&mut self, kind: EntryKind, text: impl Into<String>) {
        self.entries.push(JournalEntry::new(kind, text, self.date));
    }

    pub fn save(&self) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, self.to_markdown())
    }

    pub fn to_markdown(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }

        let mut markdown = self
            .entries
            .iter()
            .map(JournalEntry::to_markdown_line)
            .collect::<Vec<_>>()
            .join("\n");
        markdown.push('\n');
        markdown
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn title(&self) -> String {
        format_journal_title(self.date)
    }
}

pub fn parse_markdown(contents: &str, date: NaiveDate) -> Vec<JournalEntry> {
    contents
        .lines()
        .map(|line| parse_markdown_line(line, date))
        .collect()
}

pub fn parse_markdown_line(line: &str, date: NaiveDate) -> JournalEntry {
    if let Some(rest) = line.strip_prefix("- ") {
        return JournalEntry::new(EntryKind::Note, rest, date);
    }

    if let Some(rest) = line.strip_prefix("◦ ") {
        let (cancelled, text) = unwrap_strikethrough(rest);
        let mut entry = JournalEntry::new(EntryKind::Event, text, date);
        if cancelled {
            entry.state = EntryState::Cancelled;
        }
        return entry;
    }

    if let Some(rest) = line.strip_prefix("= ") {
        return JournalEntry::new(EntryKind::Feeling, rest, date);
    }

    if let Some(rest) = line.strip_prefix("· ") {
        let (cancelled, text) = unwrap_strikethrough(rest);
        let mut entry = JournalEntry::new(EntryKind::Task, text, date);
        if cancelled {
            entry.state = EntryState::Cancelled;
        }
        return entry;
    }

    if let Some(rest) = line.strip_prefix("X ") {
        let (_, text) = unwrap_strikethrough(rest);
        let mut entry = JournalEntry::new(EntryKind::Task, text, date);
        entry.state = EntryState::Completed;
        return entry;
    }

    JournalEntry::raw(line, date)
}

pub fn format_journal_title(date: NaiveDate) -> String {
    format!(
        "{}.{}.{}",
        date.month(),
        date.day(),
        weekday_label(date.weekday())
    )
}

fn weekday_label(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "M",
        Weekday::Tue => "Tu",
        Weekday::Wed => "W",
        Weekday::Thu => "Th",
        Weekday::Fri => "F",
        Weekday::Sat => "Sa",
        Weekday::Sun => "Su",
    }
}

fn unwrap_strikethrough(text: &str) -> (bool, String) {
    let trimmed = text.trim();
    if trimmed.starts_with("~~") && trimmed.ends_with("~~") && trimmed.len() >= 4 {
        (true, trimmed[2..trimmed.len() - 2].to_string())
    } else {
        (false, trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env, fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 5, 21).unwrap()
    }

    #[test]
    fn formats_title_with_distinct_weekdays() {
        assert_eq!(format_journal_title(date()), "5.21.Th");
        assert_eq!(
            format_journal_title(NaiveDate::from_ymd_opt(2026, 5, 19).unwrap()),
            "5.19.Tu"
        );
        assert_eq!(
            format_journal_title(NaiveDate::from_ymd_opt(2026, 5, 23).unwrap()),
            "5.23.Sa"
        );
        assert_eq!(
            format_journal_title(NaiveDate::from_ymd_opt(2026, 5, 24).unwrap()),
            "5.24.Su"
        );
    }

    #[test]
    fn renders_markdown_symbols_and_strikes_only_cancelled_entries() {
        let note = JournalEntry::new(EntryKind::Note, "plain note", date());
        assert_eq!(note.to_markdown_line(), "- plain note");

        let mut event = JournalEntry::new(EntryKind::Event, "cancelled event", date());
        event.state = EntryState::Cancelled;
        assert_eq!(event.to_markdown_line(), "◦ ~~cancelled event~~");

        let feeling = JournalEntry::new(EntryKind::Feeling, "accomplished", date());
        assert_eq!(feeling.to_markdown_line(), "= accomplished");

        let task = JournalEntry::new(EntryKind::Task, "open task", date());
        assert_eq!(task.to_markdown_line(), "· open task");

        let mut completed = JournalEntry::new(EntryKind::Task, "done", date());
        completed.state = EntryState::Completed;
        assert_eq!(completed.to_markdown_line(), "X done");
    }

    #[test]
    fn parses_markdown_entries() {
        let entries = parse_markdown(
            "- note\n◦ ~~event~~\n= mood\n· task\nX done\nX ~~legacy done~~\n",
            date(),
        );

        assert_eq!(entries[0].kind, EntryKind::Note);
        assert_eq!(entries[1].state, EntryState::Cancelled);
        assert_eq!(entries[2].kind, EntryKind::Feeling);
        assert_eq!(entries[3].kind, EntryKind::Task);
        assert_eq!(entries[3].state, EntryState::Open);
        assert_eq!(entries[4].state, EntryState::Completed);
        assert_eq!(entries[4].text, "done");
        assert_eq!(entries[5].state, EntryState::Completed);
        assert_eq!(entries[5].text, "legacy done");
    }

    #[test]
    fn normalizes_legacy_completed_task_markdown_on_save() -> io::Result<()> {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = env::temp_dir().join(format!("bullet-journal-tui-test-{unique}"));
        fs::create_dir_all(&root)?;
        fs::write(
            root.join("2026-05-21.md"),
            "X ~~done~~\n· ~~cancelled task~~\n◦ ~~cancelled event~~\n",
        )?;

        let journal = Journal::load_for_date(&root, date())?;
        journal.save()?;

        let saved = fs::read_to_string(root.join("2026-05-21.md"))?;
        assert_eq!(
            saved,
            "X done\n· ~~cancelled task~~\n◦ ~~cancelled event~~\n"
        );

        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[test]
    fn toggles_task_completion_and_rejects_cancelling_completed_tasks() {
        let mut task = JournalEntry::new(EntryKind::Task, "ship", date());

        assert_eq!(task.toggle_complete().unwrap(), "Task completed.");
        assert_eq!(task.state, EntryState::Completed);
        assert_eq!(
            task.toggle_cancel().unwrap_err(),
            "Completed tasks cannot be cancelled."
        );

        assert_eq!(task.toggle_complete().unwrap(), "Task reopened.");
        assert_eq!(task.state, EntryState::Open);
        assert_eq!(task.toggle_cancel().unwrap(), "Entry cancelled.");
        assert_eq!(task.state, EntryState::Cancelled);
        assert_eq!(task.toggle_cancel().unwrap(), "Entry reopened.");
        assert_eq!(task.state, EntryState::Open);
    }

    #[test]
    fn toggles_event_cancellation() {
        let mut event = JournalEntry::new(EntryKind::Event, "meeting", date());

        assert_eq!(event.toggle_cancel().unwrap(), "Entry cancelled.");
        assert_eq!(event.state, EntryState::Cancelled);
        assert_eq!(event.toggle_cancel().unwrap(), "Entry reopened.");
        assert_eq!(event.state, EntryState::Open);
    }

    #[test]
    fn persists_journal_file_after_changes() -> io::Result<()> {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = env::temp_dir().join(format!("bullet-journal-tui-test-{unique}"));

        let mut journal = Journal::load_for_date(&root, date())?;
        journal.add_entry(EntryKind::Task, "persist this");
        journal.save()?;

        let saved = fs::read_to_string(root.join("2026-05-21.md"))?;
        assert_eq!(saved, "· persist this\n");

        fs::remove_dir_all(root)?;
        Ok(())
    }
}
