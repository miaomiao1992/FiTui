use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::models::TransactionType;

/* ============================================================================
 * THEME CONFIGURATION
 * ========================================================================== */

#[derive(Clone, Copy)]
pub struct Theme {
    pub accent: Color,
    pub credit: Color,
    pub debit: Color,
    pub muted: Color,
    pub background: Color,
    pub foreground: Color,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            accent: Color::Rgb(100, 181, 246),
            credit: Color::Rgb(102, 187, 106),
            debit: Color::Rgb(239, 83, 80),
            muted: Color::Rgb(158, 158, 158),
            background: Color::Rgb(30, 30, 46),
            foreground: Color::Rgb(205, 214, 244),
        }
    }

    pub fn transaction_color(&self, tx_type: TransactionType) -> Color {
        match tx_type {
            TransactionType::Credit => self.credit,
            TransactionType::Debit => self.debit,
        }
    }

    pub fn highlight_style(&self) -> Style {
        Style::default()
            .bg(Color::Rgb(49, 50, 68))
            .fg(self.foreground)
            .add_modifier(Modifier::BOLD)
    }

    pub fn block<'a>(&self, title: &'a str) -> Block<'a> {
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::ROUNDED)
            .border_style(Style::default().fg(self.accent))
    }
}
