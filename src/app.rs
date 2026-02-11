use rusqlite::Connection;

use crate::{
    config::load_config,
    db,
    form::TransactionForm,
    models::{Tag, Transaction},
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

    // ✅ Tags loaded dynamically from YAML
    pub tags: Vec<Tag>,

    // Cached transaction list
    pub transactions: Vec<Transaction>,

    // Selected transaction index
    pub selected: usize,
}

impl App {
    /// Create new app instance with config + DB load
    pub fn new(conn: &Connection) -> Self {
        // ----------------------------
        // ✅ Load YAML Config Tags
        // ----------------------------
        let config = load_config();

        // ✅ Convert Vec<String> → Vec<Tag>
        let tags: Vec<Tag> = config
            .tags
            .into_iter()
            .map(|s| Tag::from_str(&s))
            .collect();

        // ----------------------------
        // ✅ Load Transactions from DB
        // ----------------------------
        let transactions = db::get_transactions(conn).unwrap_or_default();

        Self {
            mode: Mode::Normal,
            form: TransactionForm::new(),

            tags,

            transactions,
            selected: 0,
        }
    }

    /// Refresh cached transactions from DB
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

        // ----------------------------
        // ✅ Get Selected Tag from Config
        // ----------------------------
        let tag = self
            .tags
            .get(self.form.tag_index)
            .unwrap_or(&Tag("other".into()))
            .clone();

        // ----------------------------
        // ✅ Insert Transaction into DB
        // ----------------------------
        db::add_transaction(
            conn,
            &self.form.source,
            amount,
            self.form.kind,
            &tag,
            &self.form.date,
        )
        .unwrap();

        // Refresh after saving
        self.refresh(conn);
    }

    /// Delete selected transaction
    pub fn delete_selected(&mut self, conn: &Connection) {
        if self.transactions.is_empty() {
            return;
        }

        let id = self.transactions[self.selected].id;

        db::delete_transaction(conn, id).unwrap();

        // Refresh after deletion
        self.refresh(conn);
    }
}
