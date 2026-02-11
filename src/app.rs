use rusqlite::Connection;

use crate::{db, form::TransactionForm};

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Adding,
}

pub struct App {
    pub mode: Mode,
    pub form: TransactionForm,

    // ✅ NEW: selected transaction index
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            form: TransactionForm::new(),
            selected: 0,
        }
    }

    pub fn save_transaction(&mut self, conn: &Connection) {
        let amount: f64 = self.form.amount.trim().parse().unwrap_or(0.0);

        db::add_transaction(
            conn,
            &self.form.source,
            amount,
            self.form.kind,
            self.form.tag,
            &self.form.date,
        )
        .unwrap();
    }

    // ✅ Delete selected transaction
    pub fn delete_selected(&mut self, conn: &Connection, txs_len: usize) {
        if txs_len == 0 {
            return;
        }

        let id = db::get_transactions(conn)
            .unwrap()[self.selected]
            .id;

        db::delete_transaction(conn, id).unwrap();

        // Clamp selection
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}
