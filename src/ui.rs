use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    app::{App, CommandPaneMode, Focus},
    journal::{EntryKind, JournalEntry},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let command_height = command_pane_height(app);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(command_height)])
        .split(frame.area());

    draw_journal(frame, chunks[0], app);
    draw_command(frame, chunks[1], app);
}

fn draw_journal(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let is_focused = matches!(app.focus, Focus::Journal);
    let block = Block::default()
        .title(app.journal.title())
        .borders(Borders::ALL)
        .border_style(border_style(is_focused));

    let items = if app.journal.entries.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No entries yet.",
            Style::default().fg(Color::DarkGray),
        )))]
    } else {
        app.journal
            .entries
            .iter()
            .map(entry_item)
            .collect::<Vec<_>>()
    };

    let mut state = ListState::default();
    if is_focused {
        state.select(app.selected);
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

    let line = if entry.kind == EntryKind::Raw {
        Line::from(Span::styled(entry.text.clone(), text_style))
    } else {
        Line::from(vec![
            Span::styled(symbol.to_string(), Style::default().fg(Color::Yellow)),
            Span::raw(" "),
            Span::styled(entry.text.clone(), text_style),
        ])
    };

    ListItem::new(line)
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
    use ratatui::{backend::TestBackend, buffer::Buffer, Terminal};

    use crate::{
        app::{CommandPaneMode, Focus},
        journal::Journal,
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

    fn render_text(app: &App) -> io::Result<String> {
        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|frame| draw(frame, app))?;
        Ok(buffer_text(terminal.backend().buffer()))
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
