use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    app::{App, Focus},
    journal::{EntryKind, JournalEntry},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(4)])
        .split(frame.area());

    draw_journal(frame, chunks[0], app);
    draw_command(frame, chunks[1], app);
}

fn draw_journal(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let is_focused = matches!(app.focus, Focus::Journal | Focus::JournalCommand);
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

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_command(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let is_focused = matches!(app.focus, Focus::Command | Focus::JournalCommand);
    let title = match app.focus {
        Focus::Command => "Command",
        Focus::Journal => "Status",
        Focus::JournalCommand => "Journal Command",
    };

    let input = match app.focus {
        Focus::JournalCommand => app.mini_input.as_str(),
        _ => app.command_input.as_str(),
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::DarkGray)),
            Span::raw(input.to_string()),
        ]),
        Line::from(Span::styled(
            app.status.clone(),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style(is_focused)),
    );

    frame.render_widget(paragraph, area);
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
