use rusqlite::{Connection, Result};

use crate::models::Transaction;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("budget.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            amount REAL NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn get_transactions(conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, description, amount, created_at
         FROM transactions
         ORDER BY id DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            description: row.get(1)?,
            amount: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    let mut transactions = Vec::new();
    for tx in rows {
        transactions.push(tx?);
    }

    Ok(transactions)
}

pub fn seed_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO transactions (description, amount, created_at)
         VALUES (?1, ?2, datetime('now'))",
        ("Coffee", -120.0),
    )?;

    conn.execute(
        "INSERT INTO transactions (description, amount, created_at)
         VALUES (?1, ?2, datetime('now'))",
        ("Salary", 50000.0),
    )?;

    Ok(())
}
