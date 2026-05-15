/// Chat TUI module — Interactive terminal chat interface for Coven agents
mod app;
mod commands;
mod events;
mod ui;

pub use app::ChatApp;

use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use std::io;

/// Run the interactive chat TUI
pub async fn run_chat() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = ChatApp::new();

    // Run the app
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    ).ok();
    terminal.show_cursor().ok();

    result
}

async fn run_app<B: Backend + Send>(terminal: &mut Terminal<B>, app: &mut ChatApp) -> Result<()>
where
    B::Error: std::error::Error + Send + Sync + 'static,
{
    loop {
        terminal
            .draw(|f| ui::draw(f, app))
            .map_err(|e| anyhow::anyhow!("Failed to draw terminal: {}", e))?;

        if events::handle_events(app).await? {
            break; // Quit signal
        }
    }

    Ok(())
}
