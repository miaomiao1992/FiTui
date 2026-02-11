use crate::models::{Tag, TransactionType};

#[derive(Clone, Copy)]
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
    pub tag: Tag,
    pub date: String,
    pub active: Field,
}

impl TransactionForm {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            amount: String::new(),
            kind: TransactionType::Debit,
            tag: Tag::Other,
            date: "2026-02-11".into(),
            active: Field::Source,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn push_char(&mut self, c: char) {
        match self.active {
            Field::Source => self.source.push(c),
            Field::Amount => self.amount.push(c),
            Field::Date => self.date.push(c),

            Field::Kind => {
                if c == 'c' {
                    self.kind = TransactionType::Credit;
                }
                if c == 'd' {
                    self.kind = TransactionType::Debit;
                }
            }

            Field::Tag => {
                self.tag = Tag::from_str(&c.to_string());
            }
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
}
