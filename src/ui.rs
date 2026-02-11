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
    per_tag: &Vec<(String, f64)>, // ✅ NEW
    app: &App,
) {
    // ✅ If Stats mode, draw stats page only
    if app.mode == Mode::Stats {
        draw_stats_page(f, earned, spent, balance, per_tag);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(f.size());

    draw_header(f, chunks[0], earned, spent, balance);
    draw_transactions(f, chunks[1], transactions, app);

    if app.mode == Mode::Adding {
        draw_popup(f, app);
    }
}

/* ---------------- THEME ---------------- */

fn theme() -> (Color, Color, Color, Color) {
    let accent = Color::Cyan;
    let credit = Color::LightGreen;
    let debit = Color::LightRed;
    let muted = Color::Gray;

    (accent, credit, debit, muted)
}

/* ---------------- HEADER ---------------- */

fn draw_header(f: &mut Frame, area: Rect, earned: f64, spent: f64, balance: f64) {
    let (accent, _, _, muted) = theme();

    let text = vec![
        Line::styled(
            "Personal Finance Dashboard",
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::styled(
            format!(
                "Earned: ₹{:.2}   Spent: ₹{:.2}   Balance: ₹{:.2}",
                earned, spent, balance
            ),
            Style::default().fg(muted),
        ),
    ];

    let header = Paragraph::new(text)
        .block(
            Block::default()
                .title("Overview")
                .borders(Borders::ALL)
                .border_set(ratatui::symbols::border::ROUNDED)
                .border_style(Style::default().fg(accent)),
        )
        .alignment(Alignment::Center);

    f.render_widget(header, area);
}

/* ---------------- TRANSACTIONS ---------------- */

fn draw_transactions(f: &mut Frame, area: Rect, transactions: &[Transaction], app: &App) {
    let (accent, credit, debit, muted) = theme();

    let items: Vec<ListItem> = transactions
        .iter()
        .map(|tx| {
            let color = match tx.kind {
                TransactionType::Credit => credit,
                TransactionType::Debit => debit,
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("{:<10}", tx.date),
                    Style::default().fg(muted),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("{:<14}", tx.source),
                    Style::default().fg(Color::White),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("₹{:>8.2}", tx.amount),
                    Style::default()
                        .fg(color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("   "),
                Span::styled(
                    format!("{}", tx.tag.as_str()),
                    Style::default().fg(muted),
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(
            Block::default()
                .title("Transactions")
                .borders(Borders::ALL)
                .border_set(ratatui::symbols::border::ROUNDED)
                .border_style(Style::default().fg(accent)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("  > ");

    f.render_stateful_widget(list, area, &mut state);
}

/* ---------------- 📊 STATS PAGE ---------------- */

fn draw_stats_page(
    f: &mut Frame,
    earned: f64,
    spent: f64,
    balance: f64,
    per_tag: &Vec<(String, f64)>,
) {
    let (accent, _, _, muted) = theme();

    let mut lines = vec![
        Line::styled(
            "📊 Stats Overview",
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::raw(format!("Total Earned : ₹{:.2}", earned)),
        Line::raw(format!("Total Spent  : ₹{:.2}", spent)),
        Line::raw(format!("Balance      : ₹{:.2}", balance)),
        Line::raw(""),
        Line::styled(
            "Spending Breakdown:",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
    ];

    for (tag, total) in per_tag {
        lines.push(Line::raw(format!(
            "{:<12} → ₹{:.2}",
            tag,
            total
        )));
    }

    lines.push(Line::raw(""));
    lines.push(Line::styled(
        "[Esc] Back   |   Stats Mode",
        Style::default().fg(muted),
    ));

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title("📈 Statistics")
                .borders(Borders::ALL)
                .border_set(ratatui::symbols::border::ROUNDED)
                .border_style(Style::default().fg(accent)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, f.size());
}

/* ---------------- POPUP FORM ---------------- */

fn draw_popup(f: &mut Frame, app: &App) {
    let (accent, _, _, muted) = theme();
    let area = centered_rect(65, 45, f.size());

    let form = &app.form;

    let lines = vec![
        Line::styled(
            "Add Transaction",
            Style::default()
                .fg(accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::raw(format!("Source : {}", form.source)),
        Line::raw(format!("Amount : {}", form.amount)),
        Line::raw(format!("Kind   : {:?}", form.kind)),
        Line::raw(format!("Tag    : {:?}", form.tag)),
        Line::raw(format!("Date   : {}", form.date)),
        Line::raw(""),
        Line::styled(
            "[Tab] Next   [Enter] Save   [Esc] Cancel",
            Style::default().fg(muted),
        ),
    ];

    let popup = Paragraph::new(lines)
        .block(
            Block::default()
                .title("Form")
                .borders(Borders::ALL)
                .border_set(ratatui::symbols::border::ROUNDED)
                .border_style(Style::default().fg(accent)),
        )
        .alignment(Alignment::Left);

    f.render_widget(Clear, area);
    f.render_widget(popup, area);
}

/* ---------------- CENTER RECT ---------------- */

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
