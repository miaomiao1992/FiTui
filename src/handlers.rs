use crossterm::event::KeyCode;
use rusqlite::Connection;

use crate::app::{App, Mode};

pub fn handle_key(app: &mut App, key: KeyCode, conn: &Connection) -> bool {
    match app.mode {
        Mode::Normal => handle_normal(app, key, conn),
        Mode::Adding => handle_form(app, key, conn),
    }
}

fn handle_normal(app: &mut App, key: KeyCode, conn: &Connection) -> bool {
    let txs = crate::db::get_transactions(conn).unwrap();
    let len = txs.len();

    match key {
        KeyCode::Char('q') => return true,

        KeyCode::Char('a') => {
            app.mode = Mode::Adding;
        }

        // ✅ Move selection up
        KeyCode::Up => {
            if app.selected > 0 {
                app.selected -= 1;
            }
        }

        // ✅ Move selection down
        KeyCode::Down => {
            if app.selected + 1 < len {
                app.selected += 1;
            }
        }

        // ✅ Delete selected transaction
        KeyCode::Char('d') => {
            app.delete_selected(conn, len);
        }

        _ => {}
    }

    false
}


fn handle_form(app: &mut App, key: KeyCode, conn: &Connection) -> bool {
    match key {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
        }

        KeyCode::Tab => {
            app.form.active = app.form.active.next();
        }

        KeyCode::Backspace => {
            app.form.pop_char();
        }

        KeyCode::Char(c) => {
            app.form.push_char(c);
        }

        KeyCode::Enter => {
            app.save_transaction(conn);
            app.form.reset();
            app.mode = Mode::Normal;
        }

        _ => {}
    }

    false
}
