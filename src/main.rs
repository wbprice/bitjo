mod app;
mod journal;
mod ui;

use std::{
    error::Error,
    io::{self, Stdout},
    path::Path,
};

use app::App;
use chrono::Local;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use journal::Journal;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let today = Local::now().date_naive();
    let journal = Journal::load_for_date(Path::new("journal"), today)?;
    let mut app = App::new(journal);

    let mut terminal = setup_terminal()?;
    let run_result = run_app(&mut terminal, &mut app);
    let restore_result = restore_terminal(&mut terminal);

    if let Err(error) = restore_result {
        eprintln!("failed to restore terminal: {error}");
    }

    run_result?;
    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if let Event::Key(key) = event::read()? {
            app.handle_key(key)?;
        }
    }

    Ok(())
}
