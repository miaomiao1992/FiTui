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
    /* Palette */
    pub accent: Color,
    pub accent_soft: Color,

    pub credit: Color,
    pub debit: Color,

    pub muted: Color,
    pub subtle: Color,

    pub background: Color,
    pub surface: Color,

    pub foreground: Color,
}

impl Theme {
    /* ============================================================================
     * DEFAULT DARK THEME
     * ========================================================================== */

    pub fn default() -> Self {
        Self {
            accent: Color::Rgb(100, 181, 246),
            accent_soft: Color::Rgb(80, 140, 200),

            credit: Color::Rgb(102, 187, 106),
            debit: Color::Rgb(239, 83, 80),

            muted: Color::Rgb(160, 160, 170),
            subtle: Color::Rgb(90, 90, 110),

            background: Color::Rgb(24, 24, 36),
            surface: Color::Rgb(34, 34, 52),

            foreground: Color::Rgb(220, 225, 245),
        }
    }

    /* ============================================================================
     * SEMANTIC COLOR HELPERS
     * ========================================================================== */

    pub fn transaction_color(&self, tx_type: TransactionType) -> Color {
        match tx_type {
            TransactionType::Credit => self.credit,
            TransactionType::Debit => self.debit,
        }
    }

    pub fn danger(&self) -> Style {
        Style::default()
            .fg(self.debit)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success(&self) -> Style {
        Style::default()
            .fg(self.credit)
            .add_modifier(Modifier::BOLD)
    }

    pub fn muted_text(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn title(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /* ============================================================================
     * HIGHLIGHT + SELECTION
     * ========================================================================== */

    pub fn highlight_style(&self) -> Style {
        Style::default()
            .bg(self.surface)
            .fg(self.foreground)
            .add_modifier(Modifier::BOLD)
    }

    pub fn cursor_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::REVERSED)
    }

    /* ============================================================================
     * BLOCKS + PANELS
     * ========================================================================== */

    pub fn block<'a>(&self, title: &'a str) -> Block<'a> {
        Block::default()
            .title(Span::styled(title, self.title()))
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::ROUNDED)
            .border_style(Style::default().fg(self.accent_soft))
    }

    pub fn panel<'a>(&self) -> Block<'a> {
        Block::default()
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::ROUNDED)
            .border_style(Style::default().fg(self.subtle))
            .style(Style::default().bg(self.background))
    }

    pub fn popup<'a>(&self, title: &'a str) -> Block<'a> {
        Block::default()
            .title(Span::styled(title, self.title()))
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::DOUBLE)
            .border_style(Style::default().fg(self.accent))
            .style(Style::default().bg(self.surface))
    }
}
