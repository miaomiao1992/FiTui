use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::prelude::*;

mod db;
mod models;
mod ui;

fn main() -> io::Result<()> {
    // ----------------------------
    // Database startup
    // ----------------------------
    let conn = db::init_db().expect("Failed to initialize database");

    // ----------------------------
    // Terminal setup
    // ----------------------------
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ----------------------------
    // Main loop
    // ----------------------------
    loop {
        // Load data
        let transactions = db::get_transactions(&conn).unwrap();

        let earned = db::total_earned(&conn).unwrap();
        let spent = db::total_spent(&conn).unwrap();
        let balance = earned - spent;

        // Draw UI
        terminal.draw(|f| {
            ui::draw_ui(f, &transactions, earned, spent, balance);
        })?;

        // Input handling
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Char('a') => {
                        // Add form comes next
                    }

                    _ => {}
                }
            }
        }
    }

    // ----------------------------
    // Cleanup
    // ----------------------------
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
