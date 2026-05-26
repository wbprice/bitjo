use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    app::{App, CommandPaneMode, Focus, SplitJournalView, SplitPane},
    journal::{EntryKind, Journal, JournalEntry},
};

const SPLIT_SIDE_BY_SIDE_MIN_WIDTH: u16 = 100;

pub fn draw(frame: &mut Frame, app: &App) {
    let command_height = command_pane_height(app);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(command_height)])
        .split(frame.area());

    draw_journal_area(frame, chunks[0], app);
    draw_command(frame, chunks[1], app);
}

fn draw_journal_area(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    if let Some(split) = app.split_view() {
        draw_split_journal(frame, area, app, split);
    } else {
        draw_journal(
            frame,
            area,
            &app.journal,
            app.selected,
            matches!(app.focus, Focus::Journal),
            matches!(app.focus, Focus::Journal),
        );
    }
}

fn draw_split_journal(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    app: &App,
    split: &SplitJournalView,
) {
    let direction = if area.width >= SPLIT_SIDE_BY_SIDE_MIN_WIDTH {
        Direction::Horizontal
    } else {
        Direction::Vertical
    };

    let chunks = Layout::default()
        .direction(direction)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    draw_journal(
        frame,
        chunks[0],
        &split.older.journal,
        split.older.selected,
        split.active == SplitPane::Older,
        matches!(app.focus, Focus::Journal) && split.active == SplitPane::Older,
    );
    draw_journal(
        frame,
        chunks[1],
        &split.newer.journal,
        split.newer.selected,
        split.active == SplitPane::Newer,
        matches!(app.focus, Focus::Journal) && split.active == SplitPane::Newer,
    );
}

fn draw_journal(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    journal: &Journal,
    selected: Option<usize>,
    is_active: bool,
    show_selection: bool,
) {
    let block = Block::default()
        .title(journal.title())
        .borders(Borders::ALL)
        .border_style(border_style(is_active));

    let items = if journal.entries.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No entries yet.",
            Style::default().fg(Color::DarkGray),
        )))]
    } else {
        journal.entries.iter().map(entry_item).collect::<Vec<_>>()
    };

    let mut state = ListState::default();
    if show_selection {
        state.select(selected);
    }

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_command(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let is_focused = matches!(app.focus, Focus::Command);
    let title = match app.focus {
        Focus::Command => app.command_title(),
        Focus::Journal => "Command",
    };

    let input = match (app.focus, app.command_mode) {
        (Focus::Command, CommandPaneMode::Search) => format!(":{}", app.command_input),
        _ => app.command_input.clone(),
    };

    let mut lines = vec![Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::DarkGray)),
        Span::raw(input),
    ])];

    if matches!(
        (app.focus, app.command_mode),
        (Focus::Command, CommandPaneMode::Search)
    ) {
        let results = app.visible_command_search_results();
        if results.is_empty() && !app.command_search_input_is_exact_command() {
            lines.push(Line::from(Span::styled(
                "No matching commands.",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for (index, result) in results {
                let selected = index == app.command_result_index;
                let marker_style = if selected {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                let name_style = if selected {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                lines.push(Line::from(vec![
                    Span::styled(if selected { "> " } else { "  " }, marker_style),
                    Span::styled(result.name, name_style),
                    Span::raw(" "),
                    Span::styled(result.token, Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style(is_focused)),
    );

    frame.render_widget(paragraph, area);
}

fn command_pane_height(app: &App) -> u16 {
    if matches!(
        (app.focus, app.command_mode),
        (Focus::Command, CommandPaneMode::Search)
    ) {
        app.command_search_result_limit() as u16 + 3
    } else {
        3
    }
}

fn entry_item(entry: &JournalEntry) -> ListItem<'static> {
    let symbol = match entry.kind {
        EntryKind::Note => "-",
        EntryKind::Event => "◦",
        EntryKind::Feeling => "=",
        EntryKind::Task => {
            if entry.state == crate::journal::EntryState::Completed {
                "X"
            } else {
                "·"
            }
        }
        EntryKind::Raw => "",
    };

    let text_style = if entry.is_struck() {
        Style::default().add_modifier(Modifier::CROSSED_OUT)
    } else {
        Style::default()
    };

    let mut spans = Vec::new();
    if entry.important {
        spans.push(Span::styled(
            "*",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(" "));
    } else {
        spans.push(Span::raw("  "));
    }

    if entry.kind == EntryKind::Raw {
        spans.push(Span::styled(entry.text.clone(), text_style));
    } else {
        spans.push(Span::styled(
            symbol.to_string(),
            Style::default().fg(Color::Yellow),
        ));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(entry.text.clone(), text_style));
    }

    ListItem::new(Line::from(spans))
}

fn border_style(is_focused: bool) -> Style {
    if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
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
    use ratatui::{backend::TestBackend, buffer::Buffer, Terminal};

    use crate::{
        app::{CommandPaneMode, Focus},
        journal::{EntryKind, EntryState, Journal},
    };

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 5, 21).unwrap()
    }

    fn test_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        env::temp_dir().join(format!("bullet-journal-tui-ui-test-{unique}"))
    }

    fn search_app(input: &str) -> io::Result<(App, PathBuf)> {
        let root = test_root();
        let journal = Journal::load_for_date(&root, date())?;
        let mut app = App::new(journal);
        app.focus = Focus::Command;
        app.command_mode = CommandPaneMode::Search;
        app.command_input = input.to_string();
        app.status = String::from("Search commands.");
        Ok((app, root))
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

    fn toggle_split(app: &mut App) -> io::Result<()> {
        app.handle_key(key(KeyCode::Char(':')))?;
        type_text(app, "split")?;
        app.handle_key(key(KeyCode::Enter))
    }

    fn split_app() -> io::Result<(App, PathBuf)> {
        let root = test_root();
        fs::create_dir_all(&root)?;
        fs::write(root.join("2026-05-20.md"), "- yesterday note\n")?;
        fs::write(root.join("2026-05-21.md"), "- today note\n")?;

        let journal = Journal::load_for_date(&root, date())?;
        let mut app = App::new(journal);
        toggle_split(&mut app)?;
        Ok((app, root))
    }

    fn render_text(app: &App) -> io::Result<String> {
        Ok(buffer_text(&render_buffer(app)?))
    }

    fn render_buffer(app: &App) -> io::Result<Buffer> {
        render_buffer_with_size(app, 80, 20)
    }

    fn render_buffer_with_size(app: &App, width: u16, height: u16) -> io::Result<Buffer> {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|frame| draw(frame, app))?;
        Ok(terminal.backend().buffer().clone())
    }

    fn buffer_text(buffer: &Buffer) -> String {
        let width = buffer.area.width as usize;
        let mut text = String::new();

        for row in buffer.content().chunks(width) {
            for cell in row {
                text.push_str(cell.symbol());
            }
            text.push('\n');
        }

        text
    }

    fn modifier_for_text(buffer: &Buffer, needle: &str) -> Modifier {
        let width = buffer.area.width as usize;

        for (row_index, row) in buffer.content().chunks(width).enumerate() {
            let row_text = row.iter().map(|cell| cell.symbol()).collect::<String>();
            if let Some(byte_col) = row_text.find(needle) {
                let col = row_text[..byte_col].chars().count() as u16;
                return buffer[(col, row_index as u16)].modifier;
            }
        }

        panic!("rendered buffer did not contain {needle:?}");
    }

    fn row_containing(buffer: &Buffer, needle: &str) -> usize {
        let width = buffer.area.width as usize;

        for (row_index, row) in buffer.content().chunks(width).enumerate() {
            let row_text = row.iter().map(|cell| cell.symbol()).collect::<String>();
            if row_text.contains(needle) {
                return row_index;
            }
        }

        panic!("rendered buffer did not contain {needle:?}");
    }

    #[test]
    fn strikes_cancelled_entries_but_not_completed_tasks() -> io::Result<()> {
        let root = test_root();
        let mut journal = Journal::load_for_date(&root, date())?;
        journal.add_entry(EntryKind::Task, "completed task");
        journal.entries[0].state = EntryState::Completed;
        journal.add_entry(EntryKind::Task, "cancelled task");
        journal.entries[1].state = EntryState::Cancelled;

        let mut app = App::new(journal);
        app.focus = Focus::Command;

        let buffer = render_buffer(&app)?;

        assert!(!modifier_for_text(&buffer, "completed task").contains(Modifier::CROSSED_OUT));
        assert!(modifier_for_text(&buffer, "cancelled task").contains(Modifier::CROSSED_OUT));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn renders_important_prefix_before_entry_symbol() -> io::Result<()> {
        let root = test_root();
        let mut journal = Journal::load_for_date(&root, date())?;
        journal.add_entry(EntryKind::Note, "normal note");
        journal.add_entry(EntryKind::Note, "important note");
        journal.entries[1].important = true;
        journal.add_entry(EntryKind::Task, "important task");
        journal.entries[2].important = true;
        journal.entries[2].state = EntryState::Completed;

        let app = App::new(journal);
        let rendered = render_text(&app)?;

        assert!(rendered.contains("  - normal note"));
        assert!(rendered.contains("* - important note"));
        assert!(rendered.contains("* X important task"));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_journal_renders_side_by_side_on_wide_screens() -> io::Result<()> {
        let (app, root) = split_app()?;

        let buffer = render_buffer_with_size(&app, 120, 24)?;
        let older_title_row = row_containing(&buffer, "5.20.W");
        let newer_title_row = row_containing(&buffer, "5.21.Th");
        let rendered = buffer_text(&buffer);

        assert_eq!(older_title_row, newer_title_row);
        assert!(rendered.contains("yesterday note"));
        assert!(rendered.contains("today note"));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn split_journal_renders_stacked_on_narrow_screens() -> io::Result<()> {
        let (app, root) = split_app()?;

        let buffer = render_buffer_with_size(&app, 80, 24)?;
        let older_title_row = row_containing(&buffer, "5.20.W");
        let newer_title_row = row_containing(&buffer, "5.21.Th");
        let rendered = buffer_text(&buffer);

        assert!(older_title_row < newer_title_row);
        assert!(rendered.contains("yesterday note"));
        assert!(rendered.contains("today note"));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn hides_no_match_message_for_exact_shortcut_input() -> io::Result<()> {
        let (app, root) = search_app("n exact note")?;

        let rendered = render_text(&app)?;

        assert!(!rendered.contains("No matching commands."));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn shows_no_match_message_for_unmatched_search_input() -> io::Result<()> {
        let (app, root) = search_app("zzz")?;

        let rendered = render_text(&app)?;

        assert!(rendered.contains("No matching commands."));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    #[test]
    fn does_not_render_app_status_in_command_pane() -> io::Result<()> {
        let (mut app, root) = search_app("n")?;
        app.status = String::from("Search commands.");

        let rendered = render_text(&app)?;

        assert!(!rendered.contains("Search commands."));

        let _ = fs::remove_dir_all(root);
        Ok(())
    }
}
