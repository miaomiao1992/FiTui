use crossterm::event::KeyCode;
use rusqlite::Connection;

use crate::app::{App, Mode};

pub fn handle_key(app: &mut App, key: KeyCode, conn: &Connection) -> bool {
    match app.mode {
        Mode::Normal => handle_normal(app, key),
        Mode::Adding => handle_form(app, key, conn),
    }
}

fn handle_normal(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('a') => {
            app.mode = Mode::Adding;
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
