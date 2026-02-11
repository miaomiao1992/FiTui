mod app;
mod db;
mod form;
mod handlers;
mod models;
mod ui;

use std::io;

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::prelude::*;

use app::App;

fn main() -> io::Result<()> {
    let conn = db::init_db().unwrap();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        // Load DB data
        let txs = db::get_transactions(&conn).unwrap();
        let earned = db::total_earned(&conn).unwrap();
        let spent = db::total_spent(&conn).unwrap();
        let balance = earned - spent;

        // Draw UI
        terminal.draw(|f| {
            ui::draw_ui(f, &txs, earned, spent, balance, &app);
        })?;

        // Input handling
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                // ✅ Fix: Only respond to actual key presses
                if key.kind == KeyEventKind::Press {
                    let quit = handlers::handle_key(&mut app, key.code, &conn);

                    if quit {
                        break;
                    }
                }
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
