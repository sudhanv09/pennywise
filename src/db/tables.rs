use rusqlite::{Connection, Result};

pub fn create_all(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS accounts (
            id               INTEGER PRIMARY KEY AUTOINCREMENT,
            name             TEXT    NOT NULL,
            starting_balance REAL    NOT NULL,
            icon             TEXT    NOT NULL,
            currency         TEXT    NOT NULL
        );

        CREATE TABLE IF NOT EXISTS categories (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            icon TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS goals (
            id       INTEGER PRIMARY KEY AUTOINCREMENT,
            name     TEXT NOT NULL,
            target   REAL NOT NULL,
            current  REAL NOT NULL,
            deadline REAL
        );

        CREATE TABLE IF NOT EXISTS loans (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            name         TEXT    NOT NULL,
            total_amount REAL    NOT NULL,
            paid_amount  REAL    NOT NULL,
            due          REAL    NOT NULL,
            is_lender    INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS subscriptions (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            name          TEXT NOT NULL,
            billing_cycle TEXT NOT NULL,
            next_billing  REAL NOT NULL
        );

        CREATE TABLE IF NOT EXISTS transactions (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT    NOT NULL,
            amount      REAL    NOT NULL,
            tx_date     TEXT    NOT NULL,
            tx_time     TEXT    NOT NULL,
            tx_type     TEXT    NOT NULL,
            category    INTEGER NOT NULL,
            account     INTEGER NOT NULL,
            description TEXT    NOT NULL,
            goal_id         INTEGER,
            loan_id         INTEGER,
            frequency       TEXT,
            recurring_till  TEXT
        );
    ")?;

    Ok(())
}
