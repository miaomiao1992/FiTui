use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::{
    app::{App, Mode},
    models::{Transaction, TransactionType},
};

pub fn draw_ui(
    f: &mut Frame,
    transactions: &[Transaction],
    earned: f64,
    spent: f64,
    balance: f64,
    app: &App,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(f.size());

    draw_header(f, chunks[0], earned, spent, balance);
    draw_transactions(f, chunks[1], transactions, app);


    if app.mode == Mode::Adding {
        draw_popup(f, app);
    }
}

fn draw_header(f: &mut Frame, area: Rect, earned: f64, spent: f64, balance: f64) {
    let header = Paragraph::new(format!(
        "Earned: ₹{:.2}   Spent: ₹{:.2}   Balance: ₹{:.2}",
        earned, spent, balance
    ))
    .block(Block::default().title("📊 Overview").borders(Borders::ALL))
    .alignment(Alignment::Center);

    f.render_widget(header, area);
}

fn draw_transactions(f: &mut Frame, area: Rect, transactions: &[Transaction], app: &App) {
    let items: Vec<ListItem> = transactions
        .iter()
        .map(|tx| {
            let style = match tx.kind {
                TransactionType::Credit => Style::default().fg(Color::Green),
                TransactionType::Debit => Style::default().fg(Color::Red),
            };

            ListItem::new(Line::styled(
                format!(
                    "{:<12} {:<12} ₹{:>8.2} [{}]",
                    tx.date,
                    tx.source,
                    tx.amount,
                    tx.tag.as_str()
                ),
                style,
            ))
        })
        .collect();

    // ✅ Selection state
    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().title("💰 Transactions (↑↓ select, d delete)").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .bold(),
        )
        .highlight_symbol("👉 ");

    f.render_stateful_widget(list, area, &mut state);
}


fn draw_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, f.size());

    let form = &app.form;

    let lines = vec![
        Line::raw(format!("Source: {}", form.source)),
        Line::raw(format!("Amount: {}", form.amount)),
        Line::raw(format!("Kind: {:?}", form.kind)),
        Line::raw(format!("Tag: {:?}", form.tag)),
        Line::raw(format!("Date: {}", form.date)),
        Line::raw(""),
        Line::styled("[Tab] Next  [Enter] Save  [Esc] Cancel", Style::default()),
    ];

    let popup = Paragraph::new(lines)
        .block(Block::default().title("➕ Add Transaction").borders(Borders::ALL));

    f.render_widget(Clear, area);
    f.render_widget(popup, area);
}

fn centered_rect(px: u16, py: u16, r: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(vert[1])[1]
}
