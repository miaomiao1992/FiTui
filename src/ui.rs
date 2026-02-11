use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::models::Transaction;

pub fn draw_ui(
    f: &mut Frame,
    transactions: &Vec<Transaction>,
    earned: f64,
    spent: f64,
    balance: f64,
) {
    let size = f.size();

    // Layout split: Header + List
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(size);

    // ----------------------------
    // Header Stats
    // ----------------------------
    let header = Paragraph::new(format!(
        " Earned: ₹{:.2}   Spent: ₹{:.2}   Balance: ₹{:.2} ",
        earned, spent, balance
    ))
    .block(Block::default().title("📊 Stats").borders(Borders::ALL))
    .alignment(Alignment::Center);

    f.render_widget(header, chunks[0]);

    // ----------------------------
    // Transaction List
    // ----------------------------
    let items: Vec<ListItem> = transactions
        .iter()
        .map(|tx| {
            ListItem::new(format!(
                "{} | {} | ₹{:.2} | {} | {}",
                tx.date,
                tx.source,
                tx.amount,
                tx.kind.as_str(),
                tx.tag.as_str()
            ))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("💰 Transactions (a = add, q = quit)")
            .borders(Borders::ALL),
    );

    f.render_widget(list, chunks[1]);
}
