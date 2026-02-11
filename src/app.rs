use rusqlite::Connection;

use crate::{
    db,
    form::TransactionForm,
    models::Transaction,
};

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Adding,
    Stats,
}

pub struct App {
    pub mode: Mode,
    pub form: TransactionForm,

    // ✅ Cached transaction list (no DB query every keypress)
    pub transactions: Vec<Transaction>,

    // Selected transaction index
    pub selected: usize,
}

impl App {
    pub fn new(conn: &Connection) -> Self {
        let transactions = db::get_transactions(conn).unwrap_or_default();

        Self {
            mode: Mode::Normal,
            form: TransactionForm::new(),
            transactions,
            selected: 0,
        }
    }

    /// Refresh transactions from DB
    pub fn refresh(&mut self, conn: &Connection) {
        self.transactions = db::get_transactions(conn).unwrap_or_default();

        // Clamp selection safely
        if self.selected >= self.transactions.len() && self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Save new transaction into DB
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

        // ✅ Immediately refresh after saving
        self.refresh(conn);
    }

    /// Delete selected transaction
    pub fn delete_selected(&mut self, conn: &Connection) {
        if self.transactions.is_empty() {
            return;
        }

        let id = self.transactions[self.selected].id;

        db::delete_transaction(conn, id).unwrap();

        // Refresh list after deletion
        self.refresh(conn);
    }
}
