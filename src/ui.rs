use ratatui::{
    prelude::*,
    widgets::{Clear, List, ListItem, ListState, Paragraph},
};

use crate::{
    app::{App, Mode},
    form::Field,
    models::{Transaction, TransactionType},
    theme::Theme,
};

/* ============================================================================
 * MAIN DRAW ENTRY
 * ========================================================================== */

pub fn draw_ui(
    f: &mut Frame,
    transactions: &[Transaction],
    earned: f64,
    spent: f64,
    balance: f64,
    per_tag: &[(String, f64)],
    app: &App,
) {
    let theme = Theme::default();

    match app.mode {
        Mode::Stats => draw_stats_view(f, earned, spent, balance, per_tag, &theme),

        Mode::Adding => {
            draw_main_view(f, transactions, earned, spent, balance, app, &theme);
            draw_transaction_form(f, app, &theme);
        }

        _ => draw_main_view(f, transactions, earned, spent, balance, app, &theme),
    }
}

/* ============================================================================
 * MAIN VIEW (Header + List)
 * ========================================================================== */

fn draw_main_view(
    f: &mut Frame,
    transactions: &[Transaction],
    earned: f64,
    spent: f64,
    balance: f64,
    app: &App,
    theme: &Theme,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(f.size());

    draw_header(f, chunks[0], earned, spent, balance, theme);
    draw_transactions_list(f, chunks[1], transactions, app, theme);
}

/* ============================================================================
 * HEADER
 * ========================================================================== */

fn draw_header(
    f: &mut Frame,
    area: Rect,
    earned: f64,
    spent: f64,
    balance: f64,
    theme: &Theme,
) {
    let content = vec![
        Line::styled(
            "Personal Finance Dashboard",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::styled(
            format!(
                "Earned: ₹{:.2}   Spent: ₹{:.2}   Balance: ₹{:.2}",
                earned, spent, balance
            ),
            Style::default().fg(theme.muted),
        ),
    ];

    let header = Paragraph::new(content)
        .block(theme.block("Overview"))
        .alignment(Alignment::Center);

    f.render_widget(header, area);
}

/* ============================================================================
 * TRANSACTIONS LIST
 * ========================================================================== */

fn draw_transactions_list(
    f: &mut Frame,
    area: Rect,
    transactions: &[Transaction],
    app: &App,
    theme: &Theme,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),     // List
            Constraint::Length(1),  // Footer hint
        ])
        .split(area);

    let items = build_transaction_items(transactions, theme);
    let mut state = create_list_state(app.selected);

    let list = List::new(items)
        .block(theme.block("Transactions"))
        .highlight_style(theme.highlight_style())
        .highlight_symbol("  ❯ ");

    f.render_stateful_widget(list, layout[0], &mut state);

    let footer = Paragraph::new(Line::styled(
        "[↑↓] Navigate   [a] Add   [d] Delete   [s] Stats   [q] Quit",
        Style::default().fg(theme.muted),
    ))
    .alignment(Alignment::Center);

    f.render_widget(footer, layout[1]);
}

/* ============================================================================
 * LIST BUILDERS
 * ========================================================================== */

fn build_transaction_items(transactions: &[Transaction], theme: &Theme) -> Vec<ListItem<'static>> {
    let mut items = Vec::new();

    items.push(create_table_header(theme));
    items.push(create_divider(theme));

    for tx in transactions {
        items.push(create_transaction_row(tx, theme));
    }

    items
}

fn create_table_header(theme: &Theme) -> ListItem<'static> {
    ListItem::new(Line::from(vec![
        Span::styled("Date       ", Style::default().fg(theme.muted)),
        Span::styled("Source         ", Style::default().fg(theme.muted)),
        Span::styled("Amount     ", Style::default().fg(theme.muted)),
        Span::styled("Type      ", Style::default().fg(theme.muted)),
        Span::styled("Tag", Style::default().fg(theme.muted)),
    ]))
}

fn create_divider(theme: &Theme) -> ListItem<'static> {
    ListItem::new(Line::styled(
        "──────────────────────────────────────────────",
        Style::default().fg(theme.muted),
    ))
}

fn create_transaction_row(tx: &Transaction, theme: &Theme) -> ListItem<'static> {
    let color = theme.transaction_color(tx.kind);
    let kind_label = format_transaction_type(tx.kind);
    let tag_label = format!("<{}>", tx.tag.as_str());

    let line = Line::from(vec![
        Span::styled(format!("{:<10}", tx.date), Style::default().fg(theme.muted)),
        Span::raw("  "),
        Span::styled(format!("{:<14}", tx.source), Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled(
            format!("₹{:>8.2}", tx.amount),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(format!("{:<8}", kind_label), Style::default().fg(color)),
        Span::raw("  "),
        Span::styled(
            tag_label,
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::ITALIC),
        ),
    ]);

    ListItem::new(line)
}

fn create_list_state(selected: usize) -> ListState {
    let mut state = ListState::default();
    state.select(Some(selected + 2));
    state
}

/* ============================================================================
 * STATS VIEW
 * ========================================================================== */

fn draw_stats_view(
    f: &mut Frame,
    earned: f64,
    spent: f64,
    balance: f64,
    per_tag: &[(String, f64)],
    theme: &Theme,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    let content = build_stats_content(earned, spent, balance, per_tag, theme);

    let stats_widget = Paragraph::new(content)
        .block(theme.block("Statistics"))
        .alignment(Alignment::Left);

    f.render_widget(stats_widget, layout[0]);

    let footer = Paragraph::new(Line::styled(
        "[q] Back   |   Stats Mode",
        Style::default().fg(theme.muted),
    ))
    .alignment(Alignment::Center);

    f.render_widget(footer, layout[1]);
}

fn build_stats_content(
    earned: f64,
    spent: f64,
    balance: f64,
    per_tag: &[(String, f64)],
    theme: &Theme,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(Line::raw(""));
    lines.extend(create_overview_section(earned, spent, balance));

    lines.push(Line::raw("────────────────────────────────────────"));
    lines.push(Line::raw(""));
    lines.push(Line::styled(
        "Spending Breakdown by Tag",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::raw(""));

    lines.extend(create_tag_breakdown_section(per_tag, theme));

    lines
}

fn create_overview_section(earned: f64, spent: f64, balance: f64) -> Vec<Line<'static>> {
    vec![
        Line::styled("Overview", Style::default().add_modifier(Modifier::BOLD)),
        Line::raw(format!("  Earned   : ₹{:.2}", earned)),
        Line::raw(format!("  Spent    : ₹{:.2}", spent)),
        Line::raw(format!("  Balance  : ₹{:.2}", balance)),
        Line::raw(""),
    ]
}

fn create_tag_breakdown_section(
    per_tag: &[(String, f64)],
    theme: &Theme,
) -> Vec<Line<'static>> {
    let max_spent = per_tag
        .iter()
        .map(|(_, amount)| *amount)
        .fold(0.0, f64::max);

    per_tag
        .iter()
        .map(|(tag, total)| create_tag_bar(tag, *total, max_spent, theme))
        .collect()
}

fn create_tag_bar(tag: &str, amount: f64, max_amount: f64, theme: &Theme) -> Line<'static> {
    let bar_width = calculate_bar_width(amount, max_amount);
    let bar = "█".repeat(bar_width);

    Line::from(vec![
        Span::styled(
            format!("<{:<10}>", tag),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::ITALIC),
        ),
        Span::raw("  "),
        Span::styled(bar, Style::default().fg(theme.debit)),
        Span::raw(" "),
        Span::styled(
            format!("₹{:.2}", amount),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])
}

fn calculate_bar_width(amount: f64, max_amount: f64) -> usize {
    if max_amount > 0.0 {
        ((amount / max_amount) * 12.0).round() as usize
    } else {
        0
    }
}

/* ============================================================================
 * TRANSACTION FORM POPUP
 * ========================================================================== */

fn draw_transaction_form(f: &mut Frame, app: &App, theme: &Theme) {
    let area = centered_rect(70, 55, f.size());
    let form_content = build_form_content(app, theme);

    let popup = Paragraph::new(form_content)
        .block(theme.popup("Transaction Form"))
        .alignment(Alignment::Left);

    f.render_widget(Clear, area);
    f.render_widget(popup, area);
}

fn build_form_content(app: &App, theme: &Theme) -> Vec<Line<'static>> {
    let form = &app.form;

    vec![
        Line::styled("Add Transaction", theme.title()),
        Line::raw(""),

        create_form_field("Source", &form.source, form.active, Field::Source, theme),
        create_form_field("Amount", &form.amount, form.active, Field::Amount, theme),

        Line::raw(""),

        create_type_selector(&form.kind, theme),
        create_tag_selector(&form.tag, theme),

        Line::raw(""),

        create_form_field("Date", &form.date, form.active, Field::Date, theme),

        Line::raw(""),
        Line::styled("────────────────────────────────────", theme.muted_text()),

        Line::styled(
            "[Tab] Next Field   [←→] Change Type/Tag   [Enter] Save   [Esc] Cancel",
            theme.muted_text(),
        ),
    ]
}


fn create_form_field(
   label: &str,
    value: &str,
    active_field: Field,
    field: Field,
    theme: &Theme,
) -> Line<'static> {
    let is_active = active_field == field;

    // Label style: highlighted when active
    let label_style = if is_active {
        theme.title()
    } else {
        theme.muted_text()
    };

    // Value style: cursor block when active
    let value_style = if is_active {
        theme.cursor_style()
    } else {
        Style::default().fg(theme.foreground)
    };

    Line::from(vec![
        Span::styled(format!("{:<7}: ", label), label_style),
        Span::styled(value.to_string(), value_style),
    ])
}


fn create_type_selector(kind: &TransactionType, theme: &Theme) -> Line<'static> {
    let kind_style = match kind {
        TransactionType::Credit => theme.success(),
        TransactionType::Debit => theme.danger(),
    };

    Line::from(vec![
        Span::styled("Type   : ", theme.muted_text()),
        Span::styled(format!("<{:?}>", kind), kind_style),
        Span::styled("   ← →", theme.muted_text()),
    ])
}

fn create_tag_selector(tag: &crate::models::Tag, theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled("Tag    : ", theme.muted_text()),
        Span::styled(
            format!("<{:?}>", tag),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::ITALIC),
        ),
        Span::styled("   ← →", theme.muted_text()),
    ])
}


/* ============================================================================
 * UTILITIES
 * ========================================================================== */

fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(rect);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical_layout[1])[1]
}

fn format_transaction_type(tx_type: TransactionType) -> &'static str {
    match tx_type {
        TransactionType::Credit => "<CREDIT>",
        TransactionType::Debit => "<DEBIT>",
    }
}
