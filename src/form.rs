use crate::models::TransactionType;

#[derive(PartialEq, Copy, Clone)]
pub enum Field {
    Source,
    Amount,
    Kind,
    Tag,
    Date,
}

impl Field {
    pub fn next(self) -> Self {
        use Field::*;
        match self {
            Source => Amount,
            Amount => Kind,
            Kind => Tag,
            Tag => Date,
            Date => Source,
        }
    }
}

pub struct TransactionForm {
    pub source: String,
    pub amount: String,
    pub kind: TransactionType,

    // ✅ Tag is now dynamic (index into config.tags)
    pub tag_index: usize,

    pub date: String,
    pub active: Field,
}

impl TransactionForm {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            amount: String::new(),
            kind: TransactionType::Debit,

            // Default to first tag in config
            tag_index: 0,

            date: "2026-02-11".into(),
            active: Field::Source,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    // ============================
    // Typing Support (Text Fields Only)
    // ============================

    pub fn push_char(&mut self, c: char) {
        match self.active {
            Field::Source => self.source.push(c),
            Field::Amount => self.amount.push(c),
            Field::Date => self.date.push(c),

            // Kind + Tag no typing
            _ => {}
        }
    }

    pub fn pop_char(&mut self) {
        match self.active {
            Field::Source => {
                self.source.pop();
            }
            Field::Amount => {
                self.amount.pop();
            }
            Field::Date => {
                self.date.pop();
            }
            _ => {}
        }
    }

    // ============================
    // Toggle Credit/Debit
    // ============================

    pub fn toggle_kind(&mut self) {
        self.kind = match self.kind {
            TransactionType::Credit => TransactionType::Debit,
            TransactionType::Debit => TransactionType::Credit,
        };
    }

    // ============================
    // Dynamic Tag Cycling
    // ============================

    // 👉 Right Arrow (Next Tag)
    pub fn next_tag(&mut self, total_tags: usize) {
        if total_tags == 0 {
            return;
        }

        self.tag_index = (self.tag_index + 1) % total_tags;
    }

    // 👈 Left Arrow (Previous Tag)
    pub fn prev_tag(&mut self, total_tags: usize) {
        if total_tags == 0 {
            return;
        }

        if self.tag_index == 0 {
            self.tag_index = total_tags - 1;
        } else {
            self.tag_index -= 1;
        }
    }
}
