use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};

mod db;
mod models;

fn main() -> io::Result<()> {
    // ---- Database startup ----
    let conn = db::init_db().expect("Failed to init database");

    // Seed only once for testing
    db::seed_data(&conn).ok();

    let transactions = db::get_transactions(&conn).expect("Failed to load transactions");

    // ---- Terminal setup ----
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ---- Main loop ----
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let items: Vec<ListItem> = transactions
                .iter()
                .map(|tx| {
                    ListItem::new(format!(
                        "{} | {} | ₹{}",
                        tx.created_at, tx.description, tx.amount
                    ))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title("💰 Transactions")
                        .borders(Borders::ALL),
                )
                .highlight_style(Style::default().bold());

            f.render_widget(list, size);
        })?;

        // Input handling
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // ---- Cleanup ----
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
