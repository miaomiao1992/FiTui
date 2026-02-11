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
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            form: TransactionForm::new(),
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
}
