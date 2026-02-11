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
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use ratatui::prelude::*;

use app::App;

fn main() -> io::Result<()> {
    // ----------------------------
    // ✅ Init Database
    // ----------------------------
    let conn = db::init_db().unwrap();

    // ----------------------------
    // ✅ Terminal Setup
    // ----------------------------
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ----------------------------
    // ✅ App State Init (NEW)
    // ----------------------------
    let mut app = App::new(&conn);

    // ----------------------------
    // Main Loop
    // ----------------------------
    loop {
        // ----------------------------
        // ✅ Stats Queries (every frame)
        // ----------------------------
        let earned = db::total_earned(&conn).unwrap();
        let spent = db::total_spent(&conn).unwrap();
        let balance = earned - spent;

        // ✅ Per-tag breakdown for Stats page
        let per_tag = db::spent_per_tag(&conn).unwrap();

        // ----------------------------
        // Draw UI
        // ----------------------------
        terminal.draw(|f| {
            ui::draw_ui(
                f,
                &app.transactions, // ✅ Use cached transactions
                earned,
                spent,
                balance,
                &per_tag,          // ✅ NEW
                &app,
            );
        })?;

        // ----------------------------
        // Input Handling
        // ----------------------------
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let quit = handlers::handle_key(&mut app, key.code, &conn);

                    // ✅ If user quits, break loop
                    if quit {
                        break;
                    }

                    // ✅ Refresh transactions after actions
                    // (Add/Delete updates instantly)
                    app.refresh(&conn);
                }
            }
        }
    }

    // ----------------------------
    // Cleanup Terminal
    // ----------------------------
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
